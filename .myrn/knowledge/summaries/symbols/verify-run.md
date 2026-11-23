---
id: verify-run
type: summary
status: unverified
title: verify::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/verify.rs#cmd::verify::run
anchors:
- sym:crates/myrn/src/cmd/verify.rs#cmd::verify::run@46ae421d31540890
map_version: '2'
---
# verify::run

With a confirm id, calls engine.confirm and emits a one-line confirmation. Without one, calls engine.promote then engine.suspects, emits both lists, and prints a text report that tells the user how to confirm or supersede each suspect.

## Behaviour

- confirm = Some(id) -> engine.confirm(id) only; promote and suspects are not called, and the output reports anchors refreshed and the new status
- confirm = None and nothing promoted or suspect -> text says every record matches the current code
- suspect anchor with gone = true -> printed as 'gone: name location' instead of 'changed: name location'; a missing location prints an empty string

## Terms

verify command, re-check records, promote records, suspect records, confirm record, stale record, anchors refreshed, code drift

## Connections

- **Location:** `crates/myrn/src/cmd/verify.rs:12-64`
- **Signature:** `pub fn run(ctx: &Ctx, confirm: Option<&str>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-verify-rs]]
- **Calls:** [[engine-suspects]], [[engine-confirm]], [[engine-promote]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
