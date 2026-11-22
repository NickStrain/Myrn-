---
id: engine-log-injection
type: summary
status: unverified
title: Engine::log_injection
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::log_injection
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::log_injection@add5d37a9c8813e9
---
# Engine::log_injection

Records which items were injected into a session in the usage log.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:819-821`
- **Signature:** `pub fn log_injection(&self, session: &str, items: &[String]) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[log-record]]
- **Called by:** [[context-call]], [[recall-call]], [[hook-handle]]
