---
id: runner-run-inner
type: summary
status: unverified
title: runner::run_inner
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_inner
anchors:
- sym:bench/src/runner.rs#runner::run_inner@117e62d3574a57c2
---
# runner::run_inner

Performs a full run. It prepares the workspace, writes the MCP config (empty for Without, otherwise a myrn stdio server with MYRN_HOME after setup_myrn), runs claude -p with stream-json output and local-only settings, and saves the raw stream under raw_dir. It errors if the API refused the session with zero tokens or no session id appears. For myrn conditions it then collects injections and exports records, and it scores Question tasks by answer match and other tasks by the check.

## Connections

- **Location:** `bench/src/runner.rs:317-420`
- **Signature:** `fn run_inner( spec: &RunSpec, tools: &Tools, paths: &Paths, raw_dir: &Path, rec: &mut RunRecord, ) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[tokens-total]], [[metrics-parse]], [[metrics-answer-ok]], [[condition-as-str]], [[condition-uses-myrn]], [[runner-prepare]], [[runner-run-check]], [[runner-run-with-timeout]], [[runner-tail]], [[runner-setup-myrn]], [[runner-export-records]], [[runner-collect-injections]]
- **Called by:** [[runner-run-one]]
