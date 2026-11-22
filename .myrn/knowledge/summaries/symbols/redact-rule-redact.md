---
id: redact-rule-redact
type: summary
status: unverified
title: redact::rule
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::rule
anchors:
- sym:crates/myrn-engine/src/capture/redact.rs#capture::redact::rule@31f5a2c34d3c12ab
---
# redact::rule

Builds a Rule from a kind label and a pattern string, compiling the regex with `Regex::new` and panicking if the pattern is invalid.

## Connections

- **Location:** `crates/myrn-engine/src/capture/redact.rs:24-29`
- **Signature:** `fn rule(kind: &'static str, re: &str) -> Rule`
- **In:** [[file-crates-myrn-engine-src-capture-redact-rs]]
