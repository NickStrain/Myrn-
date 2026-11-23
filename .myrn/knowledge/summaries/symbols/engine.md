---
id: engine
type: summary
status: unverified
title: engine
source: myrn summarize
about: sym:crates/myrn-mcp/tests/protocol.rs#engine
anchors:
- sym:crates/myrn-mcp/tests/protocol.rs#engine@5350abbcc078425b
map_version: '2'
---
# engine

Builds a temporary git repo with src/lib.rs (inc and two, with a doc comment on inc), commits it as 'first', and returns the TempDir with an Engine initialised on that repo.

## Behaviour

- no args -> (TempDir, Engine) for a repo with one commit 'first' and home dir under the temp dir
- the TempDir must stay alive; dropping it deletes the repo the Engine points at
- any filesystem, git or Engine::init failure panics through unwrap

## Terms

test fixture, temp repo, sample repository, git commit, test setup, indexed repo, temp dir

## Connections

- **Location:** `crates/myrn-mcp/tests/protocol.rs:7-27`
- **Signature:** `fn engine() -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-mcp-tests-protocol-rs]]
- **Calls:** [[engine-init]]
- **Tests:** `handshake_list_and_call`
