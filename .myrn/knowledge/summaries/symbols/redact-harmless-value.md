---
id: redact-harmless-value
type: summary
status: unverified
title: redact::harmless_value
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::harmless_value
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::harmless_value@a9dde069e4d4fb24
map_version: '2'
---
# redact::harmless_value

Reports whether a value captured next to a sensitive name is not a secret. Surrounding quotes and whitespace are trimmed first.

## Behaviour

- 'true', 'null', 'bearer', 'token' -> true (keywords and auth scheme names)
- '1500' -> true (parses as a number); '$GITHUB_TOKEN', '${{ secrets.X }}', '<token>' -> true
- 'abc def' -> false; '***' or 'xxx' -> true (only * x X . characters); '[REDACTED:...' -> true

## Terms

false positive filter, placeholder value, environment reference, boolean or number value, empty value, already masked

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:140-164`
- **Signature:** `fn harmless_value(v: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
- **Called by:** [[redact-redact]]
