---
id: ctx-emit
type: summary
status: unverified
title: Ctx::emit
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::emit
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::emit@1f74b44aa49b2324
map_version: '2'
---
# Ctx::emit

Writes a serializable value as pretty JSON when json is set, otherwise prints the text produced by the closure, and always returns success.

## Behaviour

- json true -> println of pretty JSON, text closure is never called
- json false -> print! of text() with no trailing newline added
- Serialization failure -> error returned; the Ok(ExitCode::SUCCESS) path is skipped

## Terms

output, print, render, display, JSON output, text output, format

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:46-53`
- **Signature:** `pub fn emit<T: Serialize>(&self, value: &T, text: impl FnOnce() -> String) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Calls:** [[text]]
- **Called by:** [[debug-review]], [[diff-run]], [[doctor-run]], [[embeddings-enable]], [[embeddings-disable]], [[embeddings-status]], [[forget-run]], [[gc-run]], [[init-run]], [[log-run]], [[publish-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[rebuild-run]], [[show-run]], [[status-run]], [[summarize-run]], [[trace-run]], [[verify-run]]
