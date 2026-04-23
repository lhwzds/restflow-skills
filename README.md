# RestFlow Binary Skills

Pre-compiled Rust tools for **any LLM agent or automation framework**. Each skill is a standalone binary that communicates via **JSON stdin/stdout** — no SDK, no runtime dependency, works with anything that can spawn a process.

## Why Binary Skills?

Most LLM agent tools are Python libraries or Node packages. That means:
- Dependency hell (virtualenv, pip, npm)
- Slow cold start
- Version conflicts between skills

Binary skills solve this:
- **Zero dependencies** — single static binary, just download and run
- **Universal protocol** — JSON in, JSON out via stdin/stdout
- **Works everywhere** — macOS, Linux, Windows
- **Any agent can use it** — RestFlow, Claude Code, OpenAI Codex, LangChain, AutoGPT, CrewAI, shell scripts, cron jobs, anything

## Available Skills

| Skill | Description | Version |
|-------|-------------|---------|
| [cdp-browser](#cdp-browser) | Control Chrome through CDP — navigate, click, type, evaluate JS, screenshot | 0.1.0 |
| [regex-finder](#regex-finder) | Match text against regex patterns, return structured JSON | 0.1.0 |

## Install

### One-liner (macOS / Linux)

```bash
# Download latest cdp-browser
mkdir -p ~/.local/bin
curl -L https://github.com/lhwzds/restflow-skills/releases/latest/download/cdp-browser-aarch64-macos.tar.gz | tar xz -C ~/.local/bin
chmod +x ~/.local/bin/cdp-browser
```

### Specify version

```bash
curl -L https://github.com/lhwzds/restflow-skills/releases/download/cdp-browser@0.1.0/cdp-browser-aarch64-macos.tar.gz | tar xz -C ~/.local/bin
```

### What's inside each archive

```
cdp-browser-aarch64-macos.tar.gz
├── cdp-browser       ← the binary
├── SKILL.md          ← documentation & usage reference
└── artifact.json     ← metadata for tooling integration
```

## Usage

Every skill supports two modes:

### CLI Mode

Pass arguments directly:

```bash
./cdp-browser launch --port 9222
./cdp-browser open --port 9222 --url https://example.com
./regex-finder --pattern "foo.*bar" --text "foo test bar"
```

### JSON Mode (for agents)

Pipe JSON to stdin, get JSON from stdout:

```bash
echo '{"action":"open","port":9222,"url":"https://example.com"}' | ./cdp-browser
# → {"ok":true,"action":"open","target_id":"ABC","url":"https://example.com/"}

echo '{"pattern":"foo.*bar","text":"foo test bar"}' | ./regex-finder
# → {"ok":true,"matched":true}
```

## Integration Guides

### RestFlow

Add to `~/.restflow/skills/`:

```bash
mkdir -p ~/.restflow/skills/cdp-browser
curl -L https://github.com/lhwzds/restflow-skills/releases/latest/download/cdp-browser-aarch64-macos.tar.gz | tar xz -C ~/.restflow/skills/cdp-browser
chmod +x ~/.restflow/skills/cdp-browser/cdp-browser
```

Then reference in agent config:

```json
{
  "skills": ["cdp-browser"],
  "tools": ["bash"]
}
```

### Claude Code / OpenAI Codex / Any CLI Agent

These agents can run shell commands. Just download the binary and invoke it:

```bash
# In your agent's tool call:
# Tool: bash
# Command: echo '{"action":"eval","port":9222,"expression":"document.title"}' | /path/to/cdp-browser
```

### LangChain / Python

```python
import subprocess, json

def call_skill(binary_path: str, payload: dict) -> dict:
    result = subprocess.run(
        [binary_path],
        input=json.dumps(payload),
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr)
    return json.loads(result.stdout)

# Example: regex match
out = call_skill("./regex-finder", {"pattern": r"\d+", "text": "abc 123"})
print(out)  # {"ok": true, "matched": true}

# Example: open a browser page
out = call_skill("./cdp-browser", {"action": "open", "port": 9222, "url": "https://example.com"})
print(out)  # {"ok": true, "action": "open", "target_id": "ABC", ...}
```

### Node.js / TypeScript

```typescript
import { execFileSync } from "child_process";

function callSkill(binaryPath: string, payload: object): object {
  const result = execFileSync(binaryPath, [], {
    input: JSON.stringify(payload),
    encoding: "utf-8",
  });
  return JSON.parse(result);
}

// Example
const out = callSkill("./cdp-browser", {
  action: "screenshot",
  port: 9222,
  path: "/tmp/screenshot.png",
});
console.log(out); // { ok: true, path: "/tmp/screenshot.png", size: 44321 }
```

### Shell Scripts / Cron

```bash
#!/bin/bash
RESULT=$(echo '{"pattern":"error.*failed","text":"some log line"}' | ./regex-finder)
MATCHED=$(echo "$RESULT" | jq -r '.matched')
if [ "$MATCHED" = "true" ]; then
  echo "Alert: error found"
fi
```

### Any Other Agent

The protocol is simple:

```
┌────────────┐     JSON      ┌──────────────┐     JSON      ┌────────────┐
│  Any Agent  │ ─────────────→│  Binary Skill │────────────→│  Any Agent  │
│             │    stdin       │              │    stdout     │             │
└────────────┘                └──────────────┘                └────────────┘
```

1. Spawn the binary as a subprocess
2. Write a JSON object to its stdin, then close stdin
3. Read one JSON object from its stdout
4. Exit code 0 = success, non-zero = error (details on stderr)

That's it. No HTTP server, no socket, no API key. Just process I/O.

## Skill Details

### cdp-browser

Control Chrome through the Chrome DevTools Protocol.

**Actions:** `launch`, `status`, `open`, `list`, `eval`, `click`, `type`, `screenshot`

```bash
# Start Chrome
echo '{"action":"launch","port":9222}' | ./cdp-browser

# Navigate
echo '{"action":"open","port":9222,"url":"https://example.com"}' | ./cdp-browser

# Click
echo '{"action":"click","port":9222,"selector":"a.more"}' | ./cdp-browser

# Type
echo '{"action":"type","port":9222,"selector":"input[name=q]","text":"hello"}' | ./cdp-browser

# Evaluate JS
echo '{"action":"eval","port":9222,"expression":"document.title"}' | ./cdp-browser

# Screenshot
echo '{"action":"screenshot","port":9222,"path":"/tmp/shot.png"}' | ./cdp-browser
```

### regex-finder

Match text against regex patterns using Rust's `regex` crate.

```bash
echo '{"pattern":"\\d+","text":"abc 123 def"}' | ./regex-finder
# → {"ok":true,"matched":true}
```

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Build all skills

```bash
cargo build --workspace --release
```

### Build a specific skill

```bash
cargo build --release -p cdp-browser
```

### Test

```bash
cargo test --workspace
```

### Lint

```bash
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

4. CI automatically builds and publishes binaries for macOS, Linux, and Windows. Each archive includes the binary + `SKILL.md` + `artifact.json`.

## License

MIT
