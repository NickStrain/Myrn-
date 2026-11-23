---
id: file-crates-myrn-engine-src-validity-anchors-rs
type: summary
status: unverified
title: crates/myrn-engine/src/validity/anchors.rs
source: myrn summarize
about: file:crates/myrn-engine/src/validity/anchors.rs
anchors:
- file:crates/myrn-engine/src/validity/anchors.rs@25ffd948f8ccd73c
---
# crates/myrn-engine/src/validity/anchors.rs

Builds the notice that shows suspect records (earned notes whose anchored code has changed since they were written) when an agent works in a file those records describe. Contains the header and ask constants, `Engine::suspect_notice`, and the helpers `anchored_path` and `short_name` that parse anchor ids.

## Connections

- **Location:** `crates/myrn-engine/src/validity/anchors.rs`
- **In:** [[dir-crates-myrn-engine-src-validity]]
- **Defines:** [[engine-suspect-notice]], [[anchors-anchored-path]], [[anchors-short-name]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-validity-mod-rs]]
