FROM scratch
COPY spin.toml /spin.toml
COPY crates/algo-example/target/wasm32-wasip1/release/algo_example.wasm /crates/algo-example/target/wasm32-wasip1/release/algo_example.wasm
