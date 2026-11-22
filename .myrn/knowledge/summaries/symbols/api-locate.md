---
id: api-locate
type: summary
status: unverified
title: api::locate
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::locate
anchors:
- sym:crates/myrn-engine/src/api.rs#api::locate@44309dbb4921a3dc
---
# api::locate

Finds the Git repository containing cwd and computes its root, project id and worktree tag; errors outside a repository and for bare repositories. The id comes from the config if set, otherwise from the root commit, otherwise from a provisional uncommitted-<hash> value.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:145-177`
- **Signature:** `fn locate(cwd: &Path, config: Option<&Config>) -> Result<Located>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[git-root-commit]], [[indexer-content-hash]]
- **Called by:** [[engine-init]], [[engine-open]]
