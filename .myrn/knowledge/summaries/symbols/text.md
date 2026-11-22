---
id: text
type: summary
status: unverified
title: text
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#text
anchors:
- sym:crates/myrn/tests/connect.rs#text@f8efc469575dc337
---
# text

Joins an Output's stdout and stderr into one lossy UTF-8 String so tests can match on either stream.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:89-95`
- **Signature:** `fn text(o: &Output) -> String`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Called by:** [[ctx-emit]]
- **Tests:** `connect_hooks_doctor_disconnect`, `sessions_are_recorded_redacted_and_listed`
