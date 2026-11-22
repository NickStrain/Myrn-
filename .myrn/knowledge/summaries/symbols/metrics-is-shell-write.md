---
id: metrics-is-shell-write
type: summary
status: unverified
title: metrics::is_shell_write
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::is_shell_write
anchors:
- sym:bench/src/metrics.rs#metrics::is_shell_write@d29e7a780cf4dd06
---
# metrics::is_shell_write

Returns true if any segment of a shell command writes files: sed -i, perl -i, tee, patch, git apply, or a redirect to a real file.

## Connections

- **Location:** `bench/src/metrics.rs:279-281`
- **Signature:** `pub fn is_shell_write(cmd: &str) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-segments]]
- **Tests:** `shell_commands`
