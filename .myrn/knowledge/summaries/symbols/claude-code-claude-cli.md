---
id: claude-code-claude-cli
type: summary
status: unverified
title: claude_code::claude_cli
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::claude_cli
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::claude_cli@40dafccc50c2816f
map_version: '2'
---
# claude_code::claude_cli

Runs the claude CLI with the given args in the repo root and returns its output, or None if it cannot be run. Non-zero exit still returns Some, so the caller must check the status.

## Behaviour

- ctx.run_agent_cli false -> None, and PATH is not searched
- claude not found by detect::which -> None
- Spawn failure -> None; a failed exit status -> Some(output)

## Terms

agent cli, subprocess, claude command, external command

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:65-75`
- **Signature:** `fn claude_cli(ctx: &InstallCtx, args: &[&str]) -> Option<std::process::Output>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[detect-which]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]]
