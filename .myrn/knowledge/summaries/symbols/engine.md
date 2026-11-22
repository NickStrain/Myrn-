---
id: engine
type: summary
status: unverified
title: engine
source: myrn summarize
about: sym:crates/myrn-mcp/tests/protocol.rs#engine
anchors:
- sym:crates/myrn-mcp/tests/protocol.rs#engine@5350abbcc078425b
---
# engine

Creates a temp git repo with src/lib.rs (doc-commented `inc` and a `two` that calls it), commits it, runs `Engine::init` with a temp home directory, and returns the TempDir (kept alive for the test) plus the Engine.

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs:7-27`
- **Signature:** `fn engine() -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-mcp-tests-protocol-rs]]
- **Calls:** [[engine-init]]
- **Tests:** `handshake_list_and_call`
