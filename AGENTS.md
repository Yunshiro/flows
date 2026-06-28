# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
npm run dev              # Browser-only dev (Vite, localStorage fallback)
npm run tauri dev         # Desktop dev (Tauri + SQLite + full features)
npm run build             # Type-check + bundle frontend
npm run tauri build       # Production build → .msi/.dmg in src-tauri/target/release/bundle/
npx vue-tsc --noEmit     # Frontend type-check only
cd src-tauri && cargo check  # Rust type-check only
```

## Architecture

Tauri v2 desktop app with Vue 3 frontend and Rust backend. SQLite for structured data, filesystem for Markdown notes.

**Dual-mode adapter** (`src/api/adapter.ts`): All Tauri `invoke` calls go through `tauriInvoke()` which detects `window.__TAURI_INTERNALS__` at runtime. When true → real Tauri IPC. When false (browser dev) → localStorage fallback implementing the same CRUD operations. This means `npm run dev` works standalone without the Rust backend.

## Serde / Tauri Naming Convention

**All Rust model structs** must have `#[serde(rename_all = "camelCase")]`. **All `#[tauri::command]`** attributes must use `rename_all = "camelCase"`. Tauri v2 does NOT auto-convert between JS camelCase and Rust snake_case — without these, invoke parameters silently fail to match.

JS → Tauri → Rust flow: `{ estMinutes: 30 }` → `invoke('add_todo', args)` → `#[tauri::command(rename_all = "camelCase")]` maps `estMinutes` → `est_minutes` Rust param.

## Data Storage

| Data | Path | Format |
|---|---|---|
| Todos, Reviews, Settings, LLM Configs | `~/.flows/flows.db` | SQLite (rusqlite, WAL mode) |
| Notes | `~/flows-notes/` | Raw `.md` files on filesystem |
| Browser fallback | `localStorage` | `flows_*` keys |

SQLite schema is auto-migrated in `src-tauri/src/db.rs`. Tables: `todos`, `daily_reviews`, `settings`, `llm_configs`.

## Streaming LLM Architecture

Weekly summary generation uses Tauri's event system for real-time output:

1. Frontend calls `generate_weekly_summary_stream` with optional `configId`
2. Rust reads config from `llm_configs` table (or falls back to `settings` JSON)
3. Command spawns `tokio::spawn` task, returns `"started"` immediately
4. Task makes streaming HTTP POST to OpenAI-compatible `/chat/completions` endpoint
5. Each SSE `data:` chunk parses `choices[0].delta.content`, emits `llm-chunk` event via `app.emit()`
6. Frontend listens via `listen('llm-chunk', ...)` from `@tauri-apps/api/event`
7. `reasoning_content` chunks (DeepSeek R1 thinking) are filtered out server-side

## Key Rust Conventions

- `tauri::State<'_, Database>` for injecting SQLite connection into commands
- `Mutex<Connection>` for thread-safe DB access (not async, blocking is fine for desktop)
- Flat command parameters (e.g. `title: String, priority: Option<String>`) rather than request structs — avoids Tauri invoke deserialization ambiguity
- `chrono::Local::now().format("%Y-%m-%d")` for date keys, `%Y-%m-%dT%H:%M:%S` for timestamps
- UUID v4 for all primary keys

## Frontend State Pattern

Pinia stores (`src/stores/`) import from `src/api/` which calls `tauriInvoke()` from `src/api/adapter.ts`. Stores never import `@tauri-apps/api/core` directly — always through the API layer. Each store method wraps calls in try-catch with `console.error` logging.
