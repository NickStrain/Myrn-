---
id: append
type: summary
status: unverified
title: append
source: myrn summarize
about: sym:bench/src/main.rs#append
anchors:
- sym:bench/src/main.rs#append@ee5ba506149ca588
map_version: '2'
---
# append

Adds one run record as a single JSON line to the end of a results file, creating the file if needed.

## Behaviour

- file missing -> created, containing one line for the record
- file exists -> new line appended, earlier lines kept
- serialization failure -> error returned and nothing is written

## Terms

write record, jsonl, results file, log line, append line, save run

## Connections

- **Location:** `bench/src/main.rs:508-515`
- **Signature:** `fn append(path: &Path, rec: &RunRecord) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Called by:** [[real-main]]
