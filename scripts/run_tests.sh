#!/bin/bash

# Defaults for local development database
USER="${DB_USER:=username}"
PASS="${DB_PASS:=password}"
HOST="${DB_HOST:=localhost}"
PORT="${DB_PORT:=5432}"
NAME="${DB_NAME:=tests}"

# Export the full url
export DATABASE_URL="postgresql://$USER:$PASS@$HOST:$PORT/$NAME"

# Create a database for testing specifically
sqlx database create

# Run all migrations
sqlx migrate run

# Insert seed data
sqlx migrate run --source src/tests/fixtures --ignore-missing

# Run our tests
cargo test
exit_code=$?

# Clean up our test database
sqlx database drop -y

exit $exit_code
