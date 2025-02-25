#!/bin/bash

echo "Initializing Couchbase Capella Connection..."

# Ensure required environment variables are set
if [[ -z "$COUCHBASE_HOST" || -z "$COUCHBASE_USERNAME" || -z "$COUCHBASE_PASSWORD" || -z "$COUCHBASE_BUCKET" ]]; then
  echo "ERROR: Couchbase Capella environment variables not set!"
  echo "Please set COUCHBASE_HOST, COUCHBASE_USERNAME, COUCHBASE_PASSWORD, and COUCHBASE_BUCKET before running this script."
  exit 1
fi

echo "Couchbase Capella connection settings initialized."
