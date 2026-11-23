---
id: summarize-run-model
type: summary
status: unverified
title: summarize::run_model
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::run_model
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::run_model@4c51e923cdf34fc6
map_version: '2'
---
# summarize::run_model

Runs the model command with the prompt on stdin in the system temp directory and returns the answer text and its cost, unwrapping Claude Code JSON output when present.

## Behaviour

- non-zero exit -> error with stderr (or stdout if stderr is empty), clipped to 300 chars
- stdout JSON with a result string -> that text and total_cost_usd; is_error true -> error 'model error'
- other stdout -> raw stdout with cost 0.0; running outside the repo keeps project hooks from loading

## Terms

model call, subprocess, stdin prompt, claude code output, model cost, command runner

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:790-834`
- **Signature:** `fn run_model(command: &[String], prompt: &str) -> Result<(String, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[summarize-ask-json]]
