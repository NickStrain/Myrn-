---
id: env
type: summary
status: unverified
title: Env
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env
anchors:
- sym:crates/myrn/tests/connect.rs#Env@3ae97699480c4a9a
map_version: '2'
---
# Env

Test fixture holding the temp directory, the repo path, and the MYRN_HOME path for one test run.

## Behaviour

- repo is tmp/repo and home is tmp/home, both under the TempDir
- _tmp is never read, but dropping Env deletes the whole temp tree, so it must stay alive
- only setup() constructs it

## Terms

test fixture, temp repo, sandbox, temp directory, MYRN_HOME, test home

## Connections

- **Location:** `crates/myrn/tests/connect.rs:11-15`
- **Signature:** `struct Env`
- **In:** [[file-crates-myrn-tests-connect-rs]]
