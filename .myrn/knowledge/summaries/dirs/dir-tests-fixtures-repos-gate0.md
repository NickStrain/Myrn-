---
id: dir-tests-fixtures-repos-gate0
type: summary
status: unverified
title: tests/fixtures/repos/gate0/
source: myrn summarize
about: module:tests/fixtures/repos/gate0
anchors:
- module:tests/fixtures/repos/gate0@3f81322eaa4f4edb
---
# tests/fixtures/repos/gate0/

Sample fixture repo for code-map and agent tooling, not production code. Three parallel implementations of a session-token refresh path (a Python package in py/, a Rust crate in src/ and tests/, and a TypeScript module in web/) each look a token up in an in-memory store and return a new Session or nothing. Each test covers only the known token 'abc' success path, and the Rust and TypeScript now() functions are hardcoded to return 42.

## Connections

- **In:** [[dir-tests-fixtures-repos]]
- **Directories:** [[dir-tests-fixtures-repos-gate0-py]], [[dir-tests-fixtures-repos-gate0-src]], [[dir-tests-fixtures-repos-gate0-tests]], [[dir-tests-fixtures-repos-gate0-web]]
