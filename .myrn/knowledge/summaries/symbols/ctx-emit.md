---
id: ctx-emit
type: summary
status: unverified
title: Ctx::emit
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::emit
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::Ctx::emit@1f74b44aa49b2324
---
# Ctx::emit

In json mode, pretty-prints value to stdout with serde_json. Otherwise it prints the result of text() with print!, which adds no newline, so the closure must supply one. It always returns Ok(ExitCode::SUCCESS), and text is only evaluated in non-json mode.

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:46-53`
- **Signature:** `pub fn emit<T: Serialize>(&self, value: &T, text: impl FnOnce() -> String) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Calls:** [[text]]
- **Called by:** [[debug-review]], [[doctor-run]], [[init-run]], [[log-run]], [[query-explain]], [[query-impact]], [[query-history]], [[query-context]], [[query-brief]], [[rebuild-run]], [[show-run]], [[status-run]], [[summarize-run]]
