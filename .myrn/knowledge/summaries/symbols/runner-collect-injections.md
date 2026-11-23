---
id: runner-collect-injections
type: summary
status: unverified
title: runner::collect_injections
source: myrn summarize
about: sym:bench/src/runner.rs#runner::collect_injections
anchors:
- sym:bench/src/runner.rs#runner::collect_injections@085041a1ee4f42dc
map_version: '2'
---
# runner::collect_injections

Reads the usage table for this session in the myrn store and fills injected, injected_used, injected_records, injected_stale and hook_ms in the record.

## Behaviour

- injected_used counts an item if its file is in files_seen, or its name (4+ characters, case-insensitive) appears in the answer
- injected_records and injected_stale count only earned records; stale means usage status suspect
- hook_ms keeps UserPromptSubmit timings at or after rec.started

## Terms

injections, injected notes, usage, hit rate, used items, stale records, hook latency, prompt hook

## Connections

- **Location:** `bench/src/runner.rs:649-685`
- **Signature:** `fn collect_injections(work: &Path, home: &Path, rec: &mut RunRecord) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-item-name]], [[runner-item-path]], [[engine-open]]
- **Called by:** [[runner-run-inner]]
