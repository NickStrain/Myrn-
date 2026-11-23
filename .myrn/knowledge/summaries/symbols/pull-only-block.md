---
id: pull-only-block
type: summary
status: unverified
title: pull_only::block
source: myrn summarize
about: sym:crates/myrn-adapters/src/pull_only.rs#pull_only::block
anchors:
- sym:crates/myrn-adapters/src/pull_only.rs#pull_only::block@ce13c950f30a6a1c
map_version: '2'
---
# pull_only::block

Builds the marker block string: START line, TEXT, and END line, each followed by a newline. Takes no input and always returns the same text.

## Behaviour

- no input -> "<!-- myrn:start -->\n" + TEXT + "\n<!-- myrn:end -->\n"
- output always ends with a newline, with no leading blank line

## Terms

marker block, instruction text, start and end markers, agent instruction, pull-only text

## Connections

- **Location:** `crates/myrn-adapters/src/pull_only.rs:14-16`
- **Signature:** `pub fn block() -> String`
- **In:** [[file-crates-myrn-adapters-src-pull-only-rs]]
- **Called by:** [[pull-only-add]]
