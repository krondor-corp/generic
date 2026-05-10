---
title: Storage
order: 12
---

File storage uses an S3-compatible interface. Locally, MinIO runs as a container. In production, any S3-compatible provider works.

Files are referenced by key in the database — store the S3 key, not a full URL. The storage client is injected via the `Context` dataclass, so application code never constructs clients directly.

## Usage

```python
async def upload_avatar(file: UploadFile, ctx: Context) -> str:
    key = f"avatars/{ctx.user_id}/{file.filename}"
    await ctx.storage.upload(key, file)
    return key

async def get_avatar_url(key: str, ctx: Context) -> str:
    return await ctx.storage.presigned_url(key)
```

## Local development

MinIO runs alongside PostgreSQL and Redis when you `make setup`:

| Service | Port | Console |
|---------|------|---------|
| MinIO API | 9000 | — |
| MinIO Console | 9001 | http://localhost:9001 |

Default credentials are `minioadmin` / `minioadmin`.

## Production

MinIO is deployed as a Kamal accessory alongside the Python service. The deploy config at `config/deploy/py.yml` defines the accessory and injects the connection details as environment variables.

## Best practices

- Store the S3 key in the database, never the full URL. Generate presigned URLs on demand.
- Use path prefixes to organize files by entity type and owner (e.g., `avatars/{user_id}/`, `exports/{thread_id}/`).
- Set lifecycle policies on your bucket for temporary files (e.g., exports expire after 7 days).
