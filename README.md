# rust-inertia-app

A full-stack application using Rust, Axum, Inertia.js v3, React, TypeScript, Vite, and Tailwind CSS 4.

Rust owns routing, request handling, and server-side logic. React owns the interactive UI. Inertia connects the two without a separate REST API or a client-side router for normal page navigation.

## Requirements

- Rust 1.99 or newer
- Node.js 20.19 or newer
- npm
- On Windows: the **Desktop development with C++** workload in Visual Studio, so the MSVC linker is available

The Inertia Rust adapter is alpha and is currently consumed from a pinned Git revision. The `.cargo/config.toml` file declares the adapter's Pilot registry so Cargo can parse its manifest; no Pilot token is required for the pinned Git dependency.

## Setup

```bash
npm install
```

Copy `.env.example` to `.env` if you want to override defaults. The app reads:

| Variable | Default |
|---|---|
| `APP_ENV` | `local` |
| `APP_HOST` | `127.0.0.1` |
| `APP_PORT` | `3000` |
| `APP_URL` | `http://127.0.0.1:3000` |

## Development

Run two terminals from the project root:

```bash
npm run dev     # Vite on http://localhost:5173
cargo run       # Rust server on http://127.0.0.1:3000
```

Open <http://127.0.0.1:3000>. Vite writes `public/hot` while it runs; the Rust root view points asset tags at the dev server.

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

## Project layout

```text
src/
├── main.rs          process startup and graceful shutdown
├── lib.rs           router, middleware, Inertia configuration
├── config.rs        environment-based runtime configuration
├── vite.rs          Vite hot file and production manifest asset tags
└── features/        one module per page or business feature
resources/
├── css/app.css      Tailwind entry point
└── js/
    ├── app.tsx      Inertia client setup
    ├── layouts/     shared application shell
    └── pages/       one React component per Inertia page
tests/app.rs         protocol-level tests with AssertablePage
```

Server-side rendering is intentionally deferred until the client-rendered application is stable.
