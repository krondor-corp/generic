---
title: Overview
order: 8
---

The Python stack is the best starting point for most apps. If you need a database, background jobs, user auth, AI chat, or an admin dashboard, start here. It ships a working FastAPI app with PostgreSQL, Redis, TaskIQ workers, pydantic-ai agents, and SSE streaming out of the box.

The service lives at `web/py/` and is deployed via Kamal with the config at `config/deploy/py.yml`.

## Stack

- **Framework:** FastAPI (async)
- **Database:** PostgreSQL 17 (deployed as a Kamal accessory)
- **Cache / Broker:** Redis 7 (deployed as a Kamal accessory)
- **Background Jobs:** TaskIQ with Redis broker
- **Migrations:** Alembic (auto-generated from SQLAlchemy models)
- **Port:** 8000

## Architecture overview

The codebase follows a **functional-first** pattern. Database models are plain data containers; business logic lives in pure async functions with `(params, ctx) -> result` signatures. A `Context` dataclass handles dependency injection (database session, logger, storage client).

- [Background Jobs]({% link _docs/py-background-jobs.md %}) — TaskIQ workers with Redis, cron scheduling, distributed locking
- [Database]({% link _docs/py-database.md %}) — PostgreSQL, Alembic migrations, soft deletes, multi-branch dev
- [AI & Chat]({% link _docs/py-ai.md %}) — pydantic-ai agents, streaming SSE, tool use, async tool execution

## Local development

```bash
cd web/py
make install
make setup      # starts postgres + redis containers, runs migrations
make dev
```

## Deployment

```bash
make kamal ARGS="py deploy"
```

The Kamal config builds from `web/py/Dockerfile` with the build context set to `web/`.

## Environment variables

Defined in `config/deploy/py.yml`:

| Variable | Value |
|----------|-------|
| `POSTGRES_URL` | `postgresql://postgres:postgres@<name>-py-postgres:5432/postgres` |
| `REDIS_URL` | `redis://<name>-py-redis:6379` |
| `HOST_NAME` | `https://py.<zone>` |
| `DEBUG` | `false` |

Secrets (`GOOGLE_O_AUTH_CLIENT_ID`, `GOOGLE_O_AUTH_CLIENT_SECRET`, `ANTHROPIC_API_KEY`) are injected from `.kamal/secrets`.

## Accessories

The Kamal config provisions two accessories alongside the app:

- **postgres** -- PostgreSQL 17 Alpine with persistent volume
- **redis** -- Redis 7 Alpine with persistent volume

## Health check

Kamal checks `/_status/livez` for readiness.

## Contributing

### Database

All database operations go through py-core (`web/py/packages/py-core/`). Applications import from py-core rather than accessing the database directly.

#### Local services

```bash
# From web/py/packages/py-core/
make setup              # Start postgres + redis, run migrations
make db-up              # Start PostgreSQL container only
make db-down            # Stop and remove container
make db-status          # Check container status
make db-endpoint        # Print connection URL
make wipe               # Drop branch-specific database (keeps container)
```

When working with git worktrees, each branch gets its own database (e.g., `krondor-generic_my_feature`). The `wipe` command only drops the database for your current branch.

#### Migrations

```bash
# From web/py/packages/py-core/

# Run pending migrations
make db-migrate

# Auto-generate from model changes
make db-prepare MSG="add user preferences table"

# Manual migration (data migrations, complex changes)
MANUAL=1 make db-prepare MSG="migrate legacy data"
```

Migration files live in `web/py/packages/py-core/alembic/versions/`.

**Best practices:**
- One logical change per migration
- Always test `downgrade()`
- Review auto-generated migrations before committing
- Never modify existing migrations once pushed

#### Soft deletes

Never hard delete records. Use soft deletes with an `archived_at` timestamp:

```python
class MyModel(Base):
    archived_at: Mapped[datetime | None] = mapped_column(DateTime, nullable=True, default=None)
```

Set `archived_at = utcnow()` instead of deleting. Filter with `WHERE archived_at IS NULL`.

#### Creating models

Models live in `web/py/packages/py-core/src/py_core/database/models/`.

```python
from datetime import datetime
from sqlalchemy import String, ForeignKey, DateTime
from sqlalchemy.orm import Mapped, mapped_column
from py_core.database.client import Base
from py_core.database.utils import utcnow, uuid7_str

class UserPreference(Base):
    __tablename__ = "user_preferences"

    id: Mapped[str] = mapped_column(String, primary_key=True, default=uuid7_str)
    user_id: Mapped[str] = mapped_column(
        String, ForeignKey("users.id", ondelete="CASCADE"), nullable=False, index=True
    )
    theme: Mapped[str] = mapped_column(String, nullable=False, default="light")
    created_at: Mapped[datetime] = mapped_column(default=utcnow)
    updated_at: Mapped[datetime] = mapped_column(default=utcnow, onupdate=utcnow)
```

**ID strategies:**
- Content-addressed (immutable): `prefix_<blake2b_hash>` (e.g., `con_`, `seq_`)
- User-mutable: UUID7 via `uuid7_str`

After creating a model: add it to `database/models/__init__.py`, generate a migration with `make db-prepare`, review, and apply with `make db-migrate`.

### Cleanup

| Command | Effect |
|---------|--------|
| `make clean` | Remove build artifacts (`__pycache__`, `.venv`) |
| `make wipe` | Drop branch database (container stays running) |
| `make teardown` | Remove all containers and volumes |

**Reset workflow:** `make wipe && make setup`

**Full reset:** `make teardown && make setup`
