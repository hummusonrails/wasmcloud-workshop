#!/bin/bash

# INSTALL WASH
echo "Installing the wasmCloud CLI (wash)..."
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
sudo apt install wash openssl -y