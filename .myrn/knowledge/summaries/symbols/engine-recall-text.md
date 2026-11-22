---
id: engine-recall-text
type: summary
status: unverified
title: Engine::recall_text
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall_text
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall_text@a6014485897f3010
---
# Engine::recall_text

Calls recall for the top 3 hits and renders them as a '>'-quoted reference block within the given token budget, returning the text and the ids actually included; returns empty output when there are no matches or none fit the budget.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:102-150`
- **Signature:** `pub fn recall_text(&self, error: &str, budget: usize) -> Result<(String, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[budget-new]], [[budget-push]], [[engine-recall]], [[recall-clip]], [[recall-first-paragraph]]
- **Called by:** [[hook-handle]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
