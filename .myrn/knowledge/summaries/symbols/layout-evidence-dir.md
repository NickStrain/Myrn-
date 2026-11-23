---
id: layout-evidence-dir
type: summary
status: unverified
title: Layout::evidence_dir
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::evidence_dir
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::evidence_dir@21da9dd1b3edf494
map_version: '2'
---
# Layout::evidence_dir

Returns the evidence directory in the per-user data dir.

## Behaviour

- -> <data_dir>/evidence
- created by create_dirs

## Terms

evidence, evidence store, evidence folder

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:75-77`
- **Signature:** `pub fn evidence_dir(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[layout-create-dirs]]
