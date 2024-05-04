# Database things

## Creating a migration

From root, run:

```bash
sqlx migrate add <migration-name>
```

## Creating a fixture for tests

From root, run:

```bash
sqlx migrate add --source tests/fixtures <migration-name>
```
