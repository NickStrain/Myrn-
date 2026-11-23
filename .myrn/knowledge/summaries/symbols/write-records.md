---
id: write-records
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#write
anchors:
- sym:crates/myrn-engine/tests/records.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, contents) pair under root, creating parent directories first. Overwrites any existing file at that path.

## Behaviour

- [("pkg/x.py", "...")] -> file written at root/pkg/x.py, parents created
- existing file at the path -> contents replaced
- IO failure -> panic via unwrap

## Terms

write files, create directories, fixture files, overwrite file, nested paths, test scaffolding

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:35-41`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-58c6]]
- **Tests:** `summaries_show_on_cards_and_go_stale_with_the_code`, `shell_edits_are_captured_and_lead_to_a_record_request`
