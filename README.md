# Axum Template

Production-grade web application template built with Axum framework.

## Features

- **Axum 0.8** — Modern async web framework
- **Diesel** — Type-safe ORM with PostgreSQL support
- **Auto-migrations** — Database migrations run automatically on startup
- **ULID** — Universally Unique Lexicographically Sortable Identifiers
- **JWT Authentication** — Access and refresh token support
- **Redis** — Caching and job queue
- **Email (lettre)** — SMTP email sending with HTML templates
- **Validation** — Request validation with validator
- **Rate Limiting** — Configurable rate limiting middleware
- **Structured Logging** — JSON and pretty logging with tracing

## Project Structure

```
src/
├── config/           # Configuration and settings
│   ├── database.rs   # Database pool setup
│   ├── settings.rs   # App settings from TOML/env
│   └── mod.rs
├── core/             # Core utilities
│   ├── error/        # Error handling (ApiError)
│   ├── extractors/   # Custom Axum extractors
│   ├── middleware/   # Auth, rate limit, request ID
│   ├── pagination.rs # Pagination helpers
│   └── response.rs   # API response wrappers
├── features/         # Feature modules
│   ├── auth/         # Authentication (login, register, refresh)
│   ├── health/       # Health check endpoint
│   ├── users/        # User CRUD operations
│   └── _template/    # Template for new features
├── infrastructure/   # External services
│   ├── cache/        # Redis client
│   ├── email/        # SMTP email client
│   ├── http/         # HTTP client
│   └── queue/        # Job queue
├── workers/          # Background workers
│   └── email_worker.rs
├── schema.rs         # Diesel schema
├── lib.rs            # Library exports
└── main.rs           # Application entry point

config/               # Configuration files
├── default.toml      # Default settings
├── development.toml  # Development overrides
├── production.toml   # Production overrides
└── test.toml         # Test overrides

migrations/           # Diesel migrations
tests/                # Integration tests
```

## Requirements

- Rust 2024 edition
- PostgreSQL 14+
- Redis 6+

## Quick Start

### 1. Clone and setup

```bash
git clone <repository>
cd axum_template
cp .env.example .env
```

### 2. Configure environment

Edit `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:5432/axum_template
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key-min-32-characters
```

### 3. Run services (Docker)

```bash
docker compose up -d postgres redis
```

### 4. Run the application

```bash
cargo run
```

The server starts at `http://localhost:3000`. Migrations run automatically.

## Configuration

Configuration is loaded from multiple sources (in order of priority):

1. Environment variables (highest priority)
2. `config/{APP_ENV}.toml` (development, production, test)
3. `config/default.toml` (lowest priority)

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APP_ENV` | Environment (development/production/test) | development |
| `APP_HOST` | Server host | 127.0.0.1 |
| `APP_PORT` | Server port | 3000 |
| `DATABASE_URL` | PostgreSQL connection URL | - |
| `REDIS_URL` | Redis connection URL | - |
| `JWT_SECRET` | JWT signing secret | - |
| `SMTP_HOST` | SMTP server host | localhost |
| `SMTP_PORT` | SMTP server port | 1025 |
| `SMTP_USERNAME` | SMTP username | - |
| `SMTP_PASSWORD` | SMTP password | - |
| `SMTP_FROM_EMAIL` | Sender email | noreply@example.com |
| `SMTP_FROM_NAME` | Sender name | Axum Template |

### TOML Configuration

```toml
[app]
name = "axum_template"
host = "127.0.0.1"
port = 3000

[database]
pool_size = 10
connection_timeout_secs = 30

[redis]
pool_size = 10

[jwt]
access_token_expires_in = "15m"
refresh_token_expires_in = "7d"

[rate_limit]
requests_per_second = 10
burst_size = 30

[logging]
level = "info"
format = "pretty"  # or "json"

[smtp]
host = "localhost"
port = 1025
username = ""
password = ""
from_email = "noreply@example.com"
from_name = "Axum Template"
tls = false
```

## API Endpoints

### Health

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |

### Authentication

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/auth/register` | Register new user |
| POST | `/api/v1/auth/login` | Login |
| POST | `/api/v1/auth/refresh` | Refresh tokens |
| POST | `/api/v1/auth/logout` | Logout |

### Users

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| GET | `/api/v1/users` | List users | No |
| GET | `/api/v1/users/:id` | Get user by ID | No |
| POST | `/api/v1/users` | Create user | No |
| GET | `/api/v1/users/me` | Get current user | Yes |
| PATCH | `/api/v1/users/me` | Update current user | Yes |
| PATCH | `/api/v1/users/:id` | Update user | Yes |
| DELETE | `/api/v1/users/:id` | Delete user | Yes |

## Development

### Run with auto-reload

```bash
cargo install cargo-watch
cargo watch -x run
```

### Run tests

```bash
cargo test
```

### Database migrations

Migrations run automatically on startup. To manage manually:

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Create migration
diesel migration generate create_table_name

# Run migrations
diesel migration run

# Revert migration
diesel migration revert
```

### Local email testing

Use [MailHog](https://github.com/mailhog/MailHog) for local email testing:

```bash
docker run -d -p 1025:1025 -p 8025:8025 mailhog/mailhog
```

Web UI: http://localhost:8025

## Adding a New Feature

1. Copy `src/features/_template` to `src/features/your_feature`
2. Rename files and update module names
3. Create migration for database tables
4. Register routes in `src/main.rs`
5. Add tests in `tests/integration/`

## Production Deployment

### Environment

```env
APP_ENV=production
JWT_SECRET=<strong-secret-min-32-chars>
DATABASE_URL=postgres://...
REDIS_URL=redis://...
SMTP_HOST=smtp.provider.com
SMTP_PORT=587
SMTP_USERNAME=apikey
SMTP_PASSWORD=<api-key>
SMTP_TLS=true
```

### Build

```bash
cargo build --release
./target/release/axum_template
```

### Docker

```dockerfile
FROM rust:1.83 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/axum_template /usr/local/bin/
COPY --from=builder /app/config /config
CMD ["axum_template"]
```

## License

MIT
