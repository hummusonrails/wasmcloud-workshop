#!/bin/bash

echo "Stopping NATS Messaging Provider..."

wash ctl stop nats-messaging

echo "NATS Messaging Provider stopped successfully!"
