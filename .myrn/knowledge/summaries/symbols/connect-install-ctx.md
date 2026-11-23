---
id: connect-install-ctx
type: summary
status: unverified
title: connect::install_ctx
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::install_ctx
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::install_ctx@dc6a657fe46353e8
map_version: '2'
---
# connect::install_ctx

Builds the InstallCtx for a repository root, with the canonical path of the running myrn binary and a flag that controls whether the agent CLI is run.

## Behaviour

- root=/repo, MYRN_NO_AGENT_CLI unset -> repo_root=/repo and run_agent_cli=true
- MYRN_NO_AGENT_CLI set to any value, even empty -> run_agent_cli=false
- binary path cannot be canonicalized -> falls back to the non-canonical current_exe path

## Terms

install context, binary path, repo root, agent cli env var, canonicalize

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:20-27`
- **Signature:** `pub fn install_ctx(root: &std::path::Path) -> Result<InstallCtx>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]]
