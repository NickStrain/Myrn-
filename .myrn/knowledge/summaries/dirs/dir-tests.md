---
id: dir-tests
type: summary
status: unverified
title: tests/
source: myrn summarize
about: module:tests
anchors:
- module:tests@908ac947348a26a5
---
# tests/

Test fixtures for the code-map and agent tooling, not production code. tests/fixtures/repos/gate0/ holds three parallel implementations of one session-token refresh path (Python in py/, Rust in src/ and tests/, TypeScript in web/), each looking up a token in an in-memory store and returning a new Session or nothing; the tests only cover the known token 'abc' success path.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-tests-fixtures]]
