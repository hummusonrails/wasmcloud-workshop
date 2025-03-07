# Edge Node (GitHub CodeSpace Node)

This directory contains everything needed to run a **wasmCloud edge node** inside a GitHub Codespace.

## Files

* `connect-to-cloud.sh` - Connects the edge node to the central wasmCloud.
* `deploy-actor.sh` - Deploys an actor in the local wasmCloud instance.
* `wasmcloud.toml` - Configures the edge node's connection.

## Usage
1. Set environment variables:
  ```bash
  export CENTRAL_WASMCLOUD_URL=nats://ip-address-provided-by-instructor:4222
  export WASMCLOUD_CTL_HOST=ip-address-provided-by-instructor
  ```
2. Connect to the central wasmCloud instance:
  ```bash
  ./connect-to-cloud.sh
  ```
3. Deploy an actor:
  ```bash
  ./deploy-actor.sh hello-world
  ```

## Debugging
- Run `wash get hosts` to check if the edge node is connected.
- If `wash app deploy` fails, confirm that the actor is built correctly.