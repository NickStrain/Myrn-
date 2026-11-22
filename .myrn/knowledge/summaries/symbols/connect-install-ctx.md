---
id: connect-install-ctx
type: summary
status: unverified
title: connect::install_ctx
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::install_ctx
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::install_ctx@dc6a657fe46353e8
---
# connect::install_ctx

Builds InstallCtx from the repo root and the current executable path, canonicalized with a fallback to the raw path. run_agent_cli is false only when the MYRN_NO_AGENT_CLI environment variable is set, to any value.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:20-27`
- **Signature:** `pub fn install_ctx(root: &std::path::Path) -> Result<InstallCtx>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Called by:** [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]]
