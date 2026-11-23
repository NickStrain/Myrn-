---
id: kind-as-str
type: summary
status: unverified
title: Kind::as_str
source: myrn summarize
about: sym:bench/src/task.rs#task::Kind::as_str
anchors:
- sym:bench/src/task.rs#task::Kind::as_str@472dd1cd3085d703
map_version: '2'
---
# Kind::as_str

Returns the lowercase name of a Kind as a static string. The result matches the YAML value and the second segment of a task id.

## Behaviour

- Kind::Feature returns the string feature.
- Used by the shipped-task test to check that each id starts with repo/kind/.
- Returns a static string, so it does not allocate.

## Terms

kind name, string form, label, id prefix, kind string

## Connections

- **Location:** `bench/src/task.rs:31-38`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[report-render]], [[retrieval-one]], [[redact-redact]], [[engine-check-evidence]], [[engine-record]], [[indexer-persist]], [[sym-callable]], [[resolver-parent-is-type]], [[engine-trace]], [[engine-verified-by-test]], [[show-run]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
