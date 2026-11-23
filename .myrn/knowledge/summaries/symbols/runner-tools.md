---
id: runner-tools
type: summary
status: unverified
title: runner::Tools
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Tools
anchors:
- sym:bench/src/runner.rs#runner::Tools@56998dc148490e98
map_version: '2'
---
# runner::Tools

Settings shared by all runs: the claude and myrn binaries, the model, the permission mode, per-repo project maps, the shared store flag, and the embedding model directory.

## Behaviour

- model None -> the --model flag is omitted
- shared_store true -> one myrn store per repo and condition, reused across runs
- embeddings Some -> the model is copied in and semantic search is enabled

## Terms

binaries, executables, options, config, settings, permission mode, project map, embedding model

## Connections

- **Location:** `bench/src/runner.rs:101-113`
- **Signature:** `pub struct Tools`
- **In:** [[file-bench-src-runner-rs]]
