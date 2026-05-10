---
title: Background Jobs
order: 14
---

Background jobs in the Rust service use [Apalis](https://github.com/geofmureithi/apalis), a Rust job processing framework backed by SQLite. Jobs are defined with a `Task` trait, registered at startup, and pushed from handlers.

## The Task trait

Each background job implements the `Task` trait in `tasks/`:

```rust
#[async_trait]
pub trait Task: Serialize + DeserializeOwned + Send + 'static {
    /// Identifier for this worker, used in logging and registration.
    const WORKER_NAME: &'static str;

    /// Register this task type with the Apalis worker builder.
    fn register(
        worker: WorkerBuilder<Context>,
        state: AppState,
        storage: SqliteStorage<Self>,
    ) -> WorkerBuilder<Context>
    where
        Self: Sized;

    /// Push a new instance of this task onto the queue.
    async fn push(storage: &SqliteStorage<Self>, task: Self) -> Result<()>;
}
```

Each task is a plain struct that derives `Serialize` and `Deserialize`. Apalis handles serialization to and from the SQLite job queue.

## Defining a task

Create a new file in `tasks/` for each job type:

```rust
// tasks/send_welcome_email.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct SendWelcomeEmail {
    pub user_id: Uuid,
}

#[async_trait]
impl Task for SendWelcomeEmail {
    const WORKER_NAME: &'static str = "send_welcome_email";

    fn register(
        worker: WorkerBuilder<Context>,
        state: AppState,
        storage: SqliteStorage<Self>,
    ) -> WorkerBuilder<Context> {
        worker.register_with_count::<_, _, SqliteStorage<Self>>(
            1, // concurrency
            move |task: Self, ctx| {
                let state = state.clone();
                async move {
                    let user = User::get(&state.db, &DbUuid::from(task.user_id)).await?;
                    // ... send email
                    Ok(())
                }
            },
            &storage,
        )
    }

    async fn push(storage: &SqliteStorage<Self>, task: Self) -> Result<()> {
        storage.push(task).await?;
        Ok(())
    }
}
```

## Registering tasks

All tasks are registered in the `TaskWorker`'s `Service::run` implementation. The `TaskWorker` is one of the services booted from `main.rs`:

```rust
impl Service for TaskWorker {
    async fn run(state: AppState) -> Result<Self> {
        let storage = SqliteStorage::new(state.db.clone()).await?;

        let worker = WorkerBuilder::new(WORKER_NAME)
            .enable_tracing();

        // Register each task type
        let worker = SendWelcomeEmail::register(worker, state.clone(), storage.clone());
        let worker = ProcessUpload::register(worker, state.clone(), storage.clone());

        let handle = worker.build();
        Ok(Self { handle })
    }
}
```

Adding a new task means implementing the `Task` trait and adding one `register` call here. Apalis manages its own migration table in the SQLite database -- no manual migration needed for the job queue.

## Pushing jobs from handlers

Handlers push tasks onto the queue through the storage handle on `AppState`:

```rust
// http/html/actions/create_user.rs
pub async fn handler(
    State(state): State<AppState>,
    Form(input): Form<CreateUserInput>,
) -> impl IntoResponse {
    let user = User::create(&state.db, &input.name, &input.email).await?;

    // Enqueue background work
    SendWelcomeEmail::push(&state.task_storage, SendWelcomeEmail {
        user_id: *user.id(),
    }).await?;

    Redirect::to("/users")
}
```

The job is persisted to SQLite immediately and picked up by the worker on its next poll cycle.

## Worker lifecycle

The `TaskWorker` follows the same `Service` trait lifecycle as every other subsystem:

1. **`run()`** -- Connects to the SQLite-backed queue, registers all task types, builds the worker.
2. **`spawn()`** -- Moves the worker onto a Tokio task where it polls for jobs.
3. **`shutdown()`** -- Signals the worker to stop accepting new jobs and drain in-flight work.

Failed jobs are retried according to Apalis's default retry policy. Check Apalis docs for tuning retry counts and backoff.
