---
id: connect-choose
type: summary
status: unverified
title: connect::choose
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::choose
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::choose@57213acd541ef83b
---
# connect::choose

Returns the adapters to install without installing them. Named agents resolve via by_name and unknown names error. Otherwise it uses auto-detected agents: zero is an error, one is used, and several require all or error with a message to pick with --agent.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:29-53`
- **Signature:** `fn choose(args: &Args) -> Result<Vec<Box<dyn Adapter>>>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Calls:** [[all]], [[by-name]]
- **Called by:** [[connect-run]]
