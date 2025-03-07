#!/bin/bash

echo "Connecting to Central wasmCloud Instance..."

# Ensure CENTRAL_WASMCLOUD_URL is set
if [[ -z "$CENTRAL_WASMCLOUD_URL" ]]; then
  echo "Error: CENTRAL_WASMCLOUD_URL is not set!"
  echo "Please set it before running this script."
  echo "Example: export CENTRAL_WASMCLOUD_URL=nats://34.42.212.147"
  exit 1
fi

# Connect the local wasmCloud host to the provided NATS URL
wash up -d --nats-host "$CENTRAL_WASMCLOUD_URL"

# Parse host and port from CENTRAL_WASMCLOUD_URL
host_port=${CENTRAL_WASMCLOUD_URL#nats://}
if [[ $host_port == *:* ]]; then
  host=${host_port%:*}
  port=${host_port#*:}
else
  host=$host_port
  port=4222
fi

# Verify connection
wash get hosts --ctl-host "$host" --ctl-port "$port"

echo "Successfully connected to the central wasmCloud control plane!"