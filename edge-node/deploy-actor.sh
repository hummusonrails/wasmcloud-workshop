#!/bin/bash
echo "Deploying wasmCloud Actor..."

# Validate input
if [ -z "$1" ]; then
  echo "Error: Provide an actor name!"
  echo "Usage: ./deploy-actor.sh <actor-name>"
  exit 1
fi

ACTOR_NAME=$1

ROOT_DIR="$(cd "$(dirname "$0")/../" && pwd)"
ACTOR_FILE="${ROOT_DIR}/actors/${ACTOR_NAME}/wadm.yaml"

if [ ! -f "$ACTOR_FILE" ]; then
  echo "Error: Actor file '$ACTOR_FILE' not found!"
  exit 1
fi

# Deploy actor
wash app deploy "$ACTOR_FILE"

echo "Actor '$ACTOR_NAME' deployed successfully!"
