---
id: file-crates-myrn-store-src-state-rs
type: summary
status: unverified
title: crates/myrn-store/src/state.rs
source: myrn summarize
about: file:crates/myrn-store/src/state.rs
anchors:
- file:crates/myrn-store/src/state.rs@65fc709a7eb965e9
---
# crates/myrn-store/src/state.rs

Private per-project state stored as state.json beside memory.db, outside the repo, so it survives myrn rebuild. It defines the State struct (consent, connected agents, hook timings, embeddings flag), the AgentState and HookTiming structs, the load, save and record_timing methods on State, and one round-trip unit test.

## Connections

- **Location:** `crates/myrn-store/src/state.rs`
- **In:** [[dir-crates-myrn-store-src]]
- **Defines:** [[state-state]], [[state-agentstate]], [[state-hooktiming]], [[state-load]], [[state-save]], [[state-record-timing]]
- **Imported by:** [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-src-cmd-connect-rs]], [[file-crates-myrn-src-cmd-hook-rs]]
