# Actors for wasmCloud

This directory contains all **wasmCloud actors** used in the workshop.

## Actors Included:
1. **hello-world/** - A basic "Hello World" actor to introduce wasmCloud.
2. **couchbase-actor/** - An actor that connects to Couchbase Capella.

## Usage:
- Each actor has its own `Cargo.toml`, `wadm.yaml`, and `wasmcloud.toml`.
- To build an actor from inside its directory:
  ```bash
  wash build
  ```
- To deploy an actor from inside its directory:
  ```bash
  wash app deploy wadm.yaml
  ```
- To delete an actor:
  ```bash
  wash app delete <name-of-component>
  ```
- To list all actors:
  ```bash
  wash app list
  ```

## Debugging
- If the actor does not start, check if wasmCloud is running.
- Verify the **NATS connection** with:
    ```bash
    wash get hosts
    ```
- Make sure the Rust toolchain is installed and the correct wasm target is added:
    ```bash
    rustup target add wasm32-wasip2
    ```