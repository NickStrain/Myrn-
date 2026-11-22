---
id: file-crates-myrn-engine-src-summarize-rs
type: summary
status: unverified
title: crates/myrn-engine/src/summarize.rs
source: myrn summarize
about: file:crates/myrn-engine/src/summarize.rs
anchors:
- file:crates/myrn-engine/src/summarize.rs@25e3e61ac8fb9761
---
# crates/myrn-engine/src/summarize.rs

Implements `myrn summarize`, which builds the project map: it asks a model (default Haiku via the Claude Code CLI) for a summary of the project, each directory, each code file, and each function/type, and writes them as Obsidian notes under `.myrn/knowledge/summaries/` keyed to a body hash so only changed code is re-asked. Main parts are `Engine::summarize` (staleness detection, parallel file and directory jobs, note writing and cleanup), `Engine::map_body` (note body and link rendering), and the model-call helpers (`summarize_file`, `summarize_dir`, `run_model`, `ask_json`, `parse_json`), plus `slug`, `clean`, and `parallel`.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Defines:** [[summarize-options]], [[summarize-report]], [[summarize-entry]], [[summarize-existing]], [[engine-summarize]], [[engine-map-body]], [[summarize-filejob]], [[summarize-fileout]], [[summarize-dirjob]], [[summarize-summarize-file]], [[summarize-summarize-dir]], [[summarize-run-model]], [[summarize-ask-json]], [[summarize-parse-json]], [[summarize-parallel]], [[summarize-readme-excerpt]], [[summarize-title-of]], [[summarize-display-dir]], [[summarize-clean]], [[summarize-slug]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-tests-summarize-rs]], [[file-crates-myrn-src-cmd-summarize-rs]]
