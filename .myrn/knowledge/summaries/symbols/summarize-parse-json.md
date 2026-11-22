---
id: summarize-parse-json
type: summary
status: unverified
title: summarize::parse_json
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::parse_json
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::parse_json@5b18c96ec2a95595
---
# summarize::parse_json

Extracts the first `{` through last `}` of a model answer and parses it as JSON, so code fences or surrounding prose are tolerated. It errors if no braces are found or the slice is not valid JSON.

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:763-776`
- **Signature:** `fn parse_json(text: &str) -> Result<serde_json::Value>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[summarize-ask-json]]
- **Tests:** `json_is_found_inside_noise`
