# APS CP Algorithm Interface & Documentation

本目录包含基于约束规划（Constraint Programming, CP）思想的高级排产系统组件。它允许你通过定义工序和机器资源的拓扑依赖，计算出最佳的排产时间表（Gantt Chart）。

## 算法逻辑说明

这是一个简化的基于时间线向前传播（Forward Propagation）的排程生成器。
其核心调度原则如下：
1. **依赖约束**：对于某个 Job 下的每一个 Operation（工序），它必须在其前置所有依赖项（Dependencies）完成后才能开始加工。
2. **机器资源约束**：指定的 `machine_id` 一次只能处理一道工序。设备加工完成前，其他也需要该设备的工序必须排队。
3. 算法通过一个贪心拓扑循环，不断扫描当前已满足前置条件的工序。
4. 针对挑选出的合格工序，其最早可以开工的时间被确定为：`max(其所有依赖工序的最晚完成时间, 所需机器资源的最早空闲时间)`。
5. 最终该组件会返回 `makespan`（所有订单加工完毕的最短总时间）及详细的时间排布。

## 与 Go 对接：Protobuf 说明

本仓库提供标准的 Protobuf 定义 `proto/aps_cp.proto` 供 Go 等微服务使用（详情查看该文件）。

## JSON API 接口规范

- **Endpoint**: `POST /api/v1/aps/cp/schedule`
- **Content-Type**: `application/json`

### 输入示例 (ScheduleRequest)

```json
{
  "jobs": [
    {
      "job_id": "Job_1",
      "operations": [
        {
          "op_id": "Op_1_1",
          "duration": 5,
          "machine_id": "Machine_A",
          "dependencies": []
        },
        {
          "op_id": "Op_1_2",
          "duration": 10,
          "machine_id": "Machine_B",
          "dependencies": ["Op_1_1"]
        }
      ]
    },
    {
      "job_id": "Job_2",
      "operations": [
        {
          "op_id": "Op_2_1",
          "duration": 7,
          "machine_id": "Machine_A",
          "dependencies": []
        }
      ]
    }
  ]
}
```

### 输出示例 (ScheduleResponse)

```json
{
  "makespan": 15,
  "schedule": [
    {
      "job_id": "Job_1",
      "op_id": "Op_1_1",
      "machine_id": "Machine_A",
      "start_time": 0,
      "end_time": 5
    },
    {
      "job_id": "Job_2",
      "op_id": "Op_2_1",
      "machine_id": "Machine_A",
      "start_time": 5,
      "end_time": 12
    },
    {
      "job_id": "Job_1",
      "op_id": "Op_1_2",
      "machine_id": "Machine_B",
      "start_time": 5,
      "end_time": 15
    }
  ]
}
```
可以看到，`Job_2` 的第一道工序由于 `Machine_A` 被 `Job_1` 的第一道工序占用，被迫延后到第 `5` 秒钟开始执行。
