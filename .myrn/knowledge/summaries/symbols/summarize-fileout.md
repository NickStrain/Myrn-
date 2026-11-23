---
id: summarize-fileout
type: summary
status: unverified
title: summarize::FileOut
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::FileOut
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::FileOut@b5d243800cb81d05
map_version: '2'
---
# summarize::FileOut

Parsed model answer for one file job: the file summary (None if absent), symbol answers keyed by the name the model returned, and the summed call cost.

## Behaviour

- symbol keys are the model's own names; summarize matches them to nodes by qualified name, then by bare title if unique
- cost includes a retry: ask_json adds the cost of both calls
- file is None when the 'file' key is missing or not a string

## Terms

file answer, model output, per-file result, call cost, symbol map

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:680-684`
- **Signature:** `struct FileOut`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
