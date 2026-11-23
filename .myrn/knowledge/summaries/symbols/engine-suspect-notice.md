---
id: engine-suspect-notice
type: summary
status: unverified
title: Engine::suspect_notice
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::Engine::suspect_notice
anchors:
- sym:crates/myrn-engine/src/validity/anchors.rs#validity::anchors::Engine::suspect_notice@070adf25aba9a93e
map_version: '2'
---
# Engine::suspect_notice

Queries anchors on earned suspect notes that have a symbol, keeps those whose symbol's file is in `files` and whose id is not in `seen`, and renders them as quoted blocks with drift lines under the header and ask text, within `budget`. Returns the text and the ids shown, and writes nothing.

## Behaviour

- empty `files` -> ("", []) without querying the database
- anchors with no symbol_id are never selected, so file: anchors do not appear here even though anchored_path parses them
- a block that does not fit the budget is dropped, and if no block fits or the header does not fit, returns ("", [])

## Terms

stale record, outdated note, drift, suspect status, earned note, anchor hash mismatch, budget-limited notice, reminder text

## Connections

- **Location:** `crates/myrn-engine/src/validity/anchors.rs:23-84`
- **Signature:** `pub fn suspect_notice( &self, files: &[String], seen: &HashSet<String>, budget: usize, ) -> Result<(String, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-validity-anchors-rs]]
- **Calls:** [[budget-new]], [[budget-push]], [[render-quoted]], [[anchors-anchored-path]], [[anchors-short-name]], [[graph-node-graph]], [[sync-drift-sync]]
- **Called by:** [[hook-handle]]
- **Tests:** `suspect_records_are_labelled_ranked_down_and_shown_where_the_agent_works`
