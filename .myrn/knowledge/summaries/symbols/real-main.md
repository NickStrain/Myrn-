---
id: real-main
type: summary
status: unverified
title: real_main
source: myrn summarize
about: sym:bench/src/main.rs#real_main
anchors:
- sym:bench/src/main.rs#real_main@7dc36b8d29a517b9
---
# real_main

Parses the CLI, loads repos from bench-dir/repos and tasks from bench-dir/tasks, resolves checkouts to an absolute path, then dispatches on the subcommand. For Run it requires a myrn binary (default bench-dir/../target/release/myrn, else it bails), writes raw transcripts to out/raw, appends each RunRecord to out/results.jsonl, flips condition order on even run numbers, deletes out/work at the end, and calls write_report; Validate fails the exit code if any non-question task has a bad check.

## Connections

- **Location:** `bench/src/main.rs:108-368`
- **Signature:** `fn real_main() -> Result<ExitCode>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[append]], [[rescore-runs]], [[write-report]], [[absolute]], [[indent]], [[tokens-total]], [[condition-parse]], [[runner-run-one]], [[runner-now]], [[runner-validate]], [[task-load-repos]], [[task-load-tasks]], [[review-sample]], [[review-checklist]], [[review-score-review]]
- **Called by:** [[main]]
