---
id: metrics-session
type: summary
status: unverified
title: metrics::Session
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Session
anchors:
- sym:bench/src/metrics.rs#metrics::Session@22d42451a589bf6f
map_version: '2'
---
# metrics::Session

Holds the metrics parsed from one session: tokens, cost, duration, turns, tool and edit counts, exploration counts, files touched, the final answer and the record-phase fields.

## Behaviour

- Default -> counts 0, answer empty, exploration_before_edit None
- record_* fields and asked_to_record stay at defaults unless a Stop-hook record request was seen
- shell_edits, asked_to_record, record_tokens, record_calls and record_rejected default when missing from serialized data

## Terms

session result, run metrics, transcript summary, per-run stats, record phase

## Connections

- **Location:** `bench/src/metrics.rs:26-59`
- **Signature:** `pub struct Session`
- **In:** [[file-bench-src-metrics-rs]]
