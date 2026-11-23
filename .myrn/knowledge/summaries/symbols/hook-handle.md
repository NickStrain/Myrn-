---
id: hook-handle
type: summary
status: unverified
title: hook::handle
source: myrn summarize
about: sym:crates/myrn/src/cmd/hook.rs#cmd::hook::handle
anchors:
- sym:crates/myrn/src/cmd/hook.rs#cmd::hook::handle@0a96e24df71e4725
map_version: '2'
---
# hook::handle

Reads up to 4 MiB of stdin as JSON, translates it through the agent adapter, and returns the formatted context for the agent. It also records usage, session outcome, and hook timing, and resolves the project dir from event cwd, then CLAUDE_PROJECT_DIR, then the current dir.

## Behaviour

- stdin beyond 4194304 bytes is cut off; invalid JSON becomes null instead of an error
- unknown agent, adapter returns no event, or agent absent from state.agents -> empty string, nothing recorded
- failed tool result with neither error nor command text -> no recall text

## Terms

stdin payload, context injection, agent adapter, recall on failure, usage tracking, session outcome, episode log, hook timing

## Connections

- **Location:** `crates/myrn/src/cmd/hook.rs:35-192`
- **Signature:** `fn handle(ctx: &Ctx, agent: &str, event_arg: &str, started: Instant) -> Result<String>`
- **In:** [[file-crates-myrn-src-cmd-hook-rs]]
- **Calls:** [[adapter-translate]], [[adapter-format]], [[by-name]], [[engine-open]], [[engine-root]], [[engine-refresh]], [[engine-injected-in-session]], [[engine-context-excluding]], [[engine-brief]], [[engine-state]], [[engine-save-state]], [[engine-log-injection]], [[engine-capture]], [[engine-records-request]], [[engine-mark-used-files]], [[engine-attach-outcome]], [[engine-recall-text]], [[engine-suspect-notice]], [[engine-promote]], [[paths-repo-rel]], [[state-record-timing]], [[connect-now]]
- **Called by:** [[hook-run]]
