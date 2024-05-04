# Testing

## Running integration tests
You can run integration tests locally using three steps at the moment. Always
looking for ways to improve this, but for now, do the following:

- `docker compose --file docker-compose.test.yml up`
- `DB_NAME=walrus_glitter cargo run`
- `./scripts/run_tests.sh`

This will run all migrations that have not already been run. It will also run
migrations in the `tests/fixtures` directory, followed by reverting those
migrations after the tests are done. You should not be creating tables in
`tests/fixtures`, those are meant to insert data needed for testing and then
clean the tables it used during testing. To add fixtures, see
[database.md](./database.md#creating-a-fixture-for-tests).

If you need to recreate the test database, you can do so via docker compose by
running `docker compose --file docker-compose.test.yml down --volumes`.
