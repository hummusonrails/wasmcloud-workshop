# Central wasmCloud Node

This directory contains all files needed to **run the central wasmCloud node** for the workshop.

## Files
* `docker-compose.yaml` - Starts the wasmCloud control plane and NATS.
* `start.sh` - Starts the central wasmCloud node.
* `init-couchbase.sh` - Ensures Couchbase Capella is properly configured.
* `wasmcloud.toml` - Configuration for the wasmCloud instance.
* `wadm.yaml` - Defines the deployment manifest for actors and providers.

## Running the Central Node
1. Set environment variables for Couchbase Capella:
  ```bash
  export COUCHBASE_HOST=couchbases://your-capella-cluster.cloud.couchbase.com
  export COUCHBASE_USERNAME=your_capella_username
  export COUCHBASE_PASSWORD=your_capella_password
  export COUCHBASE_BUCKET=wasmcloud_workshop
  ```
2. Start the central wasmCloud node:
  ```bash
  ./start.sh
  ```
3. Verify that wasmCloud is running:
  ```bash
  wash get hosts
  ```

## Debugging
- If Couchbase is not connecting, ensure that the Capella credentials are correct.
- Check if the NATS messaging service is reachable.
