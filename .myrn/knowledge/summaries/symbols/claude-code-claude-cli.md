---
id: claude-code-claude-cli
type: summary
status: unverified
title: claude_code::claude_cli
source: myrn summarize
about: sym:crates/myrn-adapters/src/claude_code.rs#claude_code::claude_cli
anchors:
- sym:crates/myrn-adapters/src/claude_code.rs#claude_code::claude_cli@40dafccc50c2816f
---
# claude_code::claude_cli

Runs the claude CLI with the given args in the repo root and returns its Output. Returns None if ctx.run_agent_cli is false, claude is not on PATH, or the spawn fails.

## Connections

- **Location:** `crates/myrn-adapters/src/claude_code.rs:65-75`
- **Signature:** `fn claude_cli(ctx: &InstallCtx, args: &[&str]) -> Option<std::process::Output>`
- **In:** [[file-crates-myrn-adapters-src-claude-code-rs]]
- **Calls:** [[detect-which]]
- **Called by:** [[claudecode-install]], [[claudecode-uninstall]], [[claudecode-inspect]]
