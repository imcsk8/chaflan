# Chaflan

Chaflan is a lightweight Event Manager built with Rust, using the Rocket web framework and Diesel ORM with PostgreSQL.

## Features

- Event creation and management.
- User authentication (including OAuth support).
- CLI client for easy interaction.
- Containerized database setup.

## Technology Stack

- **Language:** Rust (Edition 2021)
- **Web Framework:** [Rocket](https://rocket.rs/)
- **ORM:** [Diesel](https://diesel.rs/)
- **Database:** PostgreSQL
- **Templates:** Handlebars (via `rocket_dyn_templates`)

## Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Podman](https://podman.io/) or Docker
- [Diesel CLI](http://diesel.rs/guides/getting-started) (`cargo install diesel_cli --no-default-features --features postgres`)

### Getting Started

1.  **Initialize directories and environment:**
    ```bash
    make dirs
    ```
    This creates the `data` and `www/static` directories and initializes a default `.env` file.

2.  **Start the database:**
    ```bash
    make db
    ```
    This starts a PostgreSQL 17 container using Podman.

3.  **Run migrations:**
    ```bash
    diesel migration run
    ```

4.  **Run the application:**
    ```bash
    make run
    ```
    The server will be available at `http://127.0.0.1:8000`.

## CLI Client (`utils/chaflan`)

A bash-based CLI utility is provided for interacting with the API.

### Usage

```bash
./utils/chaflan <subcommand> <action> [args]
```

### Subcommands

- **events**: Manage events.
  - `list`: List all events.
  - `get <uuid>`: Get details of a specific event.
  - `add`: Add a test event (requires `api_data/new_event.json`).
  - `delete <uuid>`: Delete an event.
- **tokens**: Manage authentication tokens.
  - `show`: Display the current JWT token.

*Note: The CLI automatically attempts to retrieve a JWT token by logging in with default credentials (`test` / `prueba123`) before executing commands.*

## Database Schema

### `events` Table

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | `UUID` | Primary Key |
| `name` | `TEXT` | Event name (Not Null) |
| `venue` | `TEXT` | Venue name (Not Null) |
| `address` | `TEXT` | Physical address |
| `image` | `VARCHAR(255)` | URL or path to event image |
| `comments` | `TEXT` | Additional notes |
| `contactname` | `VARCHAR(255)` | Contact person |
| `starts_at` | `TIMESTAMP` | Start time (Default: NOW) |
| `ends_at` | `TIMESTAMP` | End time (Default: NOW) |

### `users` Table

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | `UUID` | Primary Key |
| `name` | `TEXT` | User's full name |
| `email` | `TEXT` | User's email (Not Null, Unique) |
| `password` | `TEXT` | Hashed password |
| `oauth_provider` | `TEXT` | OAuth provider name |
| `oauth_user_id` | `TEXT` | External ID from OAuth provider |
| `access_token` | `TEXT` | Current access token |
| `refresh_token` | `TEXT` | Refresh token |
| `created_at` | `TIMESTAMP` | Record creation time |
| `updated_at` | `TIMESTAMP` | Record last update time |

### Database Functions

The following PostgreSQL functions are used for password management (requires `pgcrypto` extension):
- `get_pw_hash(password)`: Generates an MD5-salted hash.
- `check_pw_hash(password, hash)`: Verifies a password against a hash.
- `check_pw(email, password)`: Higher-level function to authenticate a user.

## Project Structure

- `src/`: Rust source code.
- `migrations/`: Diesel database migrations.
- `templates/`: Handlebars templates for the web UI.
- `www/static/`: Static assets (CSS, JS, Images).
- `utils/`: Utility scripts and CLI tools.
- `api_data/`: Sample JSON data for API testing.
