---
id: used
type: summary
status: unverified
title: used
source: myrn summarize
about: sym:crates/myrn-engine/tests/learning.rs#used
anchors:
- sym:crates/myrn-engine/tests/learning.rs#used@2805cb6e34eaf695
map_version: '2'
---
# used

Returns the sum of the used column in the usage table for one note id and one session. Reads the database directly with SQL and does not call the engine.

## Behaviour

- no usage rows for that id and session -> 0 (coalesce)
- one row with used = 1 -> 1
- rows from other sessions or other note ids are not counted

## Terms

usage count, times used, use count, session usage, marked used, read usage

## Connections

- **Location:** `crates/myrn-engine/tests/learning.rs:46-55`
- **Signature:** `fn used(e: &Engine, id: &str, session: &str) -> i64`
- **In:** [[file-crates-myrn-engine-tests-learning-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `opening_the_file_or_asking_about_it_counts_as_use`
