---
id: recordoutcome-render
type: summary
status: unverified
title: RecordOutcome::render
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::render
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::render@a317bc622246251e
map_version: '2'
---
# RecordOutcome::render

Formats the outcome as plain text for the agent: a success line naming the ids, or a rejection header, followed by each message as a bullet.

## Behaviour

- accepted with ids ['a','b'] -> 'Recorded a, b (status unverified until an outcome backs it).' then a newline.
- updated true -> the success line starts 'Updated existing record' instead of 'Recorded'.
- Not accepted -> 'Rejected; nothing was stored. Fix these and call record again:' then one '- message' line each. Messages also get bullets when accepted.

## Terms

tool reply, rendered text, success message, rejection message, bullet list, status line

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:121-139`
- **Signature:** `pub fn render(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
