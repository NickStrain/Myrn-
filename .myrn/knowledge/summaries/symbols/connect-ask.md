---
id: connect-ask
type: summary
status: unverified
title: connect::ask
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::ask
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::ask@939b4b833dadd6ad
map_version: '2'
---
# connect::ask

Prints a yes/no question on stdout and reads one line from stdin, returning true only for an explicit yes.

## Behaviour

- stdin is not a terminal -> returns false without printing anything
- user types Y or yes with spaces around it -> true
- empty line or any other text -> false

## Terms

prompt, consent prompt, yes no, stdin read, confirmation, non interactive

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:67-76`
- **Signature:** `fn ask(question: &str) -> Result<bool>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]]
