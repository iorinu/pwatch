<div align="center">

![logo](assets/logo.png)

A fast, friendly CLI/TUI tool for viewing port usage and killing processes.

Quickly resolve "port already in use" errors — check ports, identify processes, and kill them in a single command.

[日本語版 README](README.ja.md)

<!-- Replace with your own demo GIF -->
![demo](assets/demo.gif)

</div>

## Installation

```bash
cargo install --path .
```

## Usage

### List all listening ports

```bash
pwatch list
```

Output as JSON:

```bash
pwatch list --json
```

Continuously refresh (like `top`):

```bash
pwatch list --watch                # refresh every 2 seconds
pwatch list --watch --interval 5   # custom interval
```

### Check a specific port

```bash
pwatch check 8080
```

### Kill process(es) on one or more ports

```bash
pwatch kill 8080                     # SIGTERM
pwatch kill 8080 --force             # SIGKILL
pwatch kill 8080 3000 5173           # kill multiple ports at once
```

If you get a permission error:

```bash
sudo pwatch kill 8080
```

### TUI mode

```bash
pwatch ui
```

| Key | Action |
|-----|--------|
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `d` | Kill with SIGTERM (with confirmation) |
| `D` | Kill with SIGKILL (with confirmation) |
| `/` | Search mode |
| `r` | Manual refresh |
| `a` | Toggle auto-refresh |
| `+` / `-` | Adjust auto-refresh interval (±0.5s) |
| `q` / `Esc` | Quit |

### Configuration

Disable the startup banner:

```bash
pwatch config banner off
```

Re-enable it:

```bash
pwatch config banner on
```

Switch the display language (runtime messages and TUI). The default is English:

```bash
pwatch config lang ja   # Japanese
pwatch config lang en   # English (default)
```

> Note: `--help` output is always in English. Only runtime messages (CLI output, TUI labels) are localized.

Settings are saved to `~/.config/pwatch/config.toml`.

## Supported Platforms

| OS | Scan Method |
|----|-------------|
| Linux | Direct `/proc/net/tcp` parsing |
| macOS | Via `lsof` command |

## Build

```bash
cargo build --release
```

## License

Licensed under the [MIT License](LICENSE).

Copyright (c) 2026 iorinu
