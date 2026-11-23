---
id: codex-failure-text
type: summary
status: unverified
title: codex::failure_text
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::failure_text
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::failure_text@8d69217a77df3421
map_version: '2'
---
# codex::failure_text

Returns error text for a tool response when it reports failure, otherwise None. It checks exit_code or exitCode first, then error, then success == false.

## Behaviour

- exit_code 101 with stderr 'boom' -> 'Exit code 101' then 'boom'
- exit_code 0 -> None, and error is not checked
- error null -> not a failure; success false -> the whole response as JSON text

## Terms

tool failure, non-zero exit, error message, command failed

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:130-153`
- **Signature:** `fn failure_text(resp: &Value) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-str-field]]
- **Called by:** [[codex-translate]]
