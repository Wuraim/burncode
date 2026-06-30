# Agent notes for Burncode

Burncode is a Tauri v2 desktop client that wraps a local opencode HTTP server. It does **not** run its own LLM or replace opencode.

## Stack & layout
- Frontend: Vue 3 + TypeScript + Pinia, rooted at `/src/`, built with Vite.
- Backend: Rust in `/src-tauri/src/`, exposed through Tauri commands.
- Tauri config: `src-tauri/tauri.conf.json`.
- The app owns the UI, routing policy, telemetry aggregation, and local settings. Opencode owns sessions, messages, todos, diffs, provider auth, and project state.

## Commands you will actually use
```bash
# Start the desktop app in dev mode (spins up Vite on :1420 + cargo run)
npm run tauri dev

# Build the production app (frontend -> Rust bundle)
npm run tauri build

# Frontend typecheck only
npx vue-tsc --noEmit

# Rust check only
cd src-tauri && cargo check
```

There are no tests, linters, or formatters configured yet. The gate is `vue-tsc --noEmit` plus `cargo check`.

## Runtime requirement
You need a local opencode server running. Example:
```bash
opencode serve --port 4096
# with basic auth:
OPENCODE_SERVER_PASSWORD=secret opencode serve --port 4096
```
Burncode connects at `http://127.0.0.1:4096` by default.

## Architecture gotchas
- Rust `src-tauri/src/opencode/` is the only HTTP/SSE client. Frontend never calls opencode directly.
- `/event` is consumed as a Server-Sent Events stream in a spawned tokio task and re-emitted to the frontend as Tauri events (`opencode-event`).
- `src-tauri/src/router/policy.rs` classifies prompt difficulty and picks a connected `providerID/modelID` before sending the request to opencode.
- Settings are stored in the Tauri app config dir as `settings.json`; only one active server profile is supported in v1.
- Provider credentials are **not** stored by Burncode. The UI discovers auth methods via `GET /provider/auth` and submits them via `PUT /auth/:id`; opencode stores the actual keys.

## Important opencode shapes
The frontend assumes these fields from the real opencode API:
- `Project`: use `worktree` for the display name.
- `Session`: use `title`, then `directory`, then `id`.
- `Todo`: text is `content`; done means `status === "completed"`.
- `FileDiff`: fields are `file`, `before`, `after`, `additions`, `deletions`.
- `Provider`: fields are `id` and `name`.
- `ProviderAuthMethod`: fields are `type` and `label`.

If opencode responses change, these are the first places to update.

## Style & performance notes
- Visual theme is "exhaust flame": dark graphite base, blue-violet core, orange peak accents. Keep code/diff areas calmer than activity/telemetry layers.
- Token streams can fire many SSE events. Avoid heavy per-event re-renders or unthrottled animation in the activity timeline.
- No snapshot tests or screenshot tooling exist; verify UI changes by running `npm run tauri dev`.
