---
id: runner-condition
type: summary
status: unverified
title: runner::Condition
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition
anchors:
- sym:bench/src/runner.rs#runner::Condition@c072b42ce275af3a
map_version: '2'
---
# runner::Condition

The four experiment arms: no myrn, myrn with injection off, myrn with injection on, and injection on with a fixed ranking. Serialized in snake_case.

## Behaviour

- Without -> "without" in JSON; Fixed -> "fixed"
- Deserialization uses the same snake_case names that parse accepts
- Copy and comparable, so it can be used as a map key or compared with ==

## Terms

arm, variant, experiment group, treatment, mode

## Connections

- **Location:** `bench/src/runner.rs:26-34`
- **Signature:** `pub enum Condition`
- **In:** [[file-bench-src-runner-rs]]
