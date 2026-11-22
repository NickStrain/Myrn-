---
id: record-spec
type: summary
status: unverified
title: record::spec
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/record.rs#tools::record::spec
anchors:
- sym:crates/myrn-mcp/src/tools/record.rs#tools::record::spec@f606d79d4e1992cf
---
# record::spec

Returns the MCP tool definition JSON for `record`, with the description and input schema covering type (bug/decision/workflow/fact/summary), title, body, about, evidence, bug fields (error, root_cause, fix, fixed_in), workflow steps, verified_by, relations, session, and a batch `items` array. Nothing is validated here; validation happens in the engine.

## Connections

- **Location:** `crates/myrn-mcp/src/tools/record.rs:9-50`
- **Signature:** `pub fn spec() -> Value`
- **In:** [[file-crates-myrn-mcp-src-tools-record-rs]]
- **Calls:** [[tools-schema]]
- **Called by:** [[tools-list]]
