---
id: debug-review
type: summary
status: unverified
title: debug::review
source: myrn summarize
about: sym:crates/myrn/src/cmd/debug.rs#cmd::debug::review
anchors:
- sym:crates/myrn/src/cmd/debug.rs#cmd::debug::review@2bdfb67f070af5e4
map_version: '2'
---
# debug::review

Scores a marked checklist file when `score` is given, otherwise samples learned records and prints or writes a review checklist (or JSON when ctx.json is set). Returns ExitCode::SUCCESS, or an error if reading or writing fails.

## Behaviour

- score = Some(file) -> reads the file and emits review::score output via render_score; sample, seed and out are ignored
- json mode -> prints all sampled items as pretty JSON to stdout; out is not written
- out = Some(path) -> writes the checklist to that file and prints "Wrote N records to path"; out = None -> prints the checklist to stdout

## Terms

record review, gate 2, checklist, sampling, random sample, seed, score file, marked records

## Connections

- **Location:** `crates/myrn/src/cmd/debug.rs:13-49`
- **Signature:** `pub fn review( ctx: &Ctx, sample: usize, seed: u64, out: Option<&Path>, score: Option<&Path>, ) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-debug-rs]]
- **Calls:** [[engine-review-items]], [[review-sample]], [[review-checklist]], [[review-score-review]], [[debug-render-score]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
