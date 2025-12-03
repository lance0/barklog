# bark

A fast, keyboard-driven TUI for exploring logs from files, Docker containers, Kubernetes pods, and remote servers.

[![CI](https://github.com/lance0/bark/actions/workflows/ci.yml/badge.svg)](https://github.com/lance0/bark/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/bark.svg)](https://crates.io/crates/bark)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Features

- **Multiple log sources**: Local files, Docker containers, Kubernetes pods, SSH remote files
- **Multi-source aggregation**: View logs from multiple containers/pods in one merged timeline
- **Real-time streaming**: Auto-follows new log lines with smart scroll behavior
- **Powerful filtering**: Substring and regex filtering with live preview
- **Search highlighting**: Matching text highlighted, navigate with n/N
- **Bookmarks**: Mark important lines, jump between them with [/]
- **ANSI color preservation**: View colorized logs as intended
- **Log level detection**: Auto-colors ERROR, WARN, INFO, DEBUG, TRACE
- **JSON pretty-printing**: Expand single-line JSON logs for readability
- **Timestamp parsing**: View relative times ("5s ago", "2m ago")
- **Export**: Save filtered lines to a file
- **Split view**: View logs in multiple panes with independent filters (vim-style keybinds)
- **Configurable**: Persistent settings via config file or environment
- **Themes**: Built-in color themes (default, kawaii, cyber, dracula, monochrome, and 6 more)

## Installation

### From crates.io

```bash
cargo install bark
```

### From source

```bash
git clone https://github.com/lance0/bark.git
cd bark
cargo build --release
# Binary is at target/release/bark
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/lance0/bark/releases).

## Usage

```bash
# Tail a local file
bark /var/log/syslog

# Follow Docker container logs
bark --docker my-container

# Follow Kubernetes pod logs
bark --k8s my-pod
bark --k8s my-pod -n my-namespace
bark --k8s my-pod -n my-namespace -c my-container

# Tail a remote file via SSH
bark --ssh user@host /var/log/app.log

# Multiple sources (logs merged chronologically)
bark --docker nginx --docker redis --docker postgres
bark --k8s frontend --k8s backend -n production
bark /var/log/app.log --docker nginx
```

## Keyboard Shortcuts

### Navigation
| Key | Action |
|-----|--------|
| `j` / `k` | Scroll down/up |
| `h` / `l` | Scroll left/right (when wrap off) |
| `H` / `L` | Large horizontal scroll |
| `0` | Scroll to line start |
| `g` / `G` | Go to top/bottom |
| `PgUp` / `PgDn` | Page up/down |
| `Ctrl+u` / `Ctrl+d` | Half page up/down |
| `n` / `N` | Next/previous match |
| `m` | Toggle bookmark |
| `[` / `]` | Previous/next bookmark |

### Filtering
| Key | Action |
|-----|--------|
| `/` | Start filter input |
| `↑` / `↓` | Browse filter history |
| `r` | Toggle regex mode |
| `Enter` | Apply filter |
| `Esc` | Clear filter / cancel |
| `s` | Save current filter |
| `e` | Export filtered lines |

### Display
| Key | Action |
|-----|--------|
| `p` | Pause/resume auto-scroll |
| `w` | Toggle line wrapping |
| `#` | Toggle line numbers |
| `c` | Toggle level colors |
| `t` | Toggle relative time |
| `J` | Toggle JSON pretty-print |
| `b` | Toggle side panel |
| `Tab` | Cycle panel focus |
| `y` | Yank (copy) line to clipboard |
| `S` | Open settings |
| `?` | Show help |
| `q` | Quit |

### Sources (when focused on Sources panel)
| Key | Action |
|-----|--------|
| `j` / `k` | Navigate sources |
| `Space` | Toggle source visibility |
| `v` | Solo view (show only selected) |
| `a` | Show all sources |

### Runtime Source Discovery
| Key | Action |
|-----|--------|
| `D` | Open Docker container picker |
| `K` | Open Kubernetes pod picker |

In the picker overlay:
| Key | Action |
|-----|--------|
| `j` / `k` | Navigate list |
| `Space` | Toggle checkbox (multi-select) |
| `Enter` | Add selected sources |
| `Esc` | Cancel |

### Split View (Ctrl+W prefix)
| Key | Action |
|-----|--------|
| `Ctrl+W, v` | Vertical split (side-by-side) |
| `Ctrl+W, s` | Horizontal split (stacked) |
| `Ctrl+W, q` | Close current pane |
| `Ctrl+W, w` | Cycle to next pane |
| `Ctrl+W, h/j/k/l` | Navigate between panes |

Each pane has independent scroll position, filter, bookmarks, and source visibility.

## Configuration

Bark reads configuration from `~/.config/bark/config.toml`:

```toml
# Maximum lines in the ring buffer
max_lines = 10000

# Enable log level coloring
level_colors = true

# Enable line wrapping by default
line_wrap = false

# Show side panel on startup
show_side_panel = true

# Directory for exported logs
export_dir = "/tmp"

# Color theme (11 options, see below)
theme = "default"
```

### Environment Variables

Environment variables override config file settings:

- `BARK_MAX_LINES` - Maximum lines in buffer
- `BARK_LEVEL_COLORS` - Enable level coloring (1/true or 0/false)
- `BARK_LINE_WRAP` - Enable line wrapping
- `BARK_SIDE_PANEL` - Show side panel
- `BARK_EXPORT_DIR` - Export directory
- `BARK_THEME` - Color theme (see Themes section below)
- `BARK_SSH_HOST_KEY_CHECKING` - SSH host key verification: `yes` (default, strict), `accept-new`, or `no`

## Themes

Bark includes 11 built-in color themes:

| Theme | Description |
|-------|-------------|
| `default` | Classic terminal colors |
| `kawaii` | Cute pastel colors |
| `cyber` | Neon futuristic colors |
| `dracula` | Popular dark theme |
| `monochrome` | Grayscale only |
| `matrix` | Green on black hacker style |
| `nord` | Arctic, north-bluish colors |
| `gruvbox` | Retro groove warm colors |
| `catppuccin` | Soothing pastel (Mocha variant) |
| `tokyo_night` | Dark theme inspired by Tokyo city lights |
| `solarized` | Precision colors for readability |

Cycle through themes with `S` (Settings) then `Space` on the Theme option.

## Status Bar Indicators

The status bar shows active modes:
- `[F]` - Follow mode (auto-scroll enabled)
- `[P]` - Paused (auto-scroll disabled, press `p` or `G` to resume)
- `[.*]` - Regex filter mode
- `[W]` - Line wrap enabled
- `[C]` - Level colors enabled
- `[T]` - Relative time enabled
- `[J]` - JSON pretty-print enabled
- `[+N]` - Horizontal scroll offset
- `[N/s]` - Lines per second throughput (shown during active logging)

## Requirements

- **Rust** 1.85+ (for Rust 2024 edition)
- **For Docker**: `docker` command available
- **For Kubernetes**: `kubectl` configured
- **For SSH**: SSH key authentication recommended

## Troubleshooting

### Docker source not working

- Ensure Docker is installed and running: `docker ps`
- Check container name exists: `docker ps -a | grep <container>`
- Verify you have permissions: try `docker logs <container>` directly

### Kubernetes source not working

- Verify kubectl is configured: `kubectl cluster-info`
- Check pod exists: `kubectl get pods [-n namespace]`
- For multi-container pods, specify `-c container_name`

### SSH source not working

- Ensure SSH key authentication is set up (bark uses `BatchMode=yes`)
- Test connection manually: `ssh user@host "tail -1 /path/to/log"`
- Check the remote file exists and is readable

### High memory usage

- Reduce buffer size: `export BARK_MAX_LINES=5000` or set in config
- Use filters to reduce visible lines

### Filter not matching expected lines

- Check if regex mode is enabled (status bar shows `[.*]`)
- Substring matching is case-insensitive by default
- Press `r` to toggle between regex and substring mode

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
