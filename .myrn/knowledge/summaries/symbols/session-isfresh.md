---
id: session-isfresh
type: summary
status: unverified
title: Session::isFresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.isFresh
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.isFresh@08ddb3d71798a12b
---
# Session::isFresh

Returns true when issued + 10 > now(), so the window is 10 in whatever unit now() returns. A newly built session is fresh unless the clock advances 10 or more before the check.

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:10-12`
- **Signature:** `isFresh(): boolean`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Calls:** [[now]]
- **Called by:** [[refresh]]
