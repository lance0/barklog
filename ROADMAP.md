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
- [x] Config file support (~/.config/bark/config.toml)

## Milestone 4: Enhanced Features ✅

- [x] Search highlighting (matched text highlighted in yellow)
- [x] Match navigation (n/N for next/prev match)
- [x] Horizontal scrolling (h/l, H/L, 0 for line start)
- [x] Export filtered lines to file (e key)
- [x] Log level detection and coloring (c to toggle)
- [x] Line wrapping toggle (w to toggle)
- [x] Mouse wheel scrolling
- [x] Timestamp parsing and relative time display (t to toggle)
- [x] JSON log pretty-printing (J to toggle)
- [x] Bookmarks (m to toggle, [/] to navigate)

## Milestone 5: Multiple Sources ✅

- [x] Kubernetes pod log source (kubectl logs -f)
- [x] SSH remote file tailing (ssh + tail -F)

## Milestone 6: Themes ✅

- [x] Themeable color system
- [x] Built-in themes: default, kawaii, cyber, dracula, monochrome
- [x] Theme selection via config file and environment variable

## Release Prep ✅

- [x] Unit tests for core modules (filter, app, theme, config) - 51 tests
- [x] Module-level documentation (doc comments)
- [x] Centralized constants (buffer sizes, debounce, scroll lines)
- [x] Improved error messages with actionable hints
- [x] Config error reporting (warns on invalid TOML)
- [x] CLI help with examples and environment variables
- [x] README troubleshooting section
- [x] CI workflow fix (main → master branch)

## Future Ideas

- [ ] Docker multi-container support (`--docker-all` to auto-discover running containers)
- [ ] Multiple panes (split view)
- [ ] Multiple source switching at runtime (Tab/s to cycle sources)
- [ ] Kubernetes pod selector UI
- [ ] Log aggregation from multiple sources
- [ ] Log correlation/linking between sources
- [ ] Search history
- [ ] Named bookmark groups
- [ ] Custom theme definitions in config
- [ ] Plugin system for custom sources
