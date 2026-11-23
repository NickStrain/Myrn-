---
id: learn-usage
type: summary
status: unverified
title: learn::Usage
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Usage
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Usage@f52d5a342fef426e
map_version: '2'
---
# learn::Usage

Holds per-item counts of injections, uses, and uses in sessions that ended well. Only sessions with an attached outcome are counted.

## Behaviour

- Usage::default() -> injected 0, used 0, used_good 0; factor() on it returns 1.0
- used_good is a subset of used: a use counts there only if its session outcome was tests_passed or committed
- serialized with serde, so it can be emitted as JSON for status output

## Terms

injection count, usage history, hit counts, ignored items, helpfulness stats, past use, good session use

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:29-34`
- **Signature:** `pub struct Usage`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
