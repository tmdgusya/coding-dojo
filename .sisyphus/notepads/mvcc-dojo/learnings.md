# Learnings - mvcc-dojo

Conventions, patterns, and best practices discovered during implementation.

---

## Wave 1, Task 1: Docker + Project Infrastructure Setup

### Setup Completed
- ✅ uv project initialized with psycopg2-binary and pytest dependencies
- ✅ docker-compose.yml created with PostgreSQL 17 configuration
- ✅ src/__init__.py and tests/__init__.py created
- ✅ Project structure ready for MVCC observation experiments

### Docker Configuration Notes
- PostgreSQL 17 image used (latest stable)
- Container name: mvcc-dojo-pg
- Port mapping: 15432 (host) → 5432 (container)
- Database: mvcc_dojo
- User: dojo / Password: dojo
- autovacuum: OFF (critical for MVCC observation)
- log_statement: all (for debugging)

### Key Settings for MVCC Observation
- autovacuum=off is essential to prevent automatic cleanup of dead tuples
- This allows us to observe MVCC behavior without interference
- log_statement=all helps track transaction activity

### Verification Commands (when Docker daemon is available)
```bash
cd mvcc-dojo
docker compose up -d
docker compose exec postgres pg_isready -U dojo -d mvcc_dojo
docker compose exec postgres psql -U dojo -d mvcc_dojo -c "SHOW autovacuum;"
```

### Environment Notes
- Docker Desktop on macOS requires daemon to be running
- Use `docker context use desktop-linux` if needed
- Socket location: /Users/roach/.docker/run/docker.sock

## Wave 1, Task 2: Common Code Generation

### Files Created
- ✅ src/connection.py: Database connection helpers
- ✅ src/observations.py: 7 observation function stubs
- ✅ tests/conftest.py: pytest fixtures for table setup/teardown

### connection.py Implementation
- `DB_CONFIG` dict: localhost:15432, mvcc_dojo database, dojo user
- `get_connection(autocommit=False)`: Returns psycopg2 connection with configurable autocommit
- `execute(sql, params=None, autocommit=True)`: Single SQL execution with automatic connection management
- Connection cleanup handled in finally block to prevent leaks

### observations.py Structure
- 7 observation functions with detailed docstrings
- Each docstring includes:
  - Scenario description (step-by-step test plan)
  - Expected return dict structure
  - Mission number and goal
- All functions raise NotImplementedError (to be implemented in later tasks)
- Function names:
  1. observe_reader_not_blocking_writer
  2. observe_snapshot_isolation
  3. observe_dead_tuples
  4. observe_vacuum_effect
  5. observe_xid_status
  6. observe_write_write_conflict
  7. observe_long_tx_blocks_vacuum

### conftest.py Fixture Pattern
- `@pytest.fixture(autouse=True)` ensures setup/teardown runs for every test
- Creates 6 tables: reader_writer_test, accounts, bloat_test, vacuum_test, conflict_test, long_tx_test
- Uses `autocommit=True` for DDL operations (CREATE/DROP TABLE)
- Proper cleanup in teardown: DROP tables with CASCADE, close cursor and connection
- Yield pattern separates setup from teardown

### Import Verification
- All imports work correctly with `uv run python -c "..."`
- Module structure allows clean imports: `from src.connection import get_connection`
- All 7 observation functions importable from src.observations

### Key Patterns
- Use `autocommit=True` for DDL (CREATE TABLE, DROP TABLE, VACUUM)
- Use `autocommit=False` for transactional observation (BEGIN/COMMIT)
- Always close cursor and connection to prevent resource leaks
- Fixture teardown must clean up all test tables to avoid state pollution

