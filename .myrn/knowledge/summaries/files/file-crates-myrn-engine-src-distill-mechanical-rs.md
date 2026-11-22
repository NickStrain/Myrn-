---
id: file-crates-myrn-engine-src-distill-mechanical-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/mechanical.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/mechanical.rs
anchors:
- file:crates/myrn-engine/src/distill/mechanical.rs@ad6b4497b5254896
---
# crates/myrn-engine/src/distill/mechanical.rs

Module that provides a model-free episode summary, used when no agent distillation runs. It has no logic of its own; its only content is a re-export of `Episode` from `capture::episode`, whose `summary` method holds the mechanical record of files, symbols, commands, errors, test outcome and commits.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mechanical.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Imports:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]]
