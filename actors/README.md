# Actors for wasmCloud

This directory contains all **wasmCloud actors** used in the workshop.

## Actors Included:
1. **hello-world/** - A basic "Hello World" actor to introduce wasmCloud.
2. **couchbase-actor/** - An actor that connects to Couchbase Capella.

## Usage:
- Each actor has its own `Cargo.toml` and `wasmcloud.toml`.
- To build an actor:
  ```bash
  cargo build --release --target wasm32-unknown-unknown
  ```
- To deploy an actor:
  ```bash
  wash app deploy actors/<actor-name>.wasm
  ```

## Debugging
- If the actor does not start, check if wasmCloud is running.
- Verify the **NATS connection** with:
    ```bash
    wash get hosts
    ```