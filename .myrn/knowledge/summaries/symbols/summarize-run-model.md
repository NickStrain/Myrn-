---
id: summarize-run-model
type: summary
status: unverified
title: summarize::run_model
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::run_model
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::run_model@4c51e923cdf34fc6
---
# summarize::run_model

Spawns the configured model command in the temp directory (so repo hooks do not run), writes the prompt to stdin, and returns the answer text plus cost, unwrapping Claude Code's JSON `result` when present. It errors on a non-zero exit or an `is_error` result, and the command must be set in `.myrn/project.toml` under `[summarize]`.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:703-747`
- **Signature:** `fn run_model(command: &[String], prompt: &str) -> Result<(String, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[summarize-ask-json]]
