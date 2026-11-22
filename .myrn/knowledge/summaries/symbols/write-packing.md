---
id: write-packing
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/packing.rs#write
anchors:
- sym:crates/myrn-engine/tests/packing.rs#write@bf0595a78a1deb5b
---
# write

Writes each (relative path, content) pair under a root directory, creating parent directories as needed, and panics on any I/O error.

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs:9-15`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-packing-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-packing]]
