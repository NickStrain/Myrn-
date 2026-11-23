---
id: recall-call
type: summary
status: unverified
title: recall::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::call
anchors:
- sym:crates/myrn-mcp/src/tools/recall.rs#tools::recall::call@b6beee73394f3e39
map_version: '2'
---
# recall::call

Runs recall_text on the error with a token budget, logs the matched ids to the injection log, and returns the matched text or a no-match message.

## Behaviour

- error text plus budget -> recall_text yields (text, ids); non-empty text is logged via log_injection('mcp', ids) and returned
- empty text from recall_text -> returns 'No recorded bug matches this error.' and logs nothing
- budget missing, negative, or not an integer -> 1500; otherwise cast to usize

## Terms

bug lookup, error lookup, past bug match, known bug search, injection log, no match result, token budget, matched ids

## Connections

- **Location:** `crates/myrn-mcp/src/tools/recall.rs:21-30`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-recall-rs]]
- **Calls:** [[engine-log-injection]], [[tools-str-arg]]
- **Called by:** [[tools-call]]
