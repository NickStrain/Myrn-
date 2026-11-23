---
id: summarizeconfig-default
type: summary
status: unverified
title: SummarizeConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::SummarizeConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::SummarizeConfig::default@3e2cdb8fb567cffd
map_version: '2'
---
# SummarizeConfig::default

Sets the default summarizer: headless claude with the haiku model, no tools, JSON output, and 4 parallel jobs.

## Behaviour

- no input -> claude -p --model haiku --tools '' --no-session-persistence --output-format json
- the empty string after --tools is passed as its own argument
- no input -> jobs 4

## Terms

default summarize command, claude haiku, headless claude, no tools, json output

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:92-109`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
