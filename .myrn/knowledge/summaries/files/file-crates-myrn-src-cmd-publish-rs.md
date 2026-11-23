---
id: file-crates-myrn-src-cmd-publish-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/publish.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/publish.rs
anchors:
- file:crates/myrn/src/cmd/publish.rs@2dc360febef52861
---
# crates/myrn/src/cmd/publish.rs

CLI handler for `myrn publish <id>`, which moves a private record into .myrn/knowledge/ only after showing a preview and getting confirmation, and commits nothing. Main parts: `run` (dry-run, confirmation, and publish flow), `rel` (display-path helper), and `show` (preview renderer that lists stripped and trimmed content).

## Connections

- **Location:** `crates/myrn/src/cmd/publish.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[publish-run]], [[publish-rel]], [[publish-show]]
- **Imports:** [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
