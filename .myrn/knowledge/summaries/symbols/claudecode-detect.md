---
id: claudecode-detect
type: summary
status: unverified
title: ClaudeCode::detect
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::detect
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::ClaudeCode::detect@b7ff4284a6c3c653
map_version: '2'
---
# ClaudeCode::detect

Reports Claude Code as present if the claude binary is on PATH or the home .claude directory exists. Returns a description naming what was found, or None.

## Behaviour

- claude on PATH -> description with its path, checked first
- No claude but ~/.claude is a directory -> description naming that directory
- Neither found, or no home directory -> None

## Terms

installation check, presence check, auto detect, home directory

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:86-92`
- **Signature:** `fn detect(&self) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[detect-home]], [[detect-which]]
