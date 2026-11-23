---
id: file-bench-src-retrieval-rs
type: summary
status: unverified
title: bench/src/retrieval.rs
source: myrn summarize
about: file:bench/src/retrieval.rs
anchors:
- file:bench/src/retrieval.rs@06a23bfab671cc9f
---
# bench/src/retrieval.rs

Implements the `myrn-bench retrieval` mode, which runs `myrn context` on each task's prompt and checks whether the packed items point at the right code (the task's `touches` files for code tasks, `answer_contains` terms for questions), without running an agent. Contains the Row result struct, item_path, one (the per-task run), and render (the markdown table and summary).

## Connections

- **Location:** `bench/src/retrieval.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[retrieval-row]], [[retrieval-item-path]], [[retrieval-one]], [[retrieval-render]]
- **Imports:** [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
- **Imported by:** [[file-bench-src-main-rs]]
