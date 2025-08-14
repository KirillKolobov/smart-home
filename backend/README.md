# Smart Home Backend

A modern, secure REST API for smart home management built with Rust, Axum, and PostgreSQL.

## Features

- **Authentication & Authorization**: JWT-based authentication with secure password hashing
- **User Management**: Complete user lifecycle management with role-based access
- **Clean Architecture**: Repository pattern, service layer, and proper error handling
- **API Documentation**: Auto-generated OpenAPI/Swagger documentation
- **Comprehensive Testing**: Unit tests, integration tests, and test coverage
- **Security**: Input validation, SQL injection protection, and secure middleware
- **Observability**: Structured logging with tracing
- **Database Migrations**: Automated database schema management

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: JWT tokens with [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- **Password Hashing**: [bcrypt](https://github.com/Walther/rust-bcrypt)
- **Validation**: [validator](https://github.com/Keats/validator)
- **Documentation**: [utoipa](https://github.com/juhaku/utoipa) for OpenAPI
- **Testing**: [axum-test](https://github.com/JosephLenton/axum-test) and [mockall](https://github.com/asomers/mockall)

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- PostgreSQL 12+
- (Optional) Docker for containerized database

### 1. Database Setup

#### Option A: Local PostgreSQL

```bash
# Create database and user
sudo -u postgres psql
CREATE DATABASE smart_home;
CREATE USER smart_home_user WITH PASSWORD '1234';
GRANT ALL PRIVILEGES ON DATABASE smart_home TO smart_home_user;
ALTER USER smart_home_user WITH SUPERUSER;
\q
```

#### Option B: Docker

```bash
docker run --name smart-home-db \
  -e POSTGRES_USER=smart_home_user \
  -e POSTGRES_PASSWORD=1234 \
  -e POSTGRES_DB=smart_home \
  -p 5432:5432 \
  -d postgres:15
```

### 2. Environment Configuration

Copy the example environment file and configure:

```bash
cp .env.example .env
```

Edit `.env`:

```env
DATABASE_URL="postgres://smart_home_user:1234@127.0.0.1:5432/smart_home"
PORT=3000
DB_HOST=127.0.0.1
DB_PORT=5432
DB_NAME=smart_home
DB_USER=smart_home_user
DB_PASSWORD=1234
JWT_SECRET=your_super_secure_secret_key_here_at_least_32_chars
JWT_EXPIRES_IN=3600
```

### 3. Install Dependencies & Run

```bash
# Install SQLx CLI for migrations
cargo install sqlx-cli --no-default-features --features postgres

# Run database migrations
sqlx migrate run

# Start the development server
cargo run
```

The server will start on `http://localhost:3000`

## API Documentation

Once running, visit:

- **Swagger UI**: http://localhost:3000/swagger-ui
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── errors.rs            # Error types and handling
├── db.rs               # Database connection wrapper
├── api_doc.rs          # OpenAPI documentation
├── handlers/           # HTTP request handlers
│   ├── mod.rs
│   ├── auth.rs         # Authentication endpoints
│   └── users.rs        # User management endpoints
├── middlewares/        # HTTP middlewares
│   ├── mod.rs
│   └── auth.rs         # JWT authentication middleware
├── models/             # Data models
│   ├── mod.rs
│   ├── auth.rs         # Authentication models
│   └── users.rs        # User models
├── repositories/       # Database access layer
│   ├── mod.rs
│   └── user_repository.rs
├── routes/             # Route definitions
│   ├── mod.rs
│   ├── auth.rs         # Authentication routes
│   └── users.rs        # User routes
├── services/           # Business logic layer
│   ├── mod.rs
│   ├── auth.rs         # Authentication service
│   └── user_service.rs # User service
└── tests/              # Integration tests
    ├── mod.rs
    └── integration_tests.rs
```

## API Endpoints

### Authentication

| Method | Endpoint    | Description       | Auth Required |
| ------ | ----------- | ----------------- | ------------- |
| POST   | `/register` | Register new user | No            |
| POST   | `/login`    | Login user        | No            |

### Users

| Method | Endpoint              | Description      | Auth Required |
| ------ | --------------------- | ---------------- | ------------- |
| GET    | `/users/{id}`         | Get user by ID   | Yes           |
| GET    | `/users/{id}/profile` | Get user profile | Yes           |
| DELETE | `/users/{id}`         | Delete user      | Yes           |

### Health

| Method | Endpoint  | Description  | Auth Required |
| ------ | --------- | ------------ | ------------- |
| GET    | `/health` | Health check | No            |

## Testing

### Running Tests

We provide a comprehensive test script:

```bash
# Run unit tests only (fast)
./scripts/test.sh unit

# Run integration tests (requires database)
./scripts/test.sh integration

# Run all tests
./scripts/test.sh all

# Generate coverage report
./scripts/test.sh coverage

# Run code quality checks
./scripts/test.sh check

# Run CI pipeline
./scripts/test.sh ci
```

### Test Types

- **Unit Tests**: Fast tests that mock dependencies
- **Integration Tests**: Full API tests with real database
- **Coverage**: Code coverage analysis with cargo-tarpaulin

### Setting Up Test Database

For integration tests, set up a test database:

```bash
# Create test database
sudo -u postgres createdb smart_home_test
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE smart_home_test TO smart_home_user;"

# Or use the helper script
./scripts/test.sh setup-db
```

Set the test database URL:

```bash
export TEST_DATABASE_URL="postgres://smart_home_user:1234@127.0.0.1:5432/smart_home_test"
```

## Development

### Code Quality

```bash
# Format code
cargo fmt

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Run all checks
./scripts/test.sh check
```

### Database Migrations

```bash
# Create new migration
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Adding New Features

1. **Models**: Define data structures in `src/models/`
2. **Repository**: Add database operations in `src/repositories/`
3. **Service**: Implement business logic in `src/services/`
4. **Handler**: Create HTTP handlers in `src/handlers/`
5. **Routes**: Wire up routes in `src/routes/`
6. **Tests**: Add unit and integration tests
7. **Documentation**: Update OpenAPI docs in handlers

## Architecture Principles

### Clean Architecture

- **Handlers**: HTTP-specific logic only
- **Services**: Business logic and validation
- **Repositories**: Database access and queries
- **Models**: Data structures and domain objects

### Error Handling

- Custom `AppError` type with proper HTTP status mapping
- Consistent error responses across all endpoints
- Structured logging for debugging

### Security

- JWT tokens for stateless authentication
- bcrypt for password hashing with salt
- Input validation on all endpoints
- SQL injection protection with SQLx
- CORS and security headers (TODO)

### Testing Strategy

- **Unit tests**: Test individual components with mocks
- **Integration tests**: Test full API flows with real database
- **Repository tests**: Test database operations
- **Service tests**: Test business logic
- **Handler tests**: Test HTTP layer

## Performance Considerations

- Connection pooling with configurable pool size
- Async/await throughout the application
- Efficient database queries with SQLx compile-time checking
- Structured logging with minimal overhead

## Deployment

### Environment Variables

Required environment variables for production:

```env
DATABASE_URL=postgres://user:password@host:port/database
PORT=3000
JWT_SECRET=your_secure_secret_minimum_32_characters
JWT_EXPIRES_IN=3600
RUST_LOG=info
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/smart-home-backend /usr/local/bin/smart-home-backend
EXPOSE 3000
CMD ["smart-home-backend"]
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `./scripts/test.sh ci`
5. Commit your changes: `git commit -am 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

### Code Style

- Follow Rust naming conventions
- Add documentation for public APIs
- Include unit tests for new functionality
- Update integration tests for API changes
- Run `cargo fmt` and `cargo clippy` before committing

## Troubleshooting

### Common Issues

**Database Connection Errors**

- Check PostgreSQL is running: `systemctl status postgresql`
- Verify database credentials in `.env`
- Ensure database and user exist

**Migration Failures**

- Check database permissions
- Verify SQLx CLI is installed: `cargo install sqlx-cli`
- Run migrations manually: `sqlx migrate run`

**Test Failures**

- Ensure test database is set up
- Check `TEST_DATABASE_URL` environment variable
- Run tests individually: `cargo test test_name`

**JWT Token Issues**

- Verify `JWT_SECRET` is at least 32 characters
- Check token expiration time
- Ensure proper Bearer token format: `Authorization: Bearer <token>`

### Logging

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0 (Current)

- Initial release
- User registration and authentication
- JWT-based authorization
- Basic user management
- OpenAPI documentation
- Comprehensive test suite
- Database migrations

## Roadmap

- [ ] Device management endpoints
- [ ] Room and area management
- [ ] Device automation rules
- [ ] Real-time WebSocket connections
- [ ] Email notifications
- [ ] Admin dashboard
- [ ] API rate limiting
- [ ] Caching layer
- [ ] Monitoring and metrics
- [ ] Docker Compose setup
