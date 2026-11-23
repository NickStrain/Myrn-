---
id: dir-crates-myrn-engine-tests
type: summary
status: unverified
title: crates/myrn-engine/tests/
source: myrn summarize
about: module:crates/myrn-engine/tests
anchors:
- module:crates/myrn-engine/tests@cea2d475037fb31c
---
# crates/myrn-engine/tests/

Integration tests for the myrn-engine crate. Each file covers one feature area through the public Engine API, using temporary git repos or the multi-language gate0 fixture: explain and impact call graphs, history, the learning loop, note indexing, context packs, the record tool and distiller, Git-based record sharing, summaries, validity tracking, and the data behind 'myrn ui' (viz). Helpers are defined per file rather than in a shared module, and each test builds its own fixture repo.

## Connections

- **In:** [[dir-crates-myrn-engine]]
- **Files:** [[file-crates-myrn-engine-tests-gate0-rs]], [[file-crates-myrn-engine-tests-history-rs]], [[file-crates-myrn-engine-tests-learning-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-engine-tests-packing-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-engine-tests-sharing-rs]], [[file-crates-myrn-engine-tests-summarize-rs]], [[file-crates-myrn-engine-tests-validity-rs]], [[file-crates-myrn-engine-tests-viz-rs]]
