---
id: diff-run
type: summary
status: unverified
title: diff::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/diff.rs#cmd::diff::run
anchors:
- sym:crates/myrn/src/cmd/diff.rs#cmd::diff::run@c6b8a013cf92e881
map_version: '2'
---
# diff::run

Diffs shared knowledge from a starting point (default HEAD) to an optional end point, and emits the result as either structured data or formatted text. It returns the ExitCode from ctx.emit, and any engine error propagates unchanged.

## Behaviour

- no args -> from is HEAD and to is None, passed unchanged to the engine
- no changes and map_changes is 0 -> text ends with 'No changes to shared knowledge.'
- changed entry with detail None -> prints empty brackets '[]' after the title

## Connections

- **Location:** `crates/myrn/src/cmd/diff.rs:9-38`
- **Signature:** `pub fn run(ctx: &Ctx, from: Option<&str>, to: Option<&str>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-diff-rs]]
- **Calls:** [[engine-diff-knowledge]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
