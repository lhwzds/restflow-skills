# RestFlow Binary Skills

Pre-compiled Rust tools for LLM agents and automation frameworks. Each skill is a standalone binary that communicates via **JSON stdin/stdout** — no SDK, no runtime dependency, works with any agent that can spawn a process.

## Why Binary Skills?

Most LLM agent tools are Python libraries or Node packages. That means:
- Dependency hell (virtualenv, pip, npm)
- Slow cold start
- Version conflicts between skills

Binary skills solve this:
- **Zero dependencies** — single static binary, just download and run
- **Universal protocol** — JSON in, JSON out via stdin/stdout
- **Any agent can use it** — RestFlow, Claude Code, OpenAI Codex, LangChain, AutoGPT, shell scripts, anything that can run a command

## Available Skills

| Skill | Description | Version |
|-------|-------------|---------|
| [cdp-browser](#cdp-browser) | Control Chrome through CDP — navigate, click, type, evaluate JS, screenshot | 0.1.0 |
| [regex-finder](#regex-finder) | Match text against regex patterns, return structured JSON | 0.1.0 |

## Quick Start

### Download

```bash
# macOS (Apple Silicon)
curl -L https://github.com/lhwzds/restflow-skills/releases/latest/download/cdp-browser-aarch64-macos.tar.gz | tar xz

# Linux (x86_64)
curl -L https://github.com/lhwzds/restflow-skills/releases/latest/download/cdp-browser-x86_64-linux.tar.gz | tar xz

# Or download a specific version
curl -L https://github.com/lhwzds/restflow-skills/releases/download/cdp-browser@0.1.0/cdp-browser-aarch64-macos.tar.gz | tar xz
```

### Run

Every skill supports two modes:

**CLI mode** — pass arguments directly:
```bash
./cdp-browser launch --port 9222
./cdp-browser open --port 9222 --url https://example.com
```

**JSON mode** — pipe JSON to stdin (ideal for agents):
```bash
echo '{"action":"open","port":9222,"url":"https://example.com"}' | ./cdp-browser
# → {"ok":true,"action":"open","data":{"target_id":"...","url":"https://example.com/"}}
```

---

## Skill Reference

### cdp-browser

Control Chrome through the Chrome DevTools Protocol (CDP).

**Actions:**

| Action | Description | Required Fields |
|--------|-------------|-----------------|
| `launch` | Start Chrome with CDP enabled | `port` |
| `status` | Check if CDP is reachable | `port` |
| `open` | Open URL in a new tab | `port`, `url` |
| `list` | List all open tabs | `port` |
| `eval` | Run JavaScript in a tab | `port`, `expression` |
| `click` | Click an element by CSS selector | `port`, `selector` |
| `type` | Type text into an element | `port`, `selector`, `text` |
| `screenshot` | Capture tab to PNG | `port`, `path` |

**Examples:**

```bash
# Launch Chrome
./cdp-browser launch --port 9222

# Open a page (CLI)
./cdp-browser open --port 9222 --url https://example.com

# Open a page (JSON)
echo '{"action":"open","port":9222,"url":"https://example.com"}' | ./cdp-browser

# Get page title
echo '{"action":"eval","port":9222,"expression":"document.title"}' | ./cdp-browser

# Click a button
echo '{"action":"click","port":9222,"selector":"#submit-btn"}' | ./cdp-browser

# Type into search box
echo '{"action":"type","port":9222,"selector":"input[name=q]","text":"RestFlow"}' | ./cdp-browser

# Screenshot
echo '{"action":"screenshot","port":9222,"path":"/tmp/page.png"}' | ./cdp-browser

# List tabs
echo '{"action":"list","port":9222}' | ./cdp-browser
```

**Output format:**
```json
{
  "ok": true,
  "action": "open",
  "message": null,
  "data": {
    "target_id": "ABC123",
    "url": "https://example.com/",
    "title": "Example Domain"
  }
}
```

---

### regex-finder

Match text against a regex pattern and return whether it matches.

**Input:**
```json
{
  "pattern": "foo.*bar",
  "text": "foo test bar"
}
```

**Output:**
```json
{
  "ok": true,
  "matched": true
}
```

**CLI:**
```bash
echo '{"pattern":"\\d+","text":"abc123"}' | ./regex-finder
# → {"ok":true,"matched":true}
```

---

## Integration Guides

### RestFlow

Binary skills are natively supported. Place the binary under `~/.restflow/skills/{skill-name}/bin/` with a `SKILL.md`:

```
~/.restflow/skills/cdp-browser/
├── bin/
│   └── cdp-browser          ← the binary
└── SKILL.md                 ← metadata
```

RestFlow agents can then invoke the skill directly via the `skill` tool.

### Claude Code / Codex CLI

Use as a bash tool in your agent session:

```bash
# In your agent's shell
./cdp-browser launch --port 9222
echo '{"action":"open","port":9222,"url":"https://news.ycombinator.com"}' | ./cdp-browser
echo '{"action":"eval","port":9222,"expression":"[...document.querySelectorAll('\''.titleline a'\'')].map(a=>a.textContent).join('\''\\n'\'')"}'\ | ./cdp-browser
```

### LangChain / Python Agents

```python
import subprocess, json

def call_skill(binary_path: str, input_dict: dict) -> dict:
    result = subprocess.run(
        [binary_path],
        input=json.dumps(input_dict),
        capture_output=True,
        text=True,
        timeout=30
    )
    return json.loads(result.stdout)

# Example: open a page and get the title
call_skill("./cdp-browser", {"action": "launch", "port": 9222})
call_skill("./cdp-browser", {"action": "open", "port": 9222, "url": "https://example.com"})
title = call_skill("./cdp-browser", {"action": "eval", "port": 9222, "expression": "document.title"})
print(title)  # {"ok": true, "action": "eval", "data": {"result": "Example Domain"}}
```

### OpenAI Function Calling

Define as a function tool:

```json
{
  "name": "cdp_browser",
  "description": "Control Chrome browser via CDP protocol",
  "parameters": {
    "type": "object",
    "properties": {
      "action": {"type": "string", "enum": ["launch","status","open","list","eval","click","type","screenshot"]},
      "port": {"type": "integer", "default": 9222},
      "url": {"type": "string"},
      "selector": {"type": "string"},
      "text": {"type": "string"},
      "expression": {"type": "string"},
      "path": {"type": "string"}
    },
    "required": ["action"]
  }
}
```

Then in your tool executor:
```python
result = subprocess.run(["./cdp-browser"], input=json.dumps(args), capture_output=True, text=True)
```

### Shell Scripts

```bash
#!/bin/bash
CDP="./cdp-browser"

# Launch and open a page
$CDP launch --port 9222
sleep 2
$CDP open --port 9222 --url https://example.com

# Extract data
TITLE=$(echo '{"action":"eval","port":9222,"expression":"document.title"}' | $CDP)
echo "Page title: $TITLE"

# Screenshot
$CDP screenshot --port 9222 --path /tmp/example.png
```

---

## Release Convention

Each skill is released independently using tags in the format `skill-name@version`:

```
cdp-browser@0.1.0
cdp-browser@0.2.0
regex-finder@1.0.0
```

### Assets per Release

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `skill-name-aarch64-macos.tar.gz` |
| Linux (x86_64) | `skill-name-x86_64-linux.tar.gz` |
| Windows (x86_64) | `skill-name-x86_64-windows.zip` |

---

## Develop

### Prerequisites

- Rust 1.75+
- Chrome (for cdp-browser testing)

### Build

```bash
# Build all skills
cargo build --workspace --release

# Build a specific skill
cargo build --release -p cdp-browser

# Test
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings
```

### Add a New Skill

1. Create `skills/your-skill/` with:
   - `Cargo.toml` — standard Cargo manifest, binary target
   - `src/main.rs` — implement JSON stdin/stdout protocol
   - `SKILL.md` — document actions, input/output format

2. Add to workspace `Cargo.toml`:
   ```toml
   [workspace]
   members = ["skills/*"]
   ```

3. Commit and tag:
   ```bash
   git add skills/your-skill
   git commit -m "feat: add your-skill"
   git tag your-skill@0.1.0
   git push origin main --tags
   ```

4. CI automatically builds and publishes binaries for all platforms.

### Skill Binary Protocol

Every binary skill follows the same convention:

```
┌────────────┐     JSON      ┌──────────────┐     JSON      ┌────────────┐
│  Any Agent  │ ─────────────→│  Binary Skill │────────────→│  Any Agent  │
│             │    stdin       │              │    stdout     │             │
└────────────┘                └──────────────┘                └────────────┘
```

**Rules:**
1. Read entire stdin as a JSON object
2. Process the request
3. Write exactly one JSON object to stdout
4. Exit with code 0 on success, non-zero on error
5. Errors go to stderr as plain text

This protocol is deliberately simple so any language, any framework, any agent can integrate.

## License

MIT
