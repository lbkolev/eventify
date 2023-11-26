#!/bin/bash

# Run migrations
chainthru run --only-migrations

# Start the main application
exec chainthru run --indexer.enabled --server.enabled --src-block=3000000 --dst-block=3001000
