---
id: fixture-gate0
type: summary
status: unverified
title: Fixture
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#Fixture
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#Fixture@d583ae746c828d11
map_version: '2'
---
# Fixture

Holds one test setup: the TempDir guard, the repo root, the home dir, and the Engine built on them.

## Behaviour

- _tmp field keeps the temp directory alive until the Fixture is dropped
- root is tmp/repo and home is tmp/home
- engine is the Engine returned by Engine::init

## Terms

test context, temp repo, test state, engine handle

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:46-51`
- **Signature:** `struct Fixture`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
