# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-02

Initial release.

### Added
- `pwatch list` — show all listening ports in a colored table.
- `pwatch check <port>` — check whether a specific port is in use.
- `pwatch kill <port>...` — kill the process bound to one or more ports.
  Supports `--force` for SIGKILL.
- `pwatch ui` — interactive TUI with vim-style navigation, search, kill
  confirmation dialog, and auto-refresh (`a` to toggle, `+/-` to adjust interval).
- `pwatch list --watch` — `top`-style live refresh of the port list with
  configurable `--interval`.
- `pwatch completion <shell>` — generate shell completion scripts for
  bash, zsh, fish, powershell, and elvish.
- `pwatch --version` / `-V` — print the version.
- `pwatch config banner on|off` — toggle the figlet startup banner.
- `pwatch config lang en|ja` — switch runtime messages and TUI labels
  between English and Japanese (default: English).
- `--json` global flag for machine-readable output.
- Linux scanner via direct `/proc/net/tcp` parsing.
- macOS scanner via `lsof`.

[Unreleased]: https://github.com/iorinu/pwatch/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/iorinu/pwatch/releases/tag/v0.1.0
