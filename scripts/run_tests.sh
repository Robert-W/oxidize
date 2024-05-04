#!/bin/bash

# Defaults for local development database
USER="${DB_USER:=username}"
PASS="${DB_PASS:=password}"
HOST="${DB_HOST:=localhost}"
PORT="${DB_PORT:=5432}"
NAME="${DB_NAME:=urban_potato}"

# Export any variables needed for the tests
export DATABASE_URL="postgresql://$USER:$PASS@$HOST:$PORT/$NAME"
export SERVICE_URL="${SERVICE_URL:=http://localhost:3000}"

# Create a database for testing specifically
sqlx database create

# Run all migrations
sqlx migrate run

# Insert seed data
sqlx migrate run --source tests/fixtures --ignore-missing

# Run our tests
cargo test --test integration_tests
exit_code=$?

# Clean up our test database
sqlx database drop -y

exit $exit_code
