# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build --release    # Build optimized binary (to target/release/claude-statusline)
cargo build              # Development build
cargo fmt                # Format code
cargo clippy             # Lint
cargo test               # Run tests (none currently exist)
```

The release profile optimizes for binary size (`opt-level = "z"`, LTO enabled, symbols stripped).

## Architecture

This is a single-file Rust CLI tool (`src/main.rs`, ~130 lines) that renders a statusline for Claude Code. It reads JSON from stdin and prints a formatted status string to stdout.

**Data flow:** stdin JSON → serde deserialization → field extraction → git branch lookup (cached) → formatted output

**Output format:** `Opus ◦ dotfiles ◦ 42% ◦ master` (model name, directory basename, context window %, git branch)

**Git branch caching:** To avoid frequent subprocess calls, git branch results are cached per-directory in `/tmp/claude-statusline-git-cache` with a 5-second TTL. Cache reads/writes are best-effort (failures silently ignored).

**Dependencies:** Only `serde` and `serde_json` — no async runtime or heavy frameworks.

## Integration

Configured in `~/.claude/settings.json` as a statusline command. Claude Code pipes JSON with `model`, `workspace`, and `context_window` fields to this binary on each render cycle, so startup time and binary size matter.
