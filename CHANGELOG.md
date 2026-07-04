# Changelog

All notable changes to Griddle, newest first.

## [1.0.5] — 2026-07-04

### Fixed
- Panel header now shows the real app version, sourced from the binary at compile time (`CARGO_PKG_VERSION`). Previously the `getVersion()` API failed silently in packaged builds and a stale hardcoded fallback ("v1.0.2") was displayed.

## [1.0.4] — 2026-07-04

### Fixed
- **Auto-handshake:** the Rive MCP server requires the `initialize` + `notifications/initialized` handshake after every editor restart; bare `tools/call` requests return `400 "Server not initialized"`. Any failed call now triggers the handshake and retries once, so the launch/restart order of Rive and Griddle no longer matters.
- **Startup reconnect loop:** if Rive isn't running (or has no file open) when Griddle starts, the panel retries every 3 seconds instead of failing once, and connects by itself as soon as a file is available.
- **Ghost-proof deletion:** "Remove last" now collects the full subtree of everything it created and deletes it deepest-first (vertices → paths → paints → shapes → container). The editor's `delete_objects` doesn't cascade reliably and could leave child paths orphaned in the file — invisible in the hierarchy but still selectable as "ghosts" via select-all.

## [1.0.3] — 2026-07-03

### Added
- **True φ proportions** toggle (default on) for the golden ratio grid: inscribes the largest golden rectangle, centered, with genuinely circular quarter-arcs. Unchecked restores the previous stretch-to-fill behavior (arcs become ellipse quarters aligned with the phi lines).

## [1.0.2] — 2026-07-03

### Fixed
- **Hexagonal grid:** row-boundary zigzag polylines no longer draw straight closing chords across the grid (Rive `PointsPath` objects default to closed), and the honeycomb now clips exactly to the grid bounds instead of overflowing.
- **True path lengths:** every segment path is authored open. Closed 2-point paths measure double their visual length (forward + closing chord), which broke dash effects and path metrics.
- **Golden spiral** no longer draws a closing chord from its end back to its start.
- **Subdivisions semantics:** "subdivisions = N" now divides each cell into N parts (N−1 minor lines). Previously it inserted N lines, producing N+1 parts.

### Added
- **Cell gutters** for rectangular grids: cells separated by a gutter, with a guide line on both edges of every gap.

### Changed
- Dot grid is snap crosses only (the dash-ready mode was removed — dash effects cannot be authored through the MCP).
- UI overhaul: brand header with the Griddle mark, live version and copyright; all sections open by default; cell size moved into Grid options; per-type progressive disclosure retained.

## [1.0.1] — 2026-07-03

### Fixed
- macOS bundles are now fully ad-hoc signed (`APPLE_SIGNING_IDENTITY="-"`) instead of carrying only the linker-generated signature stub. Gatekeeper previously classified the apps as "damaged" with no recourse; with a valid bundle signature the block becomes "unidentified developer," which offers **Open Anyway** in System Settings → Privacy & Security.

## [1.0.0] — 2026-07-03

Initial release.

- Floating always-on-top panel that authors **snappable grid guides** in the Rive editor via its MCP server — deterministic, no agent, no AI. Guides are real `PointsPath` geometry, so editor vertex snapping works on every line and lattice point.
- 14 grid types: rectangular (+subdivisions), dot, baseline, brick, isometric, triangular, diamond, hexagonal, polar, one-point and two-point perspective, golden ratio, rule of thirds, layout columns.
- Artboard picker with auto-fit to artboard size, custom shape naming, grouped separate styling for subdivisions, path-count caps with auto-adjusted spacing, and "Remove last."
- CI releases for macOS (Apple Silicon + Intel) and Windows (NSIS + MSI).
