---
id: history-render
type: summary
status: unverified
title: history::render
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/history.rs#tools::history::render
anchors:
- sym:crates/myrn-mcp/src/tools/history.rs#tools::history::render@606b639022a43138
---
# history::render

Formats entries one per line as `<7-char sha> <YYYY-MM-DD> <summary>[ [change]] (<author>)`, with the date derived from `time` in seconds (floor-divided by 86400). Returns `No indexed commits touch this.` followed by a newline when the list is empty, and preserves the input order.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/history.rs:26-46`
- **Signature:** `pub fn render(entries: &[myrn_engine::api::HistoryEntry]) -> String`
- **In:** [[file-crates-myrn-mcp-src-tools-history-rs]]
- **Calls:** [[history-days-to-date]]
- **Called by:** [[history-call]], [[query-history]]
