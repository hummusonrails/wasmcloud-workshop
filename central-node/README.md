# Central wasmCloud Node

This directory contains all files needed to **run the central wasmCloud node** for the workshop.

## Files
* `docker-compose.yaml` - Starts the wasmCloud control plane and NATS.
* `start.sh` - Starts the central wasmCloud node.
* `init-couchbase.sh` - Ensures Couchbase Capella is properly configured.
* `wasmcloud.toml` - Configuration for the wasmCloud instance.
* `wadm.yaml` - Defines the deployment manifest for actors and providers.

## Running the Central Node

### Inside Google Cloud VM

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

### From a local machine using the `gcloud` CLI

1. Ensure that node can be reached externally by setting firewall rules using the `gcloud` CLI:
  ```bash
  gcloud compute firewall-rules create wasmcloud-allow \
    --allow=tcp:4222,tcp:8222,tcp:8080 \
    --source-ranges=0.0.0.0/0 \
    --target-tags=wasmcloud \
    --description="Allow wasmCloud and NATS"
  ```
2. Fetch the external IP address of the VM:
  ```bash
  gcloud compute instances describe wasmcloud-central --zone=replace-with-your-gcp-region-zone --format='get(networkInterfaces[0].accessConfigs[0].natIP)'
  ```

Share the IP address with the workshop participants to use with their edge node configuration on GitHub Codespaces.

## Debugging
- If Couchbase is not connecting, ensure that the Capella credentials are correct.
- Check if the NATS messaging service is reachable.
