---
id: runner-tail
type: summary
status: unverified
title: runner::tail
source: myrn summarize
about: sym:bench/src/runner.rs#runner::tail
anchors:
- sym:bench/src/runner.rs#runner::tail@75f28c3123b5726c
---
# runner::tail

Returns the last max bytes of s with a leading ellipsis, moving the cut forward to a UTF-8 character boundary. The ellipsis is added outside the limit, so the result can be slightly longer than max.

## Connections

- **Location:** `bench/src/runner.rs:270-279`
- **Signature:** `fn tail(s: &str, max: usize) -> String`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-run-check]], [[runner-run-inner]], [[runner-validate]]
- **Tests:** `tails_respect_char_boundaries`
