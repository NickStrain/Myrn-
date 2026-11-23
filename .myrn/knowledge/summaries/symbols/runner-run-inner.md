---
id: runner-run-inner
type: summary
status: unverified
title: runner::run_inner
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_inner
anchors:
- sym:bench/src/runner.rs#runner::run_inner@c021afab3d6a235f
map_version: '2'
---
# runner::run_inner

Runs one trial: prepares the clone, writes the MCP config, runs headless Claude Code with the task prompt, parses its metrics, then collects injections and exports records when myrn is in use, and sets success from the answer check or the hidden check.

## Behaviour

- Missing checkout -> error telling the user to run myrn-bench fetch
- Timeout does not return an error; timed_out is set and the run is still scored
- Agent errored with zero tokens -> error "the agent did no work"; no session id -> error that points to the raw log

## Terms

orchestrate, trial, pipeline, headless claude, execute run, mcp config, session metrics, scoring

## Connections

- **Location:** `bench/src/runner.rs:337-458`
- **Signature:** `fn run_inner( spec: &RunSpec, tools: &Tools, paths: &Paths, raw_dir: &Path, rec: &mut RunRecord, ) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[tokens-total]], [[metrics-parse]], [[metrics-answer-ok]], [[condition-as-str]], [[condition-uses-myrn]], [[runner-prepare]], [[runner-run-check]], [[runner-run-with-timeout]], [[runner-tail]], [[runner-setup-myrn]], [[runner-export-records]], [[runner-collect-injections]]
- **Called by:** [[runner-run-one]]
