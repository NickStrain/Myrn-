---
id: cmd-thousands
type: summary
status: unverified
title: cmd::thousands
source: myrn summarize
about: sym:crates/myrn/src/cmd/mod.rs#cmd::thousands
anchors:
- sym:crates/myrn/src/cmd/mod.rs#cmd::thousands@7bb6abacc98774e6
map_version: '2'
---
# cmd::thousands

Formats a signed 64-bit integer as a string with a comma inserted before every group of three digits.

## Behaviour

- 1234567 -> "1,234,567"
- -1234 -> "-1,234"; 0 and 999 -> no commas added
- i64::MIN: n.abs() overflows, which panics in debug builds and wraps in release builds

## Terms

digit grouping, comma separator, number formatting, thousands separator, format number, sign handling

## Connections

- **Location:** `crates/myrn/src/cmd/mod.rs:57-71`
- **Signature:** `pub fn thousands(n: i64) -> String`
- **In:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Called by:** [[init-run]], [[rebuild-run]], [[status-run]], [[summarize-run]]
