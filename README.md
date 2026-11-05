# rust-inertia-app

A full-stack application using Rust, Axum, Inertia.js v3, React, TypeScript, Vite, and Tailwind CSS 4.

Rust owns routing, request handling, and server-side logic. React owns the interactive UI. Inertia connects the two without a separate REST API or a client-side router for normal page navigation.

## Requirements

- Rust 1.99 or newer
- Node.js 20.19 or newer
- npm
- PostgreSQL 14 or newer (17.4 tested)
- On Windows: the **Desktop development with C++** workload in Visual Studio, so the MSVC linker is available

The Inertia Rust adapter is alpha and is currently consumed from a pinned Git revision. The `.cargo/config.toml` file declares the adapter's Pilot registry so Cargo can parse its manifest; no Pilot token is required for the pinned Git dependency.

## Setup

```bash
npm install
```

Create the local role and databases (adjust the password if your PostgreSQL setup does not use trust authentication):

```bash
psql -U postgres -d postgres -c "CREATE ROLE rust_inertia_app LOGIN PASSWORD 'change-me-local';"
createdb -U postgres -O rust_inertia_app rust_inertia_app
createdb -U postgres -O rust_inertia_app rust_inertia_app_test
```

If the role already exists, skip or update it. Copy `.env.example` to `.env` and adjust values if needed. The app reads:

| Variable | Default |
|---|---|
| `APP_ENV` | `local` |
| `APP_HOST` | `127.0.0.1` |
| `APP_PORT` | `3000` |
| `APP_URL` | `http://127.0.0.1:3000` |
| `DATABASE_URL` | `postgres://rust_inertia_app:change-me-local@127.0.0.1:5432/rust_inertia_app` |
| `TEST_DATABASE_URL` | `postgres://rust_inertia_app:change-me-local@127.0.0.1:5432/rust_inertia_app_test` |

`TEST_DATABASE_URL` is used only by integration tests. The default local password is `change-me-local`; it matches the setup command and is ignored by local trust authentication. Change it for any shared environment.

## Development

Run two terminals from the project root:

```bash
npm run dev     # Vite on http://localhost:5173
cargo run       # Rust server on http://127.0.0.1:3000
```

Open <http://127.0.0.1:3000>. Vite writes `public/hot` while it runs; the Rust root view points asset tags at the dev server.

Migrations run automatically when the server starts. Add a new `.sql` file under `migrations/`; SQLx applies pending migrations in order and records them in `_sqlx_migrations`. The same startup step creates the persistent session table. No separate migration command is required.

## Persistence

- The contact form inserts into `contact_messages`.
- The Contact page lists only the current browser session's 20 most recent messages; other visitors' submissions are not exposed.
- Registered accounts live in `users`. Passwords are hashed with Argon2id; plaintext passwords are never stored or logged.
- SQL uses bind parameters. User input is never concatenated into a query.
- Queries use SQLx runtime APIs (`query_as`) rather than compile-time checked macros, so builds and CI do not need a live database. Startup still applies migrations.
- Current writes are single-row inserts, so no explicit transaction is needed. Use `pool.begin()` when a future feature must write multiple rows atomically.
- Database errors map to a generic `500` response; details are logged on the server.

## Authentication and sessions

- `GET/POST /register`, `GET/POST /login`, and `POST /logout` implement the account flows.
- `/dashboard` requires a signed-in user. `/admin/users` requires `users.is_admin` and returns a plain `403 Forbidden` to non-admins.
- Registration requires a 2–80 character name, a valid email, and a password of 12–128 characters. Login failures use the generic message `These credentials do not match our records.`, whether or not the email exists.
- Sessions are stored server-side in `tower_sessions.session`, so they survive application restarts. A background task deletes expired sessions hourly.
- The session cookie is named `rust_inertia_session`, is `HttpOnly`, `SameSite=Strict`, scoped to `Path=/`, and expires after 14 days of inactivity. It is marked `Secure` whenever `APP_URL` starts with `https://`; use an HTTPS `APP_URL` in production.
- Login and registration rotate the session ID and the CSRF token. Logout flushes the session, so an old cookie can no longer load protected pages.
- Every page shares a `csrfToken` prop. Mutating requests must send it in the `X-CSRF-TOKEN` header; missing or invalid tokens receive `403 Invalid CSRF token.` Safe methods (`GET`, `HEAD`, `OPTIONS`, `TRACE`) are exempt.
- Protected pages set Inertia's history encryption, and logout clears Inertia history.
- Promote an existing account to administrator with:

  ```bash
  psql "postgres://rust_inertia_app:change-me-local@127.0.0.1:5432/rust_inertia_app" \
    -c "UPDATE users SET is_admin = TRUE WHERE email = 'you@example.com';"
  ```

  The auth middleware reloads the user from the database on each request, so the change applies immediately.

`tower-sessions` is on the 0.15 line. The `tower-sessions-sqlx-store` release on crates.io (`0.15.0`) still depends on `tower-sessions-core` 0.14, so it is pinned to a Git revision that uses core 0.15 and SQLx 0.8. Remove the pin once a compatible release is published.

## Production build

```bash
npm run build
cargo build --release
```

`npm run build` type-checks the frontend and writes the client bundle plus Vite's manifest to `public/build`. The Rust server serves those files at `/build`.

## Checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
npm run typecheck
npm run build
```

`cargo test` needs a reachable test database. It uses `TEST_DATABASE_URL`, or the default `rust_inertia_app_test` database shown above.

## Project layout

```text
src/
├── main.rs          process startup, graceful shutdown, migrations, session cleanup
├── lib.rs           router, middleware, Inertia configuration
├── auth.rs          auth middleware, user queries, password hashing, extractors
├── csrf.rs          CSRF token middleware and rotation
├── config.rs        environment-based runtime configuration
├── db.rs            PostgreSQL pool, app migrations, session store
├── errors.rs        shared application error responses
├── http.rs          Inertia response conversion helpers
├── validation.rs    shared validation helpers
├── vite.rs          Vite hot file and production manifest asset tags
└── features/        one module per page or business feature
migrations/          SQLx migration files (contact messages, users)
resources/
├── css/app.css      Tailwind entry point
└── js/
    ├── app.tsx      Inertia client setup
    ├── layouts/     shared application shell with auth navigation and flash
    ├── pages/       one React component per Inertia page
    └── types.ts     shared page prop types
tests/app.rs         protocol-level tests with AssertablePage
```

Server-side rendering is intentionally deferred until the client-rendered application is stable.
