#!/bin/bash

echo "Connecting to Central wasmCloud Instance..."

# Ensure CENTRAL_WASMCLOUD_URL is set
if [[ -z "$CENTRAL_WASMCLOUD_URL" ]]; then
  echo "Error: CENTRAL_WASMCLOUD_URL is not set!"
  echo "Please set it before running this script."
  echo "Example: export CENTRAL_WASMCLOUD_URL=nats://your-deployed-wasmcloud.fly.dev:4222"
  exit 1
fi

# Connect the local wasmCloud host to the provided NATS URL
wash up -d --nats-connect "$CENTRAL_WASMCLOUD_URL"

# Verify connection
wash get hosts

echo "Successfully connected to the central wasmCloud control plane!"
