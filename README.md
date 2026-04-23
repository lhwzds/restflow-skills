# RestFlow Binary Skills

Binary skills for [RestFlow](https://github.com/lhwzds/restflow), built with Rust.

Each skill is an independent Cargo crate in `skills/`. Skills are released as pre-compiled binaries via GitHub Releases.

## Available Skills

| Skill | Description | Version |
|-------|-------------|---------|
| [cdp-browser](skills/cdp-browser/) | Control Chrome through the Chrome DevTools Protocol | 0.1.0 |
| [regex-finder](skills/regex-finder/) | Match text against regex patterns via JSON stdin/stdout | 0.1.0 |

## Release Convention

Each skill is released independently using the tag format `skill-name@version`:

```
cdp-browser@0.1.0
regex-finder@0.1.0
```

### Assets per Release

| Platform | File |
|----------|------|
| Apple Silicon (macOS) | `skill-name-aarch64-macos.tar.gz` |
| x86_64 Linux | `skill-name-x86_64-linux.tar.gz` |
| x86_64 Windows | `skill-name-x86_64-windows.zip` |

## Install

```bash
# Example: install cdp-browser on macOS
curl -L https://github.com/lhwzds/restflow-skills/releases/download/cdp-browser@0.1.0/cdp-browser-aarch64-macos.tar.gz | tar xz
mkdir -p ~/.restflow/skills/cdp-browser/bin
mv cdp-browser ~/.restflow/skills/cdp-browser/bin/
```

## Develop

```bash
# Build all skills
cargo build --workspace

# Test all
cargo test --workspace

# Run a specific skill
cargo run -p cdp-browser -- launch --port 9222
```

## Add a New Skill

1. Create `skills/your-skill/` with `Cargo.toml`, `src/main.rs`, `SKILL.md`
2. Add to workspace `Cargo.toml` members
3. Commit and tag: `git tag your-skill@0.1.0`
4. Push: `git push origin main --tags`

CI will automatically build and publish binaries for all platforms.

## License

MIT
