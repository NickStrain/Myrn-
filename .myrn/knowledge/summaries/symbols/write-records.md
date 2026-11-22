---
id: write-records
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#write
anchors:
- sym:crates/myrn-engine/tests/records.rs#write@bf0595a78a1deb5b
---
# write

Writes each (relative path, content) pair under a root directory, creating parent directories first and panicking on any IO error.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:35-41`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-58c6]]
- **Tests:** `summaries_show_on_cards_and_go_stale_with_the_code`, `shell_edits_are_captured_and_lead_to_a_record_request`
