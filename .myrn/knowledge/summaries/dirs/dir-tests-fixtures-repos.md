---
id: dir-tests-fixtures-repos
type: summary
status: unverified
title: tests/fixtures/repos/
source: myrn summarize
about: module:tests/fixtures/repos
anchors:
- module:tests/fixtures/repos@4d91c85dfd0a7c42
---
# tests/fixtures/repos/

tests/fixtures/repos/ holds sample fixture repos for exercising code-map and agent tooling; they are not production code. gate0/ contains three parallel implementations of the same session-token refresh path: a Python package (py/), a Rust crate (src/ and tests/), and a TypeScript module (web/). Each looks a token up in an in-memory store and returns a new Session or nothing, and each test covers only the known token 'abc' success path.

## Connections

- **In:** [[dir-tests-fixtures]]
- **Directories:** [[dir-tests-fixtures-repos-gate0]]
