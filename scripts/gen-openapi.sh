#!/bin/bash

# generates the openapi spec from the server

SERVER_URL="http://localhost:6969/api-docs/openapi.json"

wait_for_server() {
    local timeout=5
    local max_attempts=10
    local attempt=1

    until curl --output /dev/null --silent --fail "$SERVER_URL"; do
        if [ $attempt -ge $max_attempts ]; then
            echo "Server did not start within the specified timeout"
            exit 1
        fi
        echo "Waiting for the server to start (Attempt $attempt/$max_attempts)..."
        sleep $timeout
        attempt=$((attempt + 1))
    done
}

echo "Starting the server..."
cargo run -- run --server.enabled &

wait_for_server
curl --silent "$SERVER_URL" | yq --prettyPrint > crates/chainthru-server/openapi.yaml

kill $(jobs -p)
echo "OpenAPI spec has been generated and saved as YAML"
