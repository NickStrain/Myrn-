---
id: codex-failure-text
type: summary
status: unverified
title: codex::failure_text
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::failure_text
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::failure_text@8d69217a77df3421
---
# codex::failure_text

Returns an error message when a tool response indicates failure, and None otherwise. A nonzero exit_code or exitCode yields 'Exit code N' followed by stderr or output, a non-null error is returned as text, and success false returns the whole response. Codex documents no failure event, so this inference is the only failure signal.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:130-153`
- **Signature:** `fn failure_text(resp: &Value) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[adapter-str-field]]
- **Called by:** [[codex-translate]]
