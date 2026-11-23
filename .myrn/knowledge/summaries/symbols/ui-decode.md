---
id: ui-decode
type: summary
status: unverified
title: ui::decode
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::decode
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::decode@754d5615577a21fb
map_version: '2'
---
# ui::decode

Percent-decodes a path or query component, turning '+' into a space and each valid %XX into a byte, and returns the result as lossy UTF-8.

## Behaviour

- 'a+b' -> 'a b' and 'sym%3Afoo' -> 'sym:foo'; '+' is also a space inside the path
- malformed or truncated escapes such as '%zz', '100%' or a trailing '%4' stay literal
- invalid UTF-8 bytes become replacement characters

## Terms

percent encoding, url decode, unescape, query value, plus sign, hex escape, url escape

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:212-234`
- **Signature:** `fn decode(s: &str) -> String`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Called by:** [[ui-read-request]]
- **Tests:** `decodes_queries`
