---
id: status-as-str
type: summary
status: unverified
title: Status::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::Status::as_str
anchors:
- sym:crates/myrn-model/src/record.rs#record::Status::as_str@187b793eb215b2ad
map_version: '2'
---
# Status::as_str

Returns the lowercase string name of a Status variant as a static str.

## Behaviour

- Status::Verified -> 'verified'
- Status::Superseded -> 'superseded'
- Output matches the serde snake_case form, so it is safe to use as a stored or displayed value

## Terms

state label, status string, lowercase name, display text, serialized value

## Connections

- **Location:** `crates/myrn-model/src/record.rs:23-30`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-record-rs]]
- **Called by:** [[symbols-resolve-all]], [[note-new]], [[frontmatter-parse]], [[sync-link]]
- **Tests:** `parses_the_design_example`
