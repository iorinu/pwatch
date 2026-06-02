<div align="center">

![logo](assets/logo.png)

A fast, friendly CLI/TUI tool for viewing port usage and killing processes.

Quickly resolve "port already in use" errors — check ports, identify processes, and kill them in a single command.

[日本語版 README](README.ja.md)

<!-- Replace with your own demo GIF -->
![demo](assets/demo.gif)

</div>

## Features

- Quickly identify which process is using any port
- Kill processes on one or more ports in a single command
- Live `--watch` mode and an interactive TUI with auto-refresh
- English / Japanese UI, switchable at runtime
- Shell completions for bash, zsh, fish, powershell, elvish
- Single static binary with no runtime dependencies

## Installation

### Shell installer (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/iorinu/pwatch/releases/latest/download/pwatch-installer.sh | sh
```

### Homebrew (macOS / Linux)

```bash
brew install iorinu/tap/pwatch
```

### Cargo (from crates.io)

```bash
cargo install pwatch
```

### From source

```bash
git clone https://github.com/iorinu/pwatch
cd pwatch
cargo install --path .
```

Pre-built binaries for every release are also available on the
[Releases page](https://github.com/iorinu/pwatch/releases).

## Quickstart

```bash
pwatch list           # what's listening on this machine?
pwatch kill 3000      # free a stuck port
pwatch kill 3000 5173 8080   # free several at once
pwatch ui             # interactive viewer + killer (TUI)
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

### Shell completion

Generate a completion script for your shell:

```bash
pwatch completion bash > /usr/local/etc/bash_completion.d/pwatch
pwatch completion zsh  > ~/.zsh/completion/_pwatch     # ensure dir is in $fpath
pwatch completion fish > ~/.config/fish/completions/pwatch.fish
```

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`.

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
