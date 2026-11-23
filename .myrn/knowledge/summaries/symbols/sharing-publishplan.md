---
id: sharing-publishplan
type: summary
status: unverified
title: sharing::PublishPlan
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::PublishPlan
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::PublishPlan@360e60d84efe3ebc
map_version: '2'
---
# sharing::PublishPlan

Holds the files publish would write and the warnings about links that would not resolve for others. It is the return value of publish_plan and publish.

## Behaviour

- record whose root cause is also private -> items has the record first, then the cause
- link to a note that stays private -> one warning string for that link
- built only, not written -> nothing changes on disk

## Terms

dry run, preview, publish preview, warnings, items, plan

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:31-35`
- **Signature:** `pub struct PublishPlan`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
