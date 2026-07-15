# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
npm run dev           # Vite dev server (frontend only, port 1420)
npm run tauri dev     # Full Tauri dev (frontend + Rust backend)
npm run build         # tsc check + Vite production build
npm run tauri build   # Production Tauri bundle
npm run preview       # Vite preview server
cargo test            # Run Rust unit tests (in src-tauri/)
```

- Rust tests: `cd src-tauri && cargo test`
- Single Rust test: `cd src-tauri && cargo test test_name`

## Tech Stack

- **Desktop shell**: Tauri 2.0 (Rust backend, webview frontend)
- **Frontend**: React 18 + TypeScript + Vite
- **UI**: Ant Design + AG-Grid Community (ag-grid-react)
- **Backend lang**: Rust (edition 2021)
- **HTTP**: reqwest (rustls-tls)
- **DB**: SQLite via rusqlite (bundled, WAL mode)
- **Logging**: tracing + tracing-subscriber + tracing-appender (daily rolling files)
- **Async**: tokio (full features)

## Architecture

Tauri app with split frontend/backend communicating via `invoke` (JSON-RPC).

### Backend (Rust) — `src-tauri/src/`

**Entrypoint:**
- `main.rs` → `lib.rs::run()` → Tauri builder setup

**State** (`state.rs`):
- `AppState`: holds `Arc<Database>`, `reqwest::Client`, `RateLimiter`, `AppSettings`, `MkvnClient`, and `ManagerRegistry`
- Created at startup with DB at `app_data_dir/proxies.db`
- `rebuild_clients()` reconfigures MKVN client and profile managers based on settings

**Commands** (`commands/`): Tauri `#[tauri::command]` handlers — thin wrappers that delegate to services:
- `sync_all`, `get_proxy_rows`, `get_cached_rows` — sync/data retrieval
- `buy_proxy`, `renew_order`, `toggle_renewal` — proxy operations
- `get_products`, `get_balance`, `get_settings`, `save_settings`

**Sync** (`sync/engine.rs`): 3-phase pipeline:
1. `sync_managers` — parallel fetch of profiles/groups from each enabled profile manager (concurrent via `join_all`)
2. `sync_orders` — fetch active MKVN orders, fetch proxy details for uncached orders (rate-limited, retry 5×, semaphore=8)
3. `match_cached` — build `ProxyProfileMatcher` from cached profiles, match each proxy by host:port, save `match_results`

**Profile Managers** (`managers/`):
- `ProfileManager` trait: `load_profiles()` + `load_groups()` → `Vec<UnifiedProfile/Group>`
- 3 impls: `GpmStandardManager`, `GpmGlobalManager`, `DonutManager` — each wraps a local HTTP API client
- Registered into `ManagerRegistry` at startup based on settings

**API Clients** (`api/`):
- `mkvn.rs` — calls `proxy.mkvn.net/api/apiv1` endpoints (GET/POST with token auth)
- `gpm_standard.rs` — hits `http://127.0.0.1:19995/api/v3` (paginated)
- `gpm_global.rs` — hits `http://127.0.0.1:9495/api/v1` (paginated)
- `donut.rs` — hits `http://127.0.0.1:10108/v1`
- `rate_limiter.rs` — token-bucket rate limiter (15 tokens / 10s window)

**Matcher** (`matcher/`):
- `extract_host_port` — parse proxy strings (URL or `host:port` format) via regex
- `ProxyProfileMatcher` — builds `HashMap<"host:port", Vec<ProfileMatch>>` from cached profiles; does simple first-match on lookup

**Database** (`database/`): SQLite via rusqlite with WAL journal. Tables: `orders`, `proxies`, `profiles_cache`, `groups_cache`, `match_results`, `products_cache`, `settings`, `app_state`. All writes in transactions.

**Events** (`events.rs`): Tauri event emitters for real-time frontend updates: `sync_started`, `sync_progress`, `sync_completed`, `database_updated`, `settings_changed`.

**Models** (`models/`): Shared structs annotated with serde Serialize/Deserialize — `ProxyRow`, `MkvnOrder`, `ProxyDetail`, `UnifiedProfile`, `UnifiedGroup`, `AppSettings`, `SyncResult`, etc.

### Frontend (React) — `src/`

- `main.tsx` — React entry, renders `<App>`
- `App.tsx` — root component, manages all state, keyboard shortcuts (Ctrl+R sync, Ctrl+B buy, Ctrl+, settings)
- `hooks/useSync.ts` — triggers full sync via `commands.syncAll`, listens to Tauri events for live progress
- `hooks/useSettings.ts` — loads/saves AppSettings, applies theme via `document.documentElement.dataset.theme`
- `lib/commands.ts` — typed wrappers around `@tauri-apps/api/core::invoke`
- `lib/gridConfig.ts` — AG-Grid column defs, row class rules (renewal-on, expiring-soon, unmatched), grid options
- `components/ProxyGrid.tsx` — AG-Grid with quick filter, selection, context menu
- `components/Toolbar.tsx` — Reload/Buy/Copy/Renew/Ren ON-Ren OFF buttons + search
- `components/GroupHeader.tsx` — AG-Grid header with dropdown filter for Group/Manager columns
- `dialogs/` — BuyProxyDialog, RenewDialog, RenewalToggleDialog, SettingsDialog
- `types/index.ts` — shared TypeScript interfaces matching Rust models

### Data Flow

1. User clicks "Reload" → `useSync.triggerSync()` → `commands.syncAll()` → Rust `SyncEngine::sync_all()`
2. Rust fetches profiles/groups from each manager (parallel), fetches MKVN orders + proxy details (rate-limited), matches proxies to profiles by host:port
3. Results stored in SQLite, Tauri event `sync_completed` emitted
4. Frontend re-reads rows via `get_proxy_rows`, displays in AG-Grid
5. Row styling: green left-border = renewal ON, red = ≤3 days remaining, amber = unmatched

### Key Design Notes

- Profile manager APIs (GPM, Donut) run as local HTTP services on the user's machine
- MKVN API is the proxy vendor (proxy.mkvn.net)
- Settings are persisted as a JSON blob in the `settings` table
- Auto-sync interval configurable (default 300s, not yet wired to a timer in frontend)
- Proxy matching is local-only: compares host:port from MKVN proxy lists against browser profile proxy settings
- No authentication, no multi-user, no cloud sync

## Release Workflow

When user asks to deploy/release/bump version, follow the workflow in `skills/mkvn-proxy-deploy/SKILL.md`. It covers the full pipeline: bump version in all files → update CHANGELOG → commit → tag → push → verify CI release.

### Version files
- `package.json` — `"version": "<current>"`
- `package-lock.json` — `"version": "<current>"` (2×)
- `src-tauri/Cargo.toml` — `version = "<current>"`
- `src-tauri/tauri.conf.json` — `"version": "<current>"`
- `CHANGELOG.md` — `## [v<current>]`

### CI release
`.github/workflows/release.yml` auto-creates GitHub release when tag `v*` is pushed (builds MSI/EXE/DMG/AppImage for all platforms). Do NOT create release locally — push tag and let CI handle it.
