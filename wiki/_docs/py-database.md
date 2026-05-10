---
title: Database
order: 11
---

The Python service uses **PostgreSQL 17** with **SQLAlchemy async** for the ORM and **Alembic** for schema migrations. Object storage is handled by **S3/MinIO**.

## Alembic migrations

### Auto-generated migrations

When you change a SQLAlchemy model, Alembic can detect the diff and generate a migration:

```bash
cd web/py
make db-migrate MSG="add widgets table"
```

This compares the current models against the database schema and produces a migration file in `alembic/versions/`. Always review the generated migration before applying it.

### Manual migrations

For migrations that Alembic cannot auto-detect (data migrations, custom SQL, index tweaks), generate an empty migration:

```bash
cd web/py
MANUAL=1 make db-migrate MSG="backfill widget slugs"
```

This creates a migration file with empty `upgrade()` and `downgrade()` functions for you to fill in.

### Applying migrations

```bash
cd web/py
make db-upgrade
```

Migrations run automatically on deploy. For local development, they run when the app container starts.

## Model creation checklist

When adding a new database model:

1. Define the model class with `uuid7` primary key and timestamp columns
2. Add `archived_at` column if the model supports deletion
3. Run `make db-migrate MSG="add <table> table"` to generate the migration
4. Review the generated migration file
5. Run `make db-upgrade` to apply
6. Add the model to `__init__.py` exports so Alembic can discover it

Example model skeleton:

```python
from uuid_extensions import uuid7

class Widget(Base):
    __tablename__ = "widgets"

    id = Column(String, primary_key=True, default=lambda: str(uuid7()))
    name = Column(String, nullable=False)
    status = Column(StringEnum(WidgetStatus), nullable=False)
    created_at = Column(DateTime(timezone=True), nullable=False, server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())
    archived_at = Column(DateTime(timezone=True), nullable=True)
```

## Soft deletes

Records are never hard-deleted from the database. Deletion sets the `archived_at` timestamp:

```python
async def archive_widget(widget_id: str, ctx: Context) -> Widget:
    widget = await get_widget(widget_id, ctx)
    widget.archived_at = utcnow()
    await ctx.db.commit()
    return widget
```

All read queries should filter out archived records by default:

```python
select(Widget).where(Widget.archived_at.is_(None))
```

## Storage (S3/MinIO)

File storage uses an S3-compatible interface. Locally, MinIO runs as a Docker Compose service. In production, any S3-compatible provider works.

Files are referenced by key in the database (store the S3 key, not a full URL). The storage client is injected via the `Context` dataclass.

## Cleanup levels

The database scripts support three levels of cleanup:

| Level | Command | Effect |
|-------|---------|--------|
| **clean** | `make db-clean` | Drop and recreate the database, run all migrations |
| **wipe** | `make db-wipe` | Delete all data but keep the schema |
| **teardown** | `make db-teardown` | Remove the database and volumes entirely |

Use `clean` during development when migrations get tangled. Use `wipe` to reset data without losing schema. Use `teardown` only when you want to start completely fresh (e.g., switching PostgreSQL versions).

## Multi-branch databases

Local development supports per-branch databases. Each git branch gets its own PostgreSQL database, so switching branches does not require re-running migrations or losing data.

The database name is derived from the current branch name. The `db.sh` script handles creation and switching automatically. When you check out a branch, the app connects to that branch's database (creating it if it does not exist).

This avoids the common problem of migration conflicts between feature branches.
