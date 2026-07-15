# Changelog

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
