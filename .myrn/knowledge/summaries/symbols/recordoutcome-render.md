---
id: recordoutcome-render
type: summary
status: unverified
title: RecordOutcome::render
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::render
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::render@a317bc622246251e
---
# RecordOutcome::render

Formats the outcome as plain text for the agent. On accept it says 'Recorded' or 'Updated existing record', lists the ids, and notes the status is unverified. On reject it says nothing was stored and asks for a retry. Each message follows as a '- ' line.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:121-139`
- **Signature:** `pub fn render(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
