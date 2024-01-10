# Database things

## Creating a migration

From root, run:

```bash
sqlx migrate add <migration-name>
```

## Creating a fixture for tests

From root, run:

```bash
sqlx migrate add --source test/fixtures <migration-name>
```
