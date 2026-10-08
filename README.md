# Rust API Template

A small Actix Web application template with PostgreSQL, Redis, in-memory caching, session-based auth, and a sample notes API. It is designed as a starter for backend services that need connection pooling, cache checks, and a simple route layout.

## What this project includes

- Actix Web API on port 8686
- PostgreSQL connection pool with warm-up support
- Redis JSON cache helpers
- Moka in-memory cache with typed caches
- Session-based authentication using Redis-backed sessions
- API-key middleware for internal endpoints
- Sample notes endpoints tied to a PostgreSQL `notes` table
- Docker Compose examples for local service dependencies

## Project layout

I have structured the project and explained in the [documentation](docs/README.md).


## Prerequisites

You need:

- Rust 1.85+ (this project uses edition 2024)
- PostgreSQL running locally or in Docker
- Redis running locally or in Docker
- Optional RabbitMQ env configuration because the app reads RMQ settings at startup


## Quick start

### 1) Start dependencies

Using Docker:

```bash
docker compose -f docker-compose-db.yaml up -d
```

That Docker Compose file only starts Redis. For PostgreSQL, you can use a direct container:

```bash
docker run --name rust-api-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  -d postgres:16
```

### 2) Configure environment variables

Copy the example below into a shell or `.env` file before running the app:

```bash
export RUST_LOG=rust_api=TRACE
# ^^^ Export it if you want to execute the application directly from the shell

# Check the `.env.example` file for all required environment variables
```

The app expects these variables to exist at startup. A root `.env.example` is included in the repo for convenience.

### 3) Run the service

```bash
cargo run
```

The server listens on:

```text
0.0.0.0:8686
```

## Auth and security model

This project currently uses two auth patterns, you can pick the appropriate one based on the route and use case or make your own combination.

### Session auth

- Created at `POST /auth/session`
- Session data is stored in Redis under a key like `session:<session_id>`
- The request requires:
  - `Session-ID` cookie
  - `x-csrf-token` header
  - `x-session-access-id` header
- Protected routes are under `/auth` and `/sample_db/*`

### API-key auth

- Used for `/internal/*` routes
- Requires the `x-api-key` header
- Value must match `SELF_API_KEY`
- Example protected paths: `/internal/api`, `/internal/pgsql`, `/internal/cache/redis`, `/internal/cache/in-mem`

### CORS

`ALLOWED_ORIGINS` configures the CORS allowlist, and `*` is accepted as a wildcard value.

## Testing

Run the test suite:

```bash
cargo test
```

This project includes environment parsing tests in `src/utils/initial.rs` and should be used as a baseline when editing configuration loading.

## Notes about the current implementation

This repository is a template and some pieces are intentionally lightweight. Notably:

- the sample auth flow is Redis-backed but minimal
- the notes API is a demo CRUD scaffold, not a full domain API
- the RabbitMQ setup is configured, but the app does not currently actively publish messages during startup
- background jobs reuse the same note deletion logic as a placeholder cleanup example

Contributions are welcome, and you can submit pull requests or issues to the repository.

## License

This project is released under the MIT license. See [LICENSE](LICENSE). You are free to use, modify, and distribute this template for any purpose.
