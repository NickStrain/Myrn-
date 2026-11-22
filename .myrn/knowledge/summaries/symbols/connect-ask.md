---
id: connect-ask
type: summary
status: unverified
title: connect::ask
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::ask
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::ask@939b4b833dadd6ad
---
# connect::ask

Prints a [y/N] prompt and reads one stdin line. If stdin is not a terminal it returns Ok(false) without prompting, and only y or yes (case-insensitive) counts as yes.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:67-76`
- **Signature:** `fn ask(question: &str) -> Result<bool>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]]
