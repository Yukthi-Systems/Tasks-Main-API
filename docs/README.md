# Documentation

This folder contains project-specific reference material for the API template.

## Documents

- [SQL DB Schema](./DB/schema.sql) — Database schema for the entire project and performance indexes


## Quick entry points

The following sections provide quick access to key parts of the project.

### Application

- [Main server entry](../src/main.rs) — Application entry point; initializes the server and starts the Actix Web application.

- [App state](../src/state.rs) — Shared application state and access to configured resources.

- [Routes](../src/routes) — HTTP route definitions and handlers.


### Configuration and infrastructure

- [Database](../src/database) — PostgreSQL integration and database access.
    -  [PostgreSQL setup](../src/database/pgsql.rs) — PostgreSQL configuration and setup.
    -  [Connection pool](../src/database/pool.rs) — Database connection pool.
    -  [Database queries](../src/database/pgsql) — SQL operations, including note-related queries and health checks.

- [Cache](../src/cache) — Caching infrastructure.
    -  [Moka cache](../src/cache/moka_cache.rs) — In-memory cache.
    -  [Redis cache](../src/cache/redis_cache.rs) — Redis-backed cache.

- [Messaging](../src/messaging) — RabbitMQ integration or any other messaging system used in the project.

- [Security](../src/security) — Security utilities.

- [Middleware](../src/middleware) — Request processing middleware.
    -  [Key-based authentication](../src/middleware/key_based.rs) — API-key-based request protection.
    -  [User session](../src/middleware/user_session.rs) — User session handling.


### Features and business logic

- [Features](../src/features) — Feature-oriented application modules.


### Background processing and utilities

- [Background tasks](../src/tasks) — Scheduled and recurring jobs and any other tasks not necessarily background-related.

- [Initialization utilities](../src/utils/initial.rs) — Application initialization helpers.

- [Logging and tracing application](../src/utils/logging.rs) — Logging and tracing setup during application initialization.

- [Error handling](../src/errors) — Application, API, PostgreSQL, Redis, and service error types.


### Custom Crates (Macros or Utilities)

- [Procedural macro](../crates/pg_row_derive) — Custom derive macro for mapping PostgreSQL rows to Rust structs.


## Runtime assumptions

The code currently expects these services to be available at startup:

- PostgreSQL connection configured via `POSTGRES_DB_URL`
- Redis connection configured via `REDIS_URL`
- Optional RabbitMQ values configured via `RABBITMQ_*` environment variables

If you are modifying the repo, start with the configuration guide before changing route or cache behavior.
