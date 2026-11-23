---
id: claude-code-handler
type: summary
status: unverified
title: claude_code::handler
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::handler
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::handler@6621d768eeb1c040
map_version: '2'
---
# claude_code::handler

Builds a command hook object that runs the myrn binary with args hook, claude and the event name, using the given timeout.

## Behaviour

- /opt/myrn/bin/myrn, SessionStart, 30 -> type command, args [hook, claude, SessionStart], timeout 30
- Binary path is converted lossily to a string
- args is a list, so no shell is involved

## Terms

hook entry, command hook, hook json, timeout

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:48-55`
- **Signature:** `fn handler(binary: &Path, event: &str, timeout: u64) -> Value`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Called by:** [[claudecode-install]]
