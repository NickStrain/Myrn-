---
id: engine-recall-text
type: summary
status: unverified
title: Engine::recall_text
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall_text
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::Engine::recall_text@7c899e0c0b9e8f2d
map_version: '2'
---
# Engine::recall_text

Runs recall with limit 3 and renders hits as a '>' quoted block into a token Budget, returning the text and the ids of hits that fit; empty output when nothing matches.

## Behaviour

- no hits -> ('', [])
- similarity 0.999 or higher is labelled 'same signature', otherwise 'NN% alike'; fix line is the first body line starting 'Fix: ', clipped to 300 chars
- header is pushed without a fit check; if no hit fits, returns ('', []) even though the header was pushed

## Terms

recall text, token budget, quoted reference, prompt context, same signature, percent alike, fix line, cause excerpt

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:107-153`
- **Signature:** `pub fn recall_text(&self, error: &str, budget: usize) -> Result<(String, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[budget-new]], [[budget-push]], [[engine-recall]], [[recall-clip]], [[recall-first-paragraph]]
- **Called by:** [[hook-handle]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
