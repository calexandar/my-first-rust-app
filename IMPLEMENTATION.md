# Rust + React + Inertia.js — Implementation Guide for OpenCode

> **Purpose:** Turn the existing local Cargo application into a maintainable full-stack web application using Rust, Axum, Inertia.js v3, React, TypeScript, Vite, and Tailwind CSS.
>
> **Working mode:** Implement this document phase by phase. Do not attempt the entire application in one pass.

---

## 0. Instructions for the coding agent

You are working inside an **existing local Cargo project**. Your first responsibility is to understand the repository as it exists today, not to replace it with a generated starter.

### Non-negotiable rules

1. **Inspect before changing anything.**
   - Read `Cargo.toml`, `Cargo.lock`, all existing Rust source files, frontend files, configuration, tests, migrations, and README files.
   - Run `git status --short` and inspect the current diff before editing.
   - Identify the existing application entry point, router, state management, error handling, and project conventions.
   - Do not overwrite or delete working code simply to match the example structure in this document.

2. **Verify the Inertia Rust adapter before integrating it.**
   - Repository: <https://github.com/inertiajs/inertia-laravel-rust>
   - Read its current README, `Cargo.toml`, examples/demo application, tests, and relevant source code.
   - Treat the repository's current code and examples as authoritative. Do not rely on remembered APIs or invent crate methods.
   - The adapter is marked **ALPHA**. Record this as a project risk and verify the exact current feature set and compatibility.
   - Its README currently describes a local path dependency/workspace setup. Determine the correct dependency arrangement from the checked-out repository. Do not assume the crate is published on crates.io.
   - Inspect the demo's Vite integration. The repository also documents a separate `laravel-vite-rust` helper; verify whether it is required and how the demo uses it before adding it.

3. **Make small, reviewable changes.**
   - Work on one phase at a time.
   - Before each phase, explain the intended changes and files affected.
   - After each phase, run relevant formatting, linting, type-checking, builds, and tests.
   - Fix regressions before continuing.
   - Do not silently skip failing checks. Explain failures and distinguish existing failures from newly introduced ones.
   - Keep `Cargo.lock` committed for this application.

4. **Preserve project conventions.**
   - Reuse existing naming, module layout, error types, configuration, and test patterns where sensible.
   - Do not introduce a framework, ORM, state-management library, or abstraction without a concrete need.
   - Avoid unnecessary dependencies.
   - Prefer explicit, strongly typed Rust code and clear TypeScript types.
   - Do not use `unwrap()` or `expect()` in normal request-handling paths. Handle errors deliberately. They may be acceptable in tests or truly invariant setup code with justification.

5. **Security and secrets.**
   - Never commit secrets, real credentials, private keys, or populated production `.env` files.
   - Add or update `.env.example` with safe placeholders only.
   - Validate and authorize every state-changing operation on the server.
   - Never trust user IDs, roles, permissions, prices, or ownership values supplied by the browser.
   - Use secure cookie settings in production and document local-development differences.

6. **Keep documentation current.**
   - Maintain the root README with setup, development, test, and build commands.
   - Maintain a concise `IMPLEMENTATION_PROGRESS.md` checklist with completed work, decisions, known issues, and the next task.
   - Do not mark a phase complete until its acceptance criteria pass.

### First response required from the agent

Before editing files, provide:
- A concise inventory of the current project.
- The current Git working-tree status.
- Existing Rust and frontend tooling discovered.
- The Inertia adapter integration approach supported by the current repository.
- Any blockers or decisions that require user input.
- A proposed phase plan adjusted to the actual repository.

Then wait for approval before making broad or destructive changes. Small, clearly safe inspection commands are allowed immediately.

---

## 1. Target architecture

Use a **single web application**. Rust owns routing, request handling, business rules, persistence, authentication, authorization, and server-side validation. React owns the interactive user interface. Inertia connects the two without requiring a separate REST API for ordinary page navigation.

### Request lifecycle

1. The browser requests a URL.
2. Axum routes the request to a Rust handler.
3. The handler reads application state, invokes application logic, and returns an Inertia response with a page component and serialized props.
4. On a first visit, the server returns the root HTML document with the Inertia page payload.
5. On an Inertia visit, the adapter returns the protocol response expected by the Inertia client.
6. React resolves and renders the requested page component.
7. Forms submit to Rust routes through Inertia. Rust validates and authorizes the request, then returns errors or a redirect/flash response as appropriate.

Do not build a second client-side router for normal application pages.

### Preferred stack

Use these choices unless the existing project has a sound, documented alternative:

| Concern | Preferred choice |
|---|---|
| Backend language | Rust, stable toolchain |
| HTTP framework | Axum, compatible version verified against adapter |
| Async runtime | Tokio |
| Server/client bridge | `inertia-laravel-rust` adapter |
| Frontend | React |
| Frontend language | TypeScript |
| Inertia client | `@inertiajs/react`, version compatible with server protocol |
| Bundler | Vite |
| CSS | Tailwind CSS 4 |
| Database | PostgreSQL, if persistence is needed |
| Database access | SQLx with compile-time/runtime query strategy chosen deliberately |
| Serialization | Serde |
| Sessions | Adapter-supported `tower-sessions` integration, with a persistent store for production |
| Validation | Adapter-supported `validator` or `garde` integration, or a small explicit validation layer |
| Password hashing | A maintained password-hashing crate such as Argon2 implementation; verify current crate guidance |
| Logging/tracing | `tracing` and `tracing-subscriber` |
| Tests | Rust unit/integration tests, adapter's Inertia testing helpers, and frontend type/build checks |

Do not add database, authentication, or other optional infrastructure until the relevant phase. If the existing project already uses a different database or framework, assess it before changing it.

---

## 2. Suggested project organization

Adapt this structure to the current repository. It is a target direction, not a requirement to move every existing file.

```text
project/
├── Cargo.toml
├── Cargo.lock
├── .env.example
├── README.md
├── IMPLEMENTATION.md
├── IMPLEMENTATION_PROGRESS.md
├── package.json
├── package-lock.json
├── tsconfig.json
├── vite.config.ts
├── public/
│   └── ...
├── resources/
│   └── js/
│       ├── app.tsx
│       ├── pages/
│       │   ├── Home.tsx
│       │   └── Dashboard/
│       │       └── Index.tsx
│       ├── components/
│       ├── layouts/
│       ├── hooks/
│       └── types/
├── migrations/
└── src/
    ├── main.rs
    ├── app.rs
    ├── config/
    ├── errors/
    ├── infrastructure/
    │   ├── database/
    │   └── session/
    ├── middleware/
    └── features/
        ├── home/
        └── dashboard/
```

### Backend organization principles

- Keep `main.rs` focused on process startup and dependency wiring.
- Put router construction and middleware layering in `app.rs` or the existing equivalent.
- Organize application code by business feature, not only by technical type.
- Keep HTTP handlers thin: extract/validate input, call application logic, and translate the result into an HTTP/Inertia response.
- Keep database access out of React-facing serialization code.
- Use explicit request DTOs and response/page DTOs. Do not serialize database models wholesale if they contain internal or sensitive fields.
- Introduce command/query modules when there is meaningful business logic. Do not create empty abstractions for every CRUD endpoint.
- Keep infrastructure details (database pools, session stores, external clients) behind clear application boundaries.

### Frontend organization principles

- Page component names must match the component names returned by Rust, including case and directory structure.
- Put shared application shells in `layouts/`.
- Put reusable visual elements in `components/`.
- Define shared prop types where reuse justifies it.
- Keep server-provided props typed. Avoid `any`.
- Use Inertia navigation and form APIs for ordinary application navigation and submissions.

---

## 3. Phase 0 — Repository audit and integration proof

### Tasks

1. Inspect the existing Cargo project and Git state.
2. Check installed versions:
   - `rustc --version`
   - `cargo --version`
   - `node --version`
   - `npm --version` (or the package manager already used)
3. Inspect the Rust Inertia repository:
   - README and quick start.
   - `Cargo.toml` and feature flags.
   - `examples/` and demo app.
   - Axum adapter implementation.
   - Root view and Vite integration.
   - Session layer ordering.
   - Testing APIs.
4. Confirm whether the project should use a Cargo workspace with the adapter as a sibling path dependency, a Git dependency, or another arrangement supported by the repository. Prefer the documented and reproducible approach.
5. Confirm compatible versions of Axum, Tower, Tokio, React, `@inertiajs/react`, and the adapter.
6. Identify how static files and Vite development assets are currently served.
7. Write the findings and decisions to `IMPLEMENTATION_PROGRESS.md`.

### Deliverable

A short technical audit and a minimal integration plan. Do not restructure the application in this phase.

### Acceptance criteria

- Existing files and behavior are understood.
- No existing changes have been lost.
- Dependency source and versions are verified.
- Any adapter alpha limitations are documented.
- The user has approved the implementation plan.

---

## 4. Phase 1 — Backend/frontend foundation

### Backend tasks

1. Integrate Axum only if it is not already present and the adapter requires it.
2. Integrate the Inertia Rust crate using the verified dependency arrangement.
3. Configure the Inertia `Config` using the current adapter API.
4. Configure the root HTML view:
   - Valid HTML5 document.
   - Character encoding and viewport.
   - Inertia head content.
   - Inertia body/page payload.
   - Correct development and production asset tags.
5. Configure the Axum router and `InertiaLayer`.
6. Configure the session middleware in the correct layer order. Verify the adapter README; its quick start places the session layer outside the Inertia layer.
7. Configure graceful application startup and shutdown if the current app supports it.
8. Read bind address and other runtime settings from configuration/environment rather than hardcoding production values.
9. Add a simple `GET /health` endpoint that does not expose secrets or internal diagnostics.

### Frontend tasks

1. Add or adapt the React + TypeScript entry point.
2. Configure Vite and the React plugin.
3. Configure Tailwind CSS 4 using the current Vite integration.
4. Configure Inertia's React client using the current Inertia v3 client setup.
5. Implement page resolution using `import.meta.glob`.
6. Ensure page resolution fails clearly for a missing component rather than silently rendering an empty page.
7. Add a minimal shared layout and a simple Home page.
8. Configure TypeScript with strict checking where practical.
9. Confirm Vite development assets and production manifest paths match the Rust root view.

### Example frontend entry point

Use this as a reference only; adapt it to the current Inertia React API and demo application:

```tsx
import { createInertiaApp } from '@inertiajs/react';
import { createRoot } from 'react-dom/client';

const pages = import.meta.glob('./pages/**/*.tsx', { eager: true });

createInertiaApp({
    resolve: (name) => {
        const page = pages[`./pages/${name}.tsx`];

        if (!page) {
            throw new Error(`Page not found: ${name}`);
        }

        return page;
    },
    setup({ el, App, props }) {
        createRoot(el).render(<App {...props} />);
    },
});
```

### Example first page

Create a simple page that receives a typed `message` prop from Rust and displays it. Keep the example minimal.

### Acceptance criteria

- Rust server starts locally.
- Vite development server starts.
- Visiting `/` returns a real Inertia page.
- Initial browser load renders React.
- Inertia client-side navigation works between at least two pages.
- No browser console errors or missing asset requests.
- Production frontend build completes.
- Rust formatting and checks pass.

---

## 5. Phase 2 — Shared props, errors, redirects, and forms

Use the adapter's current documented API. Do not recreate protocol behavior that the adapter already provides.

### Backend tasks

1. Configure global shared props such as:
   - Application name.
   - Current environment-safe UI metadata.
   - Authenticated user only after authentication is implemented.
2. Add a consistent application error type and mapping to HTTP responses.
3. Demonstrate a server-side validation failure using the adapter's validation error API.
4. Demonstrate a successful form submission and redirect.
5. Demonstrate a flash message.
6. Verify redirect behavior after `POST`, `PUT`, `PATCH`, and `DELETE`.
7. Keep secrets and internal error details out of page props and user-facing errors.

### Frontend tasks

1. Build a reusable form example using the current `useForm` API from `@inertiajs/react`.
2. Display field-level validation errors.
3. Display a flash notification from the correct Inertia page field.
4. Add processing/disabled states and accessible labels.
5. Confirm failed validation preserves appropriate form values.
6. Confirm success redirects and shows the flash message.

### Acceptance criteria

- Validation errors appear on the correct fields.
- Successful submissions redirect correctly.
- Flash messages are shown once as intended.
- Browser history does not retain sensitive flash data.
- Server-side validation is authoritative; frontend validation is only a usability enhancement.

---

## 6. Phase 3 — Database and persistence

Do this phase only if the application needs persistent data.

### Tasks

1. Confirm PostgreSQL is the chosen database and document local setup.
2. Add SQLx with only the required features.
3. Configure the connection pool from environment variables.
4. Add migrations and a documented migration workflow.
5. Add a small example feature (for example, users or notes) with:
   - Database schema.
   - Query functions.
   - Request DTOs.
   - Response/page DTOs.
   - Handler functions.
6. Use transactions for multi-step writes where consistency requires them.
7. Use parameterized SQL. Never concatenate user input into SQL.
8. Add indexes and constraints based on actual query needs.
9. Avoid exposing password hashes, tokens, internal fields, or unrestricted database records to Inertia props.
10. Decide whether SQLx compile-time query checking or runtime query APIs fit the project's build/CI environment. Document the choice.

### Example environment template

```dotenv
APP_ENV=local
APP_HOST=127.0.0.1
APP_PORT=3000

DATABASE_URL=postgres://app:change-me@127.0.0.1:5432/app_db
```

Use placeholders only. Do not assume these credentials exist.

### Acceptance criteria

- Migrations apply to a clean local database.
- The sample feature can create and list records.
- Database failures are handled without leaking connection strings or secrets.
- Integration tests use an isolated test database or a documented test strategy.
- No SQL injection vulnerabilities are introduced.

---

## 7. Phase 4 — Authentication and authorization

Authentication is security-sensitive. Read the current session adapter documentation and current crate guidance before implementation.

### Tasks

1. Choose a session store:
   - In-memory storage is acceptable only for local demos/tests.
   - Production must use a persistent store appropriate for the deployment.
2. Implement registration only if the product requires it.
3. Implement login and logout.
4. Hash passwords using a maintained password-hashing algorithm and safe parameters.
5. Regenerate or rotate the session identifier after authentication to prevent session fixation.
6. Invalidate the session on logout.
7. Set cookie attributes deliberately:
   - `HttpOnly`.
   - `Secure` in HTTPS production.
   - Appropriate `SameSite`.
   - Appropriate path and lifetime.
8. Verify CSRF protection for all cookie-authenticated state-changing requests. Confirm how the Inertia adapter, Axum setup, and session stack handle it; add protection if needed.
9. Implement authorization on the server for protected resources and actions.
10. Share only the minimum current-user information required by the UI.
11. Clear or encrypt Inertia browser history where appropriate for sensitive authenticated pages, using supported adapter functionality.

### Acceptance criteria

- Guests cannot access protected pages or actions.
- Invalid credentials do not reveal whether an account exists.
- Passwords are never stored or logged in plaintext.
- Session rotation and logout invalidation work.
- Authorization is enforced in Rust, not just hidden in React.
- Production cookie settings are secure.
- CSRF strategy is tested and documented.

---

## 8. Phase 5 — Reusable application features

After the foundation is stable, build one feature end-to-end before generalizing.

Suggested first feature: **Users management** (only if appropriate for the application).

### Backend

- Index/list handler.
- Show handler.
- Create/store handlers.
- Edit/update handlers.
- Delete handler.
- Pagination and filtering if needed.
- Authorization checks.
- Validation.
- Typed Inertia props.
- Tests for success, validation failure, not found, and forbidden access.

### Frontend

- `Users/Index.tsx`
- `Users/Create.tsx`
- `Users/Edit.tsx`
- `Users/Show.tsx`
- Shared form component.
- Confirmation UI for destructive actions.
- Loading/processing states.
- Empty state.
- Pagination controls.

### Acceptance criteria

- CRUD works end-to-end.
- Inertia navigation works without full page reloads.
- Validation and flash feedback work.
- Unauthorized operations are rejected by the backend.
- Sensitive database fields are not exposed.
- Tests cover the important paths.

Do not build a generic CRUD generator or admin framework until at least one real feature demonstrates which abstractions are actually useful.

---

## 9. Phase 6 — Testing and quality

### Rust

Run and maintain:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test
```

Add Clippy where appropriate:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

If the adapter or project has feature combinations that make `--all-features` unsuitable, document and use the supported feature matrix instead.

Test:
- Route status codes.
- Inertia component names.
- Inertia props and shared props.
- Missing props and sensitive fields.
- Validation errors.
- Redirects and flash data.
- Authentication and authorization.
- Database behavior.

Use the adapter's `AssertablePage` helper where available and appropriate. Verify its exact API from the current crate.

### Frontend

Run the project's package-manager equivalents of:

```bash
npm run typecheck
npm run build
```

Add scripts if missing. Do not claim type-checking is performed by Vite unless it actually is.

Test:
- Page component resolution.
- Form validation display.
- Processing states.
- Navigation.
- Accessible labels and keyboard behavior.

### Quality rules

- No unexplained compiler warnings.
- No unhandled errors in normal user flows.
- No `any` types without a documented reason.
- No secrets in source control.
- No frontend-only authorization.
- No unbounded database queries for list pages.

---

## 10. Phase 7 — Production readiness

Do not enable optional complexity such as SSR until the normal client-rendered application is stable.

### Tasks

1. Build optimized frontend assets.
2. Verify Rust serves the correct production asset manifest and hashed assets.
3. Configure production environment variables and secret management.
4. Use a persistent session store.
5. Enable secure cookies and HTTPS.
6. Configure trusted proxy behavior only for the actual deployment topology.
7. Configure structured logs and request correlation where appropriate.
8. Avoid logging passwords, session cookies, authorization headers, or sensitive form content.
9. Add health checks and graceful shutdown.
10. Document database backup/migration practices.
11. Build a reproducible deployment artifact or container if the hosting environment requires it.
12. Add CI checks for formatting, Clippy, tests, frontend type-checking, and production builds.
13. Document rollback and migration considerations.

### SSR decision

SSR is optional. The adapter documents an HTTP SSR gateway and a Vite SSR bundle. Before implementing:
- Verify current adapter SSR requirements.
- Confirm the frontend SSR entry point and production bundle.
- Confirm the SSR process lifecycle and deployment strategy.
- Test fallback behavior if the SSR service is unavailable.
- Measure whether SSR is needed for SEO or first-render performance.

Do not add SSR solely because it is available.

### Acceptance criteria

- A clean production build succeeds.
- Production assets load correctly without the Vite dev server.
- Sessions and cookies are configured securely.
- Secrets are supplied outside source control.
- Health checks, logs, and graceful shutdown work.
- Deployment steps are documented and reproducible.

---

## 11. Optional Inertia capabilities

Consider these only when a real feature needs them. Verify current support in the adapter and React client before use.

- Deferred props for expensive secondary data.
- Optional/lazy props for partial reloads.
- Shared props for common UI data.
- Infinite scrolling.
- Flash data.
- Error bags for complex forms.
- History encryption for sensitive pages.
- SSR.
- Live props/broadcast-driven updates.

Do not implement unsupported Inertia features by guessing or by silently changing the protocol.

---

## 12. Development workflow

The agent should use this workflow for every phase:

1. Read this document and `IMPLEMENTATION_PROGRESS.md`.
2. Inspect the relevant existing files.
3. State the objective and list intended file changes.
4. Implement the smallest coherent change.
5. Run formatting, checks, tests, and builds relevant to the change.
6. Inspect the diff for accidental changes, secrets, generated files, and unrelated formatting.
7. Update `IMPLEMENTATION_PROGRESS.md`.
8. Report:
   - What changed.
   - Important implementation decisions.
   - Commands executed and their results.
   - Any known limitations.
   - The next phase.
9. Stop and ask for approval before starting the next phase.

### Do not

- Replace the project with a template.
- Rewrite the entire application in one operation.
- Guess adapter APIs.
- Invent crate versions or feature flags.
- Add both an ORM and SQLx without a clear reason.
- Add a REST API for every page simply out of habit.
- Put database access in React.
- put business rules in route handlers.
- Store authentication state in browser local storage as a substitute for secure server sessions.
- Disable CSRF or browser security protections to make a demo work.
- Claim a phase is complete when commands have not been run.

---

## 13. Initial progress checklist

Create `IMPLEMENTATION_PROGRESS.md` based on this checklist after the repository audit.

```markdown
# Implementation Progress

## Project audit
- [ ] Existing project inspected
- [ ] Git working tree inspected
- [ ] Rust and Node toolchains verified
- [ ] Inertia Rust adapter README and source reviewed
- [ ] Demo app and Vite integration reviewed
- [ ] Dependency and feature compatibility confirmed
- [ ] Implementation plan approved

## Phase 1 — Foundation
- [ ] Axum and Inertia integrated
- [ ] Root HTML view configured
- [ ] Session middleware order verified
- [ ] React + TypeScript configured
- [ ] Vite configured
- [ ] Tailwind CSS 4 configured
- [ ] First Inertia page renders
- [ ] Client-side navigation verified
- [ ] Production frontend build passes

## Phase 2 — Inertia workflows
- [ ] Shared props
- [ ] Validation errors
- [ ] Successful form submission
- [ ] Redirects
- [ ] Flash messages

## Phase 3 — Persistence
- [ ] Database configuration
- [ ] Migrations
- [ ] Example feature
- [ ] Database integration tests

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
```

---

## 14. Reference documentation

Use these sources and prefer current official documentation over third-party tutorials:

- Inertia Rust adapter: <https://github.com/inertiajs/inertia-laravel-rust>
- Adapter examples: <https://github.com/inertiajs/inertia-laravel-rust/tree/main/examples>
- Inertia documentation: <https://inertiajs.com/>
- Axum documentation: <https://docs.rs/axum/latest/axum/>
- Tokio documentation: <https://docs.rs/tokio/latest/tokio/>
- SQLx documentation: <https://docs.rs/sqlx/latest/sqlx/>
- Vite documentation: <https://vite.dev/>
- React documentation: <https://react.dev/>
- Tailwind CSS documentation: <https://tailwindcss.com/docs>

**Important:** The Inertia Rust adapter is alpha software. Re-check the repository's current status, API, feature flags, examples, and known issues before each integration milestone. Pin dependencies deliberately and avoid unreviewed upgrades during implementation.
