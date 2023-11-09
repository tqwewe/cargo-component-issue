# Issue with Cargo Component: Inline WIT Fails

This repository demonstrates a problem with `cargo component`.
An inline WebAssembly Interface Types (WIT) definition fails to build with `cargo component build`,
yet it builds successfully when using `cargo` and `wasm-tools` directly.

## Steps to Reproduce the Issue

**Manual Build with Cargo and wasm-tools**

1. Build the project using `cargo` for the `wasm32-wasi` target:
    ```bash
    cargo build --package guest --target wasm32-wasi
    ```
2. Create the component with `wasm-tools`, specifying the adapted WIT:
    ```bash
    wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o ./target/wasm32-wasi/debug/guest.wasm --adapt ./wasi_snapshot_preview1.wasm
    ```
    The resulting wasm file is located at `./target/wasm32-wasi/debug/guest.wasm`.

3. Execute the host package to observe the output:
    ```bash
    cargo run --package host
    ```
    You should see the output: `Hello, World!`

**Build with Cargo Component**

1. Clean the previous build artifacts:
    ```bash
    cargo clean
    ```
2. Attempt to build the component using `cargo component`:
    ```bash
    cargo component build --package guest
    ```
3. Running the host package now:
    ```bash
    cargo run --package host
    ```
    Results in a failure.

---

Ideally, both build processes should yield the same successful result.
Any insights on this discrepancy would be appreciated.
