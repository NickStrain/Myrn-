---
id: file-crates-myrn-engine-src-distill-record-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/record.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/record.rs
anchors:
- file:crates/myrn-engine/src/distill/record.rs@d78e8f6efa83ad59
---
# crates/myrn-engine/src/distill/record.rs

Turns a proposed record (bug, decision, workflow, fact or summary) into a stored note. It validates the record, resolves its references, merges it into a matching existing note, writes the note file and syncs the index. It also builds the end-of-turn prompt that asks the agent to record what it learned.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[engine-record-json]], [[engine-record-any-json]], [[engine-record]], [[engine-put-note]], [[engine-note-file]], [[engine-current-session]], [[engine-records-request]], [[engine-summary-is-current]], [[engine-summary-outdated]], [[record-link]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-episode-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-paths-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]]
