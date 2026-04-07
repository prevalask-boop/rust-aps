use serde::{Deserialize, Serialize};

// --------------------------------------------------------
// Data Structures (Mapped from Proto conceptually)
// --------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Operation {
    pub op_id: String,
    pub duration: u32,
    pub machine_id: String,
    pub dependencies: Vec<String>, // op_ids that must finish before this one
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job {
    pub job_id: String,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleRequest {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScheduledOperation {
    pub job_id: String,
    pub op_id: String,
    pub machine_id: String,
    pub start_time: u32,
    pub end_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleResponse {
    pub makespan: u32,
    pub schedule: Vec<ScheduledOperation>,
}

// --------------------------------------------------------
// Simplified Constraint Programming Algorithm Engine
// --------------------------------------------------------
// In a real industrial CP engine (like OR-Tools CP-SAT), this is solved via domain reduction and branching.
// Here, we implement a simplified timeline-based forward constraint propagation logic (Active Schedule Builder).

pub fn solve_cp(request: &ScheduleRequest) -> ScheduleResponse {
    let mut schedule_result: Vec<ScheduledOperation> = Vec::new();

    // Track the current available time for each machine
    let mut machine_avail_time: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    // Track the completion time of each operation globally to satisfy dependencies
    let mut op_completion_time: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    // Flatten all operations to schedule
    // We will greedily schedule them as soon as their dependencies are met (topological order)
    let mut pending_ops: Vec<(String, Operation)> = Vec::new();
    for job in &request.jobs {
        for op in &job.operations {
            pending_ops.push((job.job_id.clone(), op.clone()));
        }
    }

    let mut makespan = 0;

    // Very naive solver loop: find an operation whose dependencies are satisfied
    while !pending_ops.is_empty() {
        let mut scheduled_in_this_pass = false;

        for i in 0..pending_ops.len() {
            let (job_id, op) = &pending_ops[i];

            // Check if all dependencies are satisfied
            let deps_satisfied = op.dependencies.iter().all(|dep_id| op_completion_time.contains_key(dep_id));

            if deps_satisfied {
                // Calculate earliest start time based on dependencies
                let earliest_start_from_deps = op.dependencies.iter()
                    .map(|dep_id| *op_completion_time.get(dep_id).unwrap_or(&0))
                    .max()
                    .unwrap_or(0);

                // Calculate earliest start time based on machine availability
                let machine_ready_time = *machine_avail_time.get(&op.machine_id).unwrap_or(&0);

                let start_time = std::cmp::max(earliest_start_from_deps, machine_ready_time);
                let end_time = start_time + op.duration;

                // Update constraints state
                machine_avail_time.insert(op.machine_id.clone(), end_time);
                op_completion_time.insert(op.op_id.clone(), end_time);

                makespan = std::cmp::max(makespan, end_time);

                schedule_result.push(ScheduledOperation {
                    job_id: job_id.clone(),
                    op_id: op.op_id.clone(),
                    machine_id: op.machine_id.clone(),
                    start_time,
                    end_time,
                });

                pending_ops.remove(i);
                scheduled_in_this_pass = true;
                break; // Restart the pass to maintain consistent state evaluation
            }
        }

        // Cycle detection / Unsolvable state
        if !scheduled_in_this_pass && !pending_ops.is_empty() {
            // In a real CP solver, this would trigger backtracking. Here we just abort.
            break;
        }
    }

    ScheduleResponse {
        makespan,
        schedule: schedule_result,
    }
}
