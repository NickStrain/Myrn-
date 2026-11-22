---
id: mode-parse
type: summary
status: unverified
title: Mode::parse
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::parse
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::parse@c4c473772d0eb49d
---
# Mode::parse

Only the exact string "pull_only" yields PullOnly; any other input, including typos, silently becomes Hooks.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:34-40`
- **Signature:** `pub fn parse(s: &str) -> Mode`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[doctor-agent-checks]]
