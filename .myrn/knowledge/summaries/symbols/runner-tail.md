---
id: runner-tail
type: summary
status: unverified
title: runner::tail
source: myrn summarize
about: sym:bench/src/runner.rs#runner::tail
anchors:
- sym:bench/src/runner.rs#runner::tail@75f28c3123b5726c
map_version: '2'
---
# runner::tail

Returns the last max bytes of a string prefixed with an ellipsis, moving the cut forward to a UTF-8 character boundary.

## Behaviour

- Length <= max -> returned unchanged
- tail("abcdef", 3) -> "…def"
- tail("aé", 1) -> "…" because the cut moves past the multi-byte character; the result can be up to max plus 3 bytes

## Terms

truncate, last bytes, cut output, keep end, utf-8 safe, trim output

## Connections

- **Location:** `bench/src/runner.rs:288-297`
- **Signature:** `fn tail(s: &str, max: usize) -> String`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-run-check]], [[runner-run-inner]], [[runner-validate]]
- **Tests:** `tails_respect_char_boundaries`
