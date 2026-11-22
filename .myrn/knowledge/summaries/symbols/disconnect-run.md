---
id: disconnect-run
type: summary
status: unverified
title: disconnect::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/disconnect.rs#cmd::disconnect::run
anchors:
- sym:crates/myrn/src/cmd/disconnect.rs#cmd::disconnect::run@808e143771f46985
---
# disconnect::run

Takes the context and a list of agent names (an empty list means every agent in state) and runs each matching adapter's uninstall with the creation records saved in state, printing each change and removing the agent from state; unknown names are printed and skipped, and capture consent is cleared only when no agent remains in hooks mode. An error from uninstall returns early via ? before save_state, so agents already processed in that call are not persisted as removed.

## Connections

- **Location:** `crates/myrn/src/cmd/disconnect.rs:9-49`
- **Signature:** `pub fn run(ctx: &Ctx, agents: Vec<String>) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-disconnect-rs]]
- **Calls:** [[by-name]], [[engine-open]], [[engine-root]], [[engine-state]], [[engine-save-state]], [[connect-install-ctx]]
- **Called by:** [[main-main]]
