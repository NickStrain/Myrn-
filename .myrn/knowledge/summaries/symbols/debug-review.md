---
id: debug-review
type: summary
status: unverified
title: debug::review
source: myrn summarize
about: sym:crates/myrn/src/cmd/debug.rs#cmd::debug::review
anchors:
- sym:crates/myrn/src/cmd/debug.rs#cmd::debug::review@2bdfb67f070af5e4
---
# debug::review

When score is set, it reads that file, passes its text to review::score, and emits the result through ctx.emit (JSON or render_score text), ignoring sample and seed. Otherwise it samples engine.review_items() with review::sample(sample, seed), and in ctx.json mode prints the items as pretty JSON to stdout (out is ignored). Otherwise it builds a checklist whose header gives the sampled count, total count, and seed, then writes it to out (printing 'Wrote N records to PATH') or prints it to stdout.

## Connections

- **Location:** `crates/myrn/src/cmd/debug.rs:13-49`
- **Signature:** `pub fn review( ctx: &Ctx, sample: usize, seed: u64, out: Option<&Path>, score: Option<&Path>, ) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-debug-rs]]
- **Calls:** [[engine-review-items]], [[review-sample]], [[review-checklist]], [[review-score-review]], [[debug-render-score]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
