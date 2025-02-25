# NATS Messaging Provider for wasmCloud

This directory contains the **NATS messaging provider** for wasmCloud.

## Files

* `Dockerfile` - Defines the container for the NATS provider.
* `wasmcloud.toml` - Configuration file for the provider.
* `start.sh` - Starts the provider in wasmCloud.
* `stop.sh` - Stops the provider in wasmCloud.

## Usage

1. Set the NATS connection URL:
   ```bash
   export CENTRAL_WASMCLOUD_URL=nats://your-deployed-wasmcloud.fly.dev:4222
   ```
2. Start the provider:
  ```bash
  ./start.sh
  ```
3. Verify provider status:
  ```bash
  wash get providers
  ```
4. Stop the provider:
  ```bash
    ./stop.sh
    ```

## Debugging
- Ensure the NATS URL is correct.
- Verify that the wasmCloud central node is running.