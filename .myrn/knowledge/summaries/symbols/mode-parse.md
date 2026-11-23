---
id: mode-parse
type: summary
status: unverified
title: Mode::parse
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::parse
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Mode::parse@c4c473772d0eb49d
map_version: '2'
---
# Mode::parse

Turns a stored string into a Mode, treating only the exact text pull_only as PullOnly and everything else as Hooks.

## Behaviour

- 'pull_only' -> PullOnly
- 'hooks' -> Hooks
- '' or 'PULL_ONLY' or unknown text -> Hooks (match is exact and case-sensitive, never errors)

## Terms

string to mode, fallback default, unknown mode, mode from config, case sensitive

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:34-40`
- **Signature:** `pub fn parse(s: &str) -> Mode`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[doctor-agent-checks]]
