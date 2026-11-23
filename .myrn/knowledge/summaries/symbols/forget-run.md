---
id: forget-run
type: summary
status: unverified
title: forget::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/forget.rs#cmd::forget::run
anchors:
- sym:crates/myrn/src/cmd/forget.rs#cmd::forget::run@5dd679961712e628
map_version: '2'
---
# forget::run

Plans the removal of record `id` via the engine, confirms with the user unless `yes` is true, then removes it and prints the ids, links, and notes affected. Returns exit code 1 when the user declines.

## Behaviour

- yes=false and stdin not a terminal -> error asking for --yes; nothing is removed
- yes=false on a terminal -> lists the ids, files, and links in the plan; only trimmed "y", "Y", or "yes" proceeds, anything else prints "Nothing removed." and returns exit code 1
- yes=true -> skips the prompt and removes immediately; output lists removed ids, any removed links, and a note that shared notes persist in git history until committed

## Terms

permanent delete, purge, remove record, confirmation prompt, yes flag, dry run plan, unlink, shared notes

## Connections

- **Location:** `crates/myrn/src/cmd/forget.rs:10-50`
- **Signature:** `pub fn run(ctx: &Ctx, id: &str, yes: bool) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-forget-rs]]
- **Calls:** [[engine-forget-plan]], [[engine-forget]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
