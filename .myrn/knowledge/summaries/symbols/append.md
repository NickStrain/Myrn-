---
id: append
type: summary
status: unverified
title: append
source: myrn summarize
about: sym:bench/src/main.rs#append
anchors:
- sym:bench/src/main.rs#append@ee5ba506149ca588
---
# append

Appends one JSON line for a RunRecord to the given file, creating it if needed; it never truncates existing content.

## Connections

- **Location:** `bench/src/main.rs:370-377`
- **Signature:** `fn append(path: &Path, rec: &RunRecord) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Called by:** [[real-main]]
