---
id: history-render
type: summary
status: unverified
title: history::render
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::render
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::render@606b639022a43138
map_version: '2'
---
# history::render

Formats each commit as one line: 7-character short SHA, date, summary, optional bracketed change tag, then author in parentheses.

## Behaviour

- empty entries -> the line 'No indexed commits touch this.' with a trailing newline
- change Some(c) -> ' [c]' appended after the summary; None -> nothing added
- SHA shorter than 7 bytes -> the whole SHA is used; time is floored to whole days (div_euclid 86400) before date conversion

## Terms

format commits, log output, commit line, text output, short SHA, commit listing

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:26-46`
- **Signature:** `pub fn render(entries: &[myrn_engine::api::HistoryEntry]) -> String`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[history-days-to-date]]
- **Called by:** [[history-call]], [[query-history]]
