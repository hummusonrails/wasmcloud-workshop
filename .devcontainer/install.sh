#!/bin/bash

# INSTALL WASH
echo "Installing the wasmCloud CLI (wash)..."
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
sudo apt install wash openssl -y

# Configure connection to the central wasmCloud instance
export WASMCLOUD_CTL_NATS_HOST=nats://put-in-url-path-here:4222

echo "Codespace is now an edge node connected to the central wasmCloud!"