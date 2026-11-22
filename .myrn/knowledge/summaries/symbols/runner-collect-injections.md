---
id: runner-collect-injections
type: summary
status: unverified
title: runner::collect_injections
source: myrn summarize
about: sym:bench/src/runner.rs#runner::collect_injections
anchors:
- sym:bench/src/runner.rs#runner::collect_injections@11c3d1d52e83d571
---
# runner::collect_injections

Opens the myrn engine in work and reads the distinct note_id values with injected = 1 into rec.injected. It counts an injected item as used when its file is in session.files_seen or its name (at least 4 characters) appears in the answer, and it copies UserPromptSubmit hook timings into hook_ms.

## Connections

- **Location:** `bench/src/runner.rs:504-530`
- **Signature:** `fn collect_injections(work: &Path, home: &Path, rec: &mut RunRecord) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-item-name]], [[runner-item-path]], [[engine-open]]
- **Called by:** [[runner-run-inner]]
