# Changelog

All notable changes to Bark will be documented in this file.

## [Unreleased]

### Added
- Initial project structure with modular architecture
- File log source using `tail -F`
- Docker log source using `docker logs -f`
- Ring buffer with configurable max lines (default 10,000, via `BARK_MAX_LINES`)
- ANSI color code preservation and rendering
- Smart follow mode (auto-scroll to bottom, manual scroll disables follow)
- Basic navigation: j/k, arrows, g/G, PageUp/PageDown, Ctrl+u/d
- Substring filtering with `/` key
- Filter clear with `Esc`
- Status bar showing mode, line counts, and active filter
- Scrollbar for large log buffers
- Test log generator script (`scripts/gen_logs.sh`)
- **tui-textarea** integration for filter input with full editing support
- Filter debounce (150ms) for live preview while typing
- Regex filter toggle (`r` in normal mode, `Ctrl+r` in filter mode)
- Status bar shows regex mode indicator `[.*]` when enabled
- Side panel with sources list and saved filters (toggle with `b`)
- Panel focus cycling with `Tab` key
- Save filters with `s`, apply with `Enter`, delete with `x`
- Help overlay showing all keyboard shortcuts (`?`)
- **Log level detection** - auto-colors ERROR (red), WARN (yellow), INFO (green), DEBUG (blue), TRACE (gray)
- **Line wrapping** - toggle with `w` key
- **Mouse wheel scrolling** - scroll up/down with mouse wheel

### Technical
- Async event loop using tokio
- ratatui-based TUI with crossterm backend
- Lazy ANSI-to-styled-text conversion with caching
- Panic hook for clean terminal restoration
- Debounced filter recomputation to avoid lag during typing
