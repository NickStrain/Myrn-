---
id: hook-run
type: summary
status: unverified
title: hook::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/hook.rs#cmd::hook::run
anchors:
- sym:crates/myrn/src/cmd/hook.rs#cmd::hook::run@3bdd5f6f241ff22b
---
# hook::run

Calls handle and always returns ExitCode::SUCCESS. It prints the returned text only if it is non-empty and sends any error to stderr as 'myrn hook: ...', so stdout stays empty on failure.

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs:22-33`
- **Signature:** `pub fn run(ctx: &Ctx, agent: &str, event: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-hook-rs]]
- **Calls:** [[hook-handle]]
- **Called by:** [[main-main]]
