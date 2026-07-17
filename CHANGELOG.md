# Changelog

## [v0.2.0] - 2026-07-17

### Added

- IP-based fallback matching: match proxy by resolved IP when domain match fails
- Multi-profile support: multiple profiles can match a single proxy (GROUP_CONCAT)
- Retry logic (3 attempts with backoff) for loading profiles/groups from managers
- Filter chaining between Group and Manager dropdowns (mutually filtering)
- React Context API for grid data sharing between components
- Default URL fallback functions for profile manager settings

### Changed

- Database schema: removed profiles_cache/groups_cache tables, reworked match_results table
- Grid data flow: refactored from ag-Grid context to React Context (GridDataContext)
- Manager labels shortened: GPM Standard → GPM, GPM Global → GPM-G, Donut Browser → Donut
- Settings URL validation: guard against empty URLs when building clients
- Logging: added stdout logger alongside file logger, default level changed to debug

### Fixed

- Filter dropdowns now correctly cross-filter: selecting a group limits manager options and vice versa
- Empty URL values in settings no longer cause profile manager registration

## [v0.1.1] - 2026-07-15

### Added

- Single-instance mechanism: only one app window allowed (show/focus on second launch)
- Silent update support with progress tracking
- Unmatched proxies display mode toggle in settings
- Update settings UI (auto-update toggle, check now button)

### Changed

- Sync engine: save sync timestamp per-manager, skip if unchanged
- Proxy grid: improved unmatched proxy handling with configurable visibility
- Bump dependencies: tauri-plugin-single-instance, additional crate updates

### Fixed

- Tauri plugin initialization ordering for single-instance and update features

## [v0.1.0] - 2026-07-15

### Added

- Initial release of MKVN Proxy Manager
- Sync proxy orders from MKVN API
- Fetch browser profiles from GPM (Standard & Global) and Donut
- Match proxies to profiles by host:port
- AG-Grid based proxy table with filtering, search, and context menu
- Buy, renew, and toggle auto-renewal for proxy orders
- Settings dialog with profile manager configuration
- Dark/Light theme support
- Real-time sync progress via Tauri events
