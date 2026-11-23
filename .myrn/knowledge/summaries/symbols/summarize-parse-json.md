---
id: summarize-parse-json
type: summary
status: unverified
title: summarize::parse_json
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::parse_json
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::parse_json@5b18c96ec2a95595
map_version: '2'
---
# summarize::parse_json

Takes the span from the first '{' to the last '}' in a model answer and parses it as one JSON value, so prose and code fences around the object are ignored.

## Behaviour

- prose, then a code fence, then one JSON object -> that object parsed
- no '{' or no '}' in the text -> error 'answer has no JSON'
- two separate objects in one answer -> the span covers both, so parsing usually fails

## Terms

JSON extraction, code fence stripping, answer parsing, brace span

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:850-863`
- **Signature:** `fn parse_json(text: &str) -> Result<serde_json::Value>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[summarize-ask-json]]
- **Tests:** `json_is_found_inside_noise`
