<h1 align="center">🔥 Burncode</h1>

<p align="center">
  <img
    src="https://github.com/user-attachments/assets/b5fd6b34-6338-4f3f-be34-c8cc4687723e"
    alt="Burncode logo"
    width="700"
  />
</p>

<p align="center">
  <strong>A tactical desktop client for local opencode servers.</strong><br>
  Built with Tauri v2, Rust, and Vue 3.
  Token in → exhaust out. Prompts ignite, models fire, tokens burn.
</p>

## Features

- Connect to a local opencode HTTP server with optional basic auth
- Session workspace with real-time token-by-token streaming
- Smart autorouting — picks the best available provider/model based on
  task difficulty, historical success, cost, and remaining quota
- Agent mode selector (plan, build, and all opencode agents)
- Live SSE event pipeline showing model activity as it happens
- Read-only diff inspector for session changes
- Telemetry panel with most-used model tracking and burn animation
- Provider connect/disconnect with API-key-based auth flow
- Provider remaining-request capacity chart
- Session switcher, rename, delete, and fork at any message
- Command palette (`Ctrl+K`) for slash commands
- Route config panel showing scoring and routing decisions

## Requirements

- [Node.js](https://nodejs.org/) 22+
- [Rust](https://www.rust-lang.org/) toolchain
- A running local opencode server:

  ```bash
  opencode serve --port 4096

  # with auth
  OPENCODE_SERVER_PASSWORD=secret opencode serve --port 4096
  ```

## Quick Start

```bash
# Install frontend dependencies
npm install

# Start development mode (Vite on :1420 + Rust backend)
npm run tauri dev

# Build for production
npm run tauri build
```

On first launch, enter your opencode server URL (defaults to `http://127.0.0.1:4096`) and optional credentials. Auto-connect can be enabled in the connection screen.

## Development

```bash
# Frontend typecheck only
npx vue-tsc --noEmit

# Rust check only
cd src-tauri && cargo check

# Combined build check
npm run build
```

There are no tests or linters configured yet. The gate is `vue-tsc --noEmit` + `cargo check`.

## Architecture

```
frontend/       Vue 3 + TypeScript + Pinia (Vite dev server :1420)
src-tauri/      Rust backend exposed via Tauri commands
  opencode/       HTTP/SSE client — only code that talks to opencode
  router/         Model routing engine (difficulty classification, scoring)
  store/          Local settings + persistent routing analytics
  telemetry/      Usage aggregation
  commands/       Tauri command handlers
```

- Burncode owns the UI, routing policy, telemetry, and local settings.
- Opencode owns sessions, messages, todos, diffs, provider auth, and project state.
- SSE events from `/event` are consumed in a tokio task and re-emitted as Tauri events.
- Provider credentials are stored by opencode via `/auth/:id` — Burncode never stores API keys.

## Visual identity

"Exhaust flame" palette:

- Matte black / graphite base
- Electric blue core (idle / data)
- Violet → magenta transition (processing / routing)
- Orange flare peaks (token burn / model activity)

Keep code and diff views cooler than telemetry and live-activity layers.

## License

MIT
