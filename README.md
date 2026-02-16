# claude-statusline

Minimal Rust binary that renders a formatted statusline for [Claude Code](https://docs.anthropic.com/en/docs/claude-code).

Reads JSON from stdin, prints a single line:

```
Opus ◦ dotfiles ◦ 42% ◦ master
```

## Install

```bash
brew install nclandrei/tap/claude-statusline
```

## Usage

Add to your Claude Code settings (`~/.claude/settings.json`):

```json
{
  "statusLine": {
    "type": "command",
    "command": "claude-statusline"
  }
}
```

## Build from source

```bash
cargo build --release
```

## License

MIT
