#!/bin/bash

# TODO: The tests are meant to be pointed at a test database. So this currently
# fails as it modifies the "tests" database. But the running application
# connects to a different database. So the tests are not querying the database
# with the fixtures they expect.
# We need to either run cargo run with an alternate database, or have multiple
# pools that we can connect to. This way we can pass in the database name as a
# url and connect to a test database that has migrations ran against it. This
# however means the database must be created on start.
# Another option, don't run fixtures. Have the tests connect to a specific
# version (dev or local), and create all the resoureces it needs and clean them
# up when we are done.

# Defaults for local development database
USER="${DB_USER:=username}"
PASS="${DB_PASS:=password}"
HOST="${DB_HOST:=localhost}"
PORT="${DB_PORT:=5432}"
NAME="${DB_NAME:=walrus_glitter}"

# Export any variables needed for the tests
export DATABASE_URL="postgresql://$USER:$PASS@$HOST:$PORT/$NAME"
export SERVICE_URL="${SERVICE_URL:=http://localhost:3000}"

# Run all migrations
sqlx migrate run

# Insert seed data
sqlx migrate run --source tests/fixtures --ignore-missing

# Run our tests
cargo test --test integration_tests
exit_code=$?

# Clean up our test database
sqlx migrate revert --source tests/fixtures --ignore-missing

exit $exit_code
