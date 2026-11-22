---
id: status-as-str
type: summary
status: unverified
title: Status::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Status::as_str
anchors:
- sym:crates/myrn-model/src/record.rs#record::Status::as_str@187b793eb215b2ad
---
# Status::as_str

Returns the lowercase name of a status (for example Verified to "verified") using an exhaustive match, so a missing arm is a compile error.

## Connections

- **Location:** `crates/myrn-model/src/record.rs:23-30`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[symbols-resolve-all]], [[note-new]], [[frontmatter-parse]]
- **Tests:** `parses_the_design_example`
