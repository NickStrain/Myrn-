---
id: ctx-engine
type: summary
status: unverified
title: Ctx::engine
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::engine
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::engine@945e7acd19675fd6
map_version: '2'
---
# Ctx::engine

Opens the engine for the current working directory and refreshes the index before returning it, so every subcommand works on up-to-date data.

## Behaviour

- Every call re-runs refresh() on the index, even if nothing changed
- Unreadable current directory -> the io error is returned via ?
- data_dir Some(p) -> index stored under p; None -> engine default location

## Terms

open project, load engine, index refresh, update index, project index, startup

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:39-44`
- **Signature:** `pub fn engine(&self) -> Result<Engine>`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Calls:** [[engine-open]], [[engine-refresh]]
- **Called by:** [[debug-review]], [[diff-run]], [[embeddings-enable]], [[embeddings-disable]], [[embeddings-status]], [[forget-run]], [[gc-run]], [[log-run]], [[mcp-run]], [[publish-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[show-run]], [[status-run]], [[summarize-run]], [[trace-run]], [[ui-run]], [[verify-run]]
