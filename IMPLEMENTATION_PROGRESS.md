# Implementation Progress

## Project audit

- [x] Existing project inspected
- [x] Git working tree inspected
- [x] Rust and Node toolchains verified
- [x] Inertia Rust adapter README and source reviewed
- [x] Demo app and Vite integration reviewed (public `inertia-omega-showcase` used; the adapter's referenced demo is not public)
- [x] Dependency and feature compatibility confirmed
- [x] Implementation plan approved

### Audit notes

- Existing project was a single `Hello, world!` binary with no dependencies, no frontend, and no commits.
- Adapter pinned to `inertia-laravel-rust` `f8c46e734cedeea99a5417783b4d7a6488c062f8`.
- Adapter is alpha; it is not on crates.io. It declares the Pilot registry for its macro crate, so `.cargo/config.toml` declares that registry even though the dependency is resolved from Git.
- `laravel-vite-rust` is not publicly available, so `src/vite.rs` implements the needed hot-file and manifest behavior locally.
- SSR is deferred.

### Toolchain notes

- The **Desktop development with C++** workload is now installed. `cargo build`, `cargo test`, and `cargo run` work from a normal terminal.
- Before that install, `link.exe` resolved to Git's coreutils `link`; that blocker is resolved.
- The GNU toolchain (`stable-x86_64-pc-windows-gnu`) remains available for checks, but linking with it still fails on this machine. `windows-sys`/`windows-link` use `raw-dylib`, and Rust invokes `dlltool` with a `--temp-prefix` ending in `:`, which the bundled binutils 2.44 `dlltool` cannot create. Use the MSVC toolchain for builds and tests.
- If GNU checks are needed, add the self-contained binutils directory to `PATH`:
  `export PATH="/c/Users/calex/.rustup/toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/bin/self-contained:$PATH"`

## Phase 1 — Foundation

- [x] Axum and Inertia integrated
- [x] Root HTML view configured
- [x] Session middleware order verified in code (session layer outside Inertia layer)
- [x] React + TypeScript configured
- [x] Vite configured
- [x] Tailwind CSS 4 configured
- [x] First Inertia pages implemented (`Home`, `About`)
- [x] Rust server verified locally (`cargo build`, `cargo run`, `/health` returned `200 ok`)
- [x] Inertia client-side navigation verified in headless Chrome (`/` to `/about` without a full page reload)
- [x] Production frontend build verified (`npm run build`, 592 modules, manifest generated)
- [x] Rust formatting and Clippy verified
- [x] Rust tests executed locally (2 unit tests and 4 integration tests passed)

### Phase 1 verification log

- `npm install`: 48 packages installed, 0 vulnerabilities.
- `npm run typecheck`: passed.
- `npm run build`: passed; Vite wrote `public/build/.vite/manifest.json` plus the entry JS/CSS and the `Home`/`About` dynamic chunks.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test`: passed; 2 unit tests and 4 integration tests.
- `cargo build`: passed.
- Runtime, production asset mode: `/` returned HTML with `/build/assets/...` tags; `/about` returned `200` JSON for `About` when sent `X-Inertia: true` and the matching version; a stale version returned `409` with `X-Inertia-Location`.
- Runtime, Vite dev mode: `npm run dev` wrote `public/hot`, served `resources/js/app.tsx` and `@vite/client`, and the Rust root view switched to `http://localhost:5173` tags with Inertia version `dev`.
- Headless Chrome (DevTools Protocol): initial load rendered the Home React component; clicking the About link changed the URL to `/about` and rendered the About component while `performance.getEntriesByType("navigation").length` stayed at `1`, confirming a soft Inertia navigation.
- Headless Chrome reported 0 console errors, 0 uncaught exceptions, 0 failed requests, and 0 HTTP responses at or above 400 after adding a data-URI favicon to prevent the default `/favicon.ico` 404.
- Server and Vite processes were stopped and `public/hot` was removed after verification.

## Phase 2 — Inertia workflows

- [x] Shared props (`appName`, `environment`)
- [x] Validation errors (server-side `ValidationErrors`, field-level errors on `Contact`)
- [x] Successful form submission (`POST /contact`)
- [x] Redirects (302 after `POST`, 303 after `PUT`, `PATCH`, and `DELETE`)
- [x] Flash messages (success flash shown once and not kept in browser history)
- [x] Consistent application error type (`AppError`: generic 400, 404, and 500 responses)
- [x] Reusable `useForm` example with processing/disabled state and accessible labels
- [x] Failed validation preserves form values in the browser

### Phase 2 verification log

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test`: passed; 9 unit tests and 10 integration tests.
- `npm run typecheck`: passed.
- `npm run build`: passed; 593 modules; `Contact` chunk generated.
- Integration tests verified shared props, field-level validation errors, success redirects and one-time flash data, `AppError` mappings, and protocol redirect statuses (`POST` 302; `PUT`/`PATCH`/`DELETE` 303).
- Headless Chrome (DevTools Protocol), `Contact` form: invalid submission showed the correct field errors, kept the values, set `aria-invalid`, and kept the URL on `/contact`.
- Headless Chrome observed the disabled “Sending...” button during the request, then a successful redirect and flash. The form reset, the flash disappeared after navigation and reload, and it was absent from `history.state`.
- Headless Chrome reported 2 `POST /contact` requests, 0 console errors, 0 uncaught exceptions, 0 failed requests, and 0 HTTP responses at or above 400.
- Server and Vite processes were stopped and `public/hot` was removed after verification.

## Phase 3 — Persistence

- [x] PostgreSQL connection configuration (`DATABASE_URL`, `TEST_DATABASE_URL`)
- [x] SQLx pool, startup migrations, and documented migration workflow
- [x] Contact messages persisted to and listed from PostgreSQL
- [x] Session-scoped listing prevents exposing other visitors' records
- [x] Parameterized SQL with a query-serving index
- [x] Database integration tests against an isolated test database
- [x] Database errors mapped to generic `500` responses without leaking details

### Phase 3 verification log

- Local PostgreSQL 17.4 (Laragon) is reachable on `127.0.0.1:5432`; local `pg_hba.conf` uses trust authentication.
- Created the `rust_inertia_app` role plus the `rust_inertia_app` and `rust_inertia_app_test` databases.
- Added `sqlx` 0.8.6 with only `runtime-tokio`, `postgres`, `migrate`, and `macros`; no TLS feature is enabled for local PostgreSQL.
- Chose SQLx runtime query APIs (`query_as`) so builds and CI do not require a live database; startup still runs embedded migrations.
- `migrations/20261105000000_create_contact_messages.sql` applied successfully to both databases and created `contact_messages` plus `contact_messages_session_created_idx`.
- Contact submissions insert one row per valid POST. The Contact page lists the current session's 20 most recent rows, newest first.
- SQL uses bind parameters throughout; multi-step writes are not present, so no explicit transaction is currently required.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test`: passed; 10 unit tests and 13 integration tests, including DB-backed creation, listing, session isolation, flash behavior, SQL-injection-as-data, and a generic 500 for database failures.
- `npm run typecheck` and `npm run build`: passed; 593 modules.
- Runtime check: the server started against the development database, POST `/contact` persisted a row, and a follow-up Inertia GET returned the saved message plus the one-time success flash.
- Runtime verification rows were deleted and the test database was truncated after the checks.

## Phase 4 — Authentication

- [ ] Persistent session store
- [ ] Registration/login as required
- [ ] Logout and session invalidation
- [ ] CSRF strategy verified
- [ ] Authorization
- [ ] Secure production cookies

## Phase 5 — Application feature

- [ ] List
- [ ] Create
- [ ] Show/edit
- [ ] Update
- [ ] Delete
- [ ] Validation and authorization tests

## Phase 6 — Quality

- [ ] Rust formatting
- [ ] Clippy
- [ ] Rust tests
- [ ] TypeScript checks
- [ ] Frontend production build

## Phase 7 — Production

- [ ] Production configuration
- [ ] Production assets verified
- [ ] Logging and health checks
- [ ] Deployment documented
- [ ] CI checks configured
