---
id: kind-as-str
type: summary
status: unverified
title: Kind::as_str
source: myrn summarize
about: sym:bench/src/task.rs#task::Kind::as_str
anchors:
- sym:bench/src/task.rs#task::Kind::as_str@472dd1cd3085d703
---
# Kind::as_str

Takes a Kind by value and returns its lowercase name as a static str: bugfix, feature, question, or replay. The shipped-task test uses it to check that task ids start with repo/kind/.

## Connections

- **Location:** `bench/src/task.rs:31-38`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[report-render]], [[redact-redact]], [[engine-check-evidence]], [[engine-record]], [[indexer-persist]], [[sym-callable]], [[resolver-parent-is-type]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
