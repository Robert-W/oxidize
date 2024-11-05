# Database

## Creating a migration
```bash
sqlx migrate add <migration-name>
```

## Creating a fixture for tests
```bash
sqlx migrate add --source tests/fixtures -r <migration-name>
```

This will create up and down migrations. You should use something like
`TRUNCATE <table-name>;` in your down migration to reset the data created for
and by your tests.
