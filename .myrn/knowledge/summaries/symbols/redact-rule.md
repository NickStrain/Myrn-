---
id: redact-rule
type: summary
status: unverified
title: redact::Rule
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::Rule
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::Rule@eb700e5832bdea7f
map_version: '2'
---
# redact::Rule

Pairs a redaction kind label with its compiled regex for the token table. It is private to this module.

## Behaviour

- kind is passed to mark() when a match is replaced
- re is compiled once by rule() when the static TOKENS table is first used

## Terms

compiled pattern, token rule, kind label, matcher

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:19-22`
- **Signature:** `struct Rule`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
