---
title: Background Jobs
order: 10
---

Background work in the Python service is handled by **TaskIQ** with a **Redis** broker. TaskIQ workers run as a separate process alongside the FastAPI app.

## Simple tasks

For one-off background work, use the `@broker.task` decorator:

```python
from app.broker import broker

@broker.task
async def send_welcome_email(user_id: str) -> None:
    # ... send the email
    pass
```

Dispatch from application code:

```python
await send_welcome_email.kiq(user_id="abc123")
```

`.kiq()` enqueues the task and returns immediately. The worker picks it up asynchronously.

## Cron tasks

For scheduled recurring work, use the `@cron` decorator with distributed locking:

```python
from app.broker import broker, cron

@cron(broker, "0 */6 * * *", lock_ttl=300)
async def sync_external_data(
    db_session: AsyncSession = TaskiqDepends(get_db_session),
    logger: Logger = TaskiqDepends(get_logger),
) -> None:
    # runs every 6 hours
    # lock_ttl=300 means the lock expires after 5 minutes
    # if a previous run is still holding the lock, this invocation is skipped
    pass
```

Key points about cron tasks:

- **Distributed locking** prevents multiple workers from running the same cron job simultaneously. Always set `lock_ttl` to a reasonable upper bound for the task duration.
- If a task exceeds its `lock_ttl`, the lock expires and another worker may start a new run. Size `lock_ttl` accordingly.
- Cron expressions use standard 5-field format (`minute hour day month weekday`).

## Run tracking

Scheduled cron jobs are tracked in the `cron_job_runs` table. Each execution records:

- Task name
- Start and end timestamps
- Status (success / failure)
- Error details (if failed)

This provides observability into scheduled work without needing external monitoring.

## Dependency injection

TaskIQ uses `TaskiqDepends` for injecting dependencies into task functions. This is similar to FastAPI's `Depends` but for the worker context:

```python
from taskiq import TaskiqDepends

@broker.task
async def process_upload(
    file_id: str,
    db_session: AsyncSession = TaskiqDepends(get_db_session),
    logger: Logger = TaskiqDepends(get_logger),
    redis: Redis = TaskiqDepends(get_redis),
    event_publisher: EventPublisher = TaskiqDepends(get_event_publisher),
) -> None:
    ctx = Context(db=db_session, logger=logger)
    # ... process the file
```

Available dependencies:

| Dependency | Provider | Description |
|-----------|----------|-------------|
| `AsyncSession` | `get_db_session` | Database session (auto-committed/rolled-back) |
| `Logger` | `get_logger` | Structured logger |
| `Redis` | `get_redis` | Redis client |
| `EventPublisher` | `get_event_publisher` | Publish domain events |

## Best practices

- **Tasks must be idempotent.** Redis delivery is at-least-once, so a task may run more than once. Design accordingly (use upserts, check for existing results before creating).
- **Keep task arguments serializable.** Pass IDs and simple values, not ORM objects. The task function should fetch fresh data from the database.
- **Set reasonable timeouts.** Long-running tasks should periodically check for cancellation or use `lock_ttl` to bound execution.
- **Log generously.** Background tasks lack request context. Use the injected logger to make debugging possible.
- **Fail loudly.** Let exceptions propagate so they are captured in run tracking. Do not silently swallow errors.
