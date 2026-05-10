---
title: AI & Chat
order: 13
---

The Python stack includes a full AI chat system built on [pydantic-ai](https://ai.pydantic.dev/) with streaming responses, tool use, and async background tool execution.

## How it works

A chat interaction flows through several layers:

1. **User sends a message** via the admin UI or API
2. A `Thread` and `Completion` are created in the database
3. A TaskIQ background job picks up the completion
4. The `ChatEngine` builds a pydantic-ai agent and streams the response
5. Text chunks are published to Redis and delivered to the client via SSE
6. The LLM can call tools — some run inline, others dispatch to background jobs

## Agents

Agents are defined with the `AgentSpec` dataclass:

```python
def get_my_agent_spec() -> AgentSpec:
    return AgentSpec(
        model="claude-sonnet-4-5",
        system_prompt_builder=build_system_prompt,
        tools=[get_system_stats, run_analysis],
    )
```

The spec declares the model, a system prompt builder (async callable), and a list of tool functions. `build_agent()` turns a spec into a pydantic-ai `Agent` instance at runtime.

Agent dependencies are injected via `AgentDeps`, which carries the database session, a session factory for tools, logger, user ID, thread ID, and completion ID.

## Tools

Tools are async functions decorated with `@tool`. The decorator handles exceptions and serializes results.

**Sync tools** run inline during the agent loop and return immediately:

```python
@tool
async def get_system_stats(ctx: RunContext[AgentDeps]) -> ToolResultBase:
    async with ctx.deps.get_session() as session:
        count = await session.scalar(select(func.count(User.id)))
    return completed(
        data=SystemStatsPayload(user_count=count),
        message=f"{count} users in the system",
    )
```

**Async tools** dispatch a background job and return a `PendingResult`. The LLM sees that the tool is running and can continue the conversation. Results are injected into the next completion's context automatically.

```python
@tool
async def run_analysis(ctx: RunContext[AgentDeps]) -> ToolResultBase:
    execution = await create_async_tool_execution(...)
    await run_analysis_task.kiq(execution.id)
    return pending(message="Analysis started")
```

The background task uses the `@with_async_tool_lifecycle` decorator to handle status tracking, result storage, and event publishing.

## Tool results

All tools return a `ToolResultBase` subclass:

| Type | Use |
|------|-----|
| `completed(data=...)` | Sync success with typed payload |
| `pending(...)` | Async tool dispatched |
| `validation_error(...)` | Bad input |
| `not_found_error(...)` | Resource missing |
| `failed(...)` | System failure |

## Streaming

The `ChatEngine` uses pydantic-ai's `agent.iter()` to stream responses. Text chunks are published to a per-user Redis channel as `ThreadStreamEvent`s. The SSE endpoint (`/_events/chat/{thread_id}`) subscribes to this channel and delivers events to the browser.

The admin UI connects via HTMX's `sse-connect` and swaps chunks into the response area in real time.

## Database models

| Model | Purpose |
|-------|---------|
| `Thread` | Conversation container, belongs to a user |
| `Message` | User or assistant message with structured parts (text, tool calls, tool results) |
| `Completion` | Tracks a single agent run — status, tokens, timing, errors |
| `AsyncToolExecution` | Tracks a background tool run — status, result, errors |

## Cancellation

Users can cancel a running completion. The API sets a Redis key (`completion:cancel:{id}`) that the `ChatEngine` checks between streaming chunks and tool calls. Cancelled completions are marked in the database and a `ThreadCancelledEvent` is published.
