# Couchbase Capability Provider for wasmCloud

This directory contains the **Couchbase capability provider** for wasmCloud.

## Files
* `Dockerfile` - Defines the container for the Couchbase provider.
* `wasmcloud.toml` - Configuration file for the provider.

## Usage
1. Set Couchbase Capella credentials:
  ```bash
  export COUCHBASE_HOST=couchbases://your-capella-cluster.cloud.couchbase.com
  export COUCHBASE_USERNAME=your_capella_username
  export COUCHBASE_PASSWORD=your_capella_password
  export COUCHBASE_BUCKET=wasmcloud_workshop
  ```
2. Start the provider in wasmCloud:
  ```bash
  wash ctl start couchbase
  ```
3. Verify provider status:
  ```bash
  wash get providers
  ```

## Debugging
- If the provider fails to connect, check Couchbase credentials./
- Verify that the central wasmCloud node is running.