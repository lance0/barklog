# Bark Roadmap

## Milestone 1: Minimal Viable TUI ✅

- [x] Project scaffolding (Cargo.toml, git, module structure)
- [x] Test log generator script with ANSI colors
- [x] Core data structures (LogLine, AppState, ring buffer)
- [x] File source implementation (tail -F)
- [x] Docker source implementation (docker logs -f)
- [x] Basic UI rendering (log view, header, status bar)
- [x] Main event loop with async coordination
- [x] Navigation (j/k, arrows, g/G, PageUp/Down)
- [x] Smart follow mode (auto-scroll, manual scroll breaks follow)
- [x] ANSI color preservation via ansi-to-tui

## Milestone 2: Filtering ✅

- [x] Integrate tui-textarea for filter input
- [x] Filter debounce (150ms delay before recompute)
- [x] Regex filter toggle (r key in normal mode, Ctrl+r in filter mode)
- [x] Live filter preview while typing (debounced)

## Milestone 3: Complete v1 ✅

- [x] Side panel with source list (toggle with `b`)
- [x] Panel focus cycling (`Tab` key)
- [x] Saved filters (save with `s`, apply with `Enter`, delete with `x`)
- [x] Filter quick-select from saved list
- [x] Help overlay (`?` key)
- [ ] Config file support (~/.config/bark/config.toml) - deferred to future
- [ ] Multiple source switching at runtime - deferred to future

## Quick Wins ✅

- [x] Log level detection and coloring (`c` to toggle)
- [x] Line wrapping toggle (`w` to toggle)
- [x] Mouse wheel scrolling

## Future Ideas

- [ ] Timestamp parsing and relative time display
- [ ] Bookmarks for specific log lines
- [ ] Search within current view (n/N for next/prev match)
- [ ] Export visible/filtered lines to file
- [ ] Horizontal scrolling for long lines
- [ ] JSON log pretty-printing
- [ ] Kubernetes pod log source
- [ ] SSH remote file tailing
- [ ] Multiple panes (split view)
- [ ] Config file support (~/.config/bark/config.toml)
- [ ] Multiple source switching at runtime
