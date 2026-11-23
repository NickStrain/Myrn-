---
id: metrics-parse
type: summary
status: unverified
title: metrics::parse
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::parse
anchors:
- sym:bench/src/metrics.rs#metrics::parse@b11045000264239c
map_version: '2'
---
# metrics::parse

Reads a stream-json transcript line by line and fills a Session; root makes absolute paths repo-relative and touches names the files where the correct edit lands.

## Behaviour

- Lines that are not valid JSON are skipped silently
- Empty touches -> the first edit of any file counts as the correct edit; otherwise an edit counts if its path equals a touch or a Bash command contains the touch string
- After a user Stop-hook message that starts 'Stop hook feedback:' and contains 'record what this session learned', later assistant usage goes to record_tokens (max per message id) and the answer stops updating; the final result event overwrites tokens with its usage

## Terms

transcript parsing, stream-json, exploration before edit, session metrics, stop hook recording, answer extraction, token accounting

## Connections

- **Location:** `bench/src/metrics.rs:100-252`
- **Signature:** `pub fn parse(stream: &str, root: &str, touches: &[String]) -> Session`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-is-exploration]]
- **Called by:** [[rescore-runs]], [[runner-run-inner]]
- **Tests:** `parses_a_transcript`, `shell_reads_and_edits_count`, `recording_phase_is_split_off`
