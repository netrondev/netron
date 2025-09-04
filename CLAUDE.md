# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Netron is a full-stack Rust application built with:
- **Leptos** - Full-stack reactive web framework (using a custom fork with CSS hydration)
- **Tauri** - Desktop application framework
- **Axum** - Backend web server
- **SurrealDB** - Database (runs locally with surrealkv)
- **Iroh** - P2P networking capabilities
- **TailwindCSS** - Styling

The project uses a Rust workspace structure with multiple crates that compile to both WebAssembly (client) and native (server/desktop) targets.

## Development Commands

### Database Setup
```bash
# Start SurrealDB (required before running the app)
surreal start --user root --pass root --bind 0.0.0.0:8100 surrealkv://dbdata
```

### Web Development
```bash
# Run both the Leptos server and watch for changes (two terminals)
cargo leptos watch  # Terminal 1: Builds and watches frontend (uses fast-dev profile by default)
cargo leptos serve  # Terminal 2: Runs the server at http://localhost:8000

# Note: The fast-dev profile is configured by default in Cargo.toml for rapid iteration
# To use standard dev profile (slower but more optimized), comment out lib-profile and bin-profile in Cargo.toml
```

### Desktop Development (Tauri)
```bash
# Run the desktop app with hot reload (uses fast compilation settings via .cargo/config.toml)
cargo tauri dev

# Alternative: explicitly use fast-dev profile
CARGO_PROFILE=fast-dev cargo tauri dev
```

### Build Commands
```bash
# Build for production (web)
cargo leptos build

# Build desktop app
cargo tauri build

# Clean build artifacts
cargo clean
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p app
cargo test -p backend
cargo test -p server
```

## Architecture

### Workspace Structure
- **`app/`** - Core application logic, UI components, and business logic
  - Features: `ssr` (server-side rendering), `hydrate` (client-side hydration), `csr` (client-side rendering)
  - Contains auth, chat, P2P, database modules, and UI components
- **`frontend/`** - WebAssembly entry point for browser client
- **`backend/`** - Shared backend utilities and fallback handlers
- **`server/`** - Axum web server that serves the Leptos app
- **`src-tauri/`** - Tauri desktop application wrapper

### Key Architectural Patterns

1. **Feature-Based Compilation**: The app uses Rust features to compile different code for server (`ssr`) vs client (`hydrate`) contexts
   - Server-side code has access to database, file system, etc.
   - Client-side code runs in WASM with browser APIs

2. **Component Structure**: UI components in `app/src/components/` follow a consistent pattern
   - Each component is a Leptos function component with reactive signals
   - Components use TailwindCSS classes with dark mode support via `dark:` prefix
   - Theme state is managed by `ThemeProvider` component

3. **Database Access**: SurrealDB connections are managed in `app/src/db/`
   - Database operations are server-side only (gated by `#[cfg(feature = "ssr")]`)
   - Client-server communication happens via Leptos server functions

4. **P2P Networking**: Iroh integration in `app/src/p2p/` provides decentralized features
   - Gossip protocol for message propagation
   - Direct peer-to-peer connections

5. **Routing**: Leptos Router handles both client and server routing
   - Routes defined in `app/src/lib.rs` App component
   - File-based fallback handled by `backend/src/fallback.rs`

### Important Dependencies

The project uses a custom Leptos fork (`github.com/rvdende/leptos` branch `css_hydration`) for CSS hydration improvements. This is configured in the workspace `Cargo.toml` patch section.

## Code Conventions

- Use existing UI components from `app/src/components/` when building features
- Follow the reactive signal pattern with `RwSignal` for state management
- Server functions should be defined with `#[server]` attribute
- Database types that need to work on both client and server use conditional compilation (see `app/src/lib.rs` lines 34-39)
- Dark mode is handled via TailwindCSS classes with `dark:` prefix
- Components should support theme transitions with `transition-colors` class
- the build failed, after code changes double check with "cargo leptos test"