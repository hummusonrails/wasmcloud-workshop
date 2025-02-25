#!/bin/bash

echo "Starting NATS Messaging Provider..."

# Ensure NATS URL is set
if [[ -z "$CENTRAL_WASMCLOUD_URL" ]]; then
  echo "Error: CENTRAL_WASMCLOUD_URL is not set!"
  echo "Please set it before running this script."
  echo "Example: export CENTRAL_WASMCLOUD_URL=nats://your-deployed-wasmcloud.fly.dev:4222"
  exit 1
fi

# Start the provider
wash ctl start nats-messaging

# Verify that the provider is running
wash get providers

echo "NATS Messaging Provider is now running!"
