---
id: config-summarizeconfig
type: summary
status: unverified
title: config::SummarizeConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::SummarizeConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::SummarizeConfig@89a36b880575c7ca
map_version: '2'
---
# config::SummarizeConfig

Holds the model command that myrn summarize runs to write the project map, and how many calls run at once.

## Behaviour

- the command gets the prompt on stdin and its answer is read from stdout
- stdout may be plain text or Claude Code --output-format json
- jobs 4 -> four model calls run at the same time

## Terms

summarize command, project map, model command, stdin prompt, parallel calls, concurrency

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:83-89`
- **Signature:** `pub struct SummarizeConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
