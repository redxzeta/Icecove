# Changelog

All notable changes to alcove will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] — 2026-03-06

### Changed

- Renamed project from `docs-bridge` to `alcove`
- Consolidated CLI: `alcove setup` handles all configuration (docs root, categories, diagram format, agents)
- Removed `skill`/`mcp`/`serve` subcommands — `setup` covers everything
- Setup now shows existing values as defaults, making reconfiguration easy
- Interactive document category selection with pre-checked existing config
- Simplified `install.sh` and `Makefile` to focus on binary install + setup delegation

### Added

- `dialoguer`-based TUI for all interactive prompts (replaces Python scripts)
- `clap` CLI with `setup` and `uninstall` subcommands
- `include_str!` embedded SKILL.md in binary — no external file dependency
- crates.io publishing metadata

## [0.2.0]

### Added

- Bidirectional document flow (docs-bridge ↔ project repo) with transformation rules
- Cross-repo audit: detect exposed internal docs, misplaced reports, missing public docs
- Document classification: `doc-repo-required`, `doc-repo-supplementary`, `project-repo`
- Config consolidation to `config.toml` with `docs_root`

## [0.1.0]

### Added

- Initial MCP server with stdio JSON-RPC 2.0
- Tools: overview, search, get_file, list_projects, audit, init
- Auto-detection of active project from CWD
- Support for 8 AI agents (Claude Code, Cursor, Claude Desktop, Cline, OpenCode, Codex, Antigravity, Gemini CLI)
