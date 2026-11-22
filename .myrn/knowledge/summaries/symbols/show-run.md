---
id: show-run
type: summary
status: unverified
title: show::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/show.rs#cmd::show::run
anchors:
- sym:crates/myrn/src/cmd/show.rs#cmd::show::run@fa9e7e55580c352d
---
# show::run

Takes a Ctx and node id string, calls engine.show(id) (error propagates if lookup fails), then emits the found node: in text mode it renders only fields that are non-empty and lists relations as '->' for outgoing (src == node id) or '<-' for incoming, with an optional '(status)' suffix; returns an ExitCode wrapped in Result.

## Connections

- **Location:** `crates/myrn/src/cmd/show.rs:9-49`
- **Signature:** `pub fn run(ctx: &Ctx, id: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-show-rs]]
- **Calls:** [[engine-show]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
