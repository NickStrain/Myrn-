---
id: dir-tests-fixtures
type: summary
status: unverified
title: tests/fixtures/
source: myrn summarize
about: module:tests/fixtures
anchors:
- module:tests/fixtures@ccfcf1c1306513d9
---
# tests/fixtures/

tests/fixtures/ holds sample code used to test code-map and agent tooling, not production code. Its repos/gate0/ subfolder contains three parallel implementations of one session-token refresh path (Python in py/, Rust in src/ and tests/, TypeScript in web/), each looking a token up in an in-memory store and returning a new Session or nothing, with tests covering only the known token 'abc' success path.

## Connections

- **In:** [[dir-tests]]
- **Directories:** [[dir-tests-fixtures-repos]]
