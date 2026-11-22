---
id: hook-handle
type: summary
status: unverified
title: hook::handle
source: myrn summarize
about: sym:crates/myrn/src/cmd/hook.rs#cmd::hook::handle
anchors:
- sym:crates/myrn/src/cmd/hook.rs#cmd::hook::handle@0dc4cb7eabc83291
---
# hook::handle

Reads up to 4 MiB of stdin (invalid JSON becomes Value::Null) and returns empty text for unknown agents, unmapped events, projects that fail Engine::open, or agents missing from state. It injects context only for SessionStart and PromptSubmitted when config context.inject is set, recalls on failed ToolResult when records.recall_on_failure is set, and swaps in a records request for SessionEnd with reason 'stop'. Capture errors are only logged, but errors from refresh, brief, state load, or save_state propagate with ? and discard the output, and timing is saved last.

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs:35-132`
- **Signature:** `fn handle(ctx: &Ctx, agent: &str, event_arg: &str, started: Instant) -> Result<String>`
- **In:** [[file-crates-myrn-src-cmd-hook-rs]]
- **Calls:** [[adapter-translate]], [[adapter-format]], [[by-name]], [[engine-open]], [[engine-refresh]], [[engine-injected-in-session]], [[engine-context-excluding]], [[engine-brief]], [[engine-state]], [[engine-save-state]], [[engine-log-injection]], [[engine-capture]], [[engine-records-request]], [[engine-recall-text]], [[state-record-timing]], [[connect-now]]
- **Called by:** [[hook-run]]
