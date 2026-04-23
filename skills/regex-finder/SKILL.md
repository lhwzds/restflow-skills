---
id: regex-finder
name: Regex Finder
kind: skill_binary
---

# Regex Finder

Binary skill that reads JSON from stdin with `pattern` and `text`, evaluates the regex using Rust's `regex` crate, and prints JSON to stdout with `ok` and `matched`.

## Input

```json
{
  "pattern": "foo.*bar",
  "text": "foo test bar"
}
```

## Output

```json
{
  "ok": true,
  "matched": true
}
```
