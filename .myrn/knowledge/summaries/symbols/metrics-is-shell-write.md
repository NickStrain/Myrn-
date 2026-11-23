---
id: metrics-is-shell-write
type: summary
status: unverified
title: metrics::is_shell_write
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_shell_write
anchors:
- sym:bench/src/metrics.rs#metrics::is_shell_write@d29e7a780cf4dd06
map_version: '2'
---
# metrics::is_shell_write

Returns true if any segment of a shell command changes files, using the same rules as is_write_segment.

## Behaviour

- 'sed -i s/a/b/ a.py' -> true
- 'cd x && sed -i.bak ...' -> true
- 'cargo test 2>&1 | tail' -> false

## Terms

shell edit, file write detection, in-place edit, shell mutation, bash write

## Connections

- **Location:** `bench/src/metrics.rs:279-281`
- **Signature:** `pub fn is_shell_write(cmd: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-segments]]
- **Tests:** `shell_commands`
