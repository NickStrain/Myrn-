---
id: api-locate
type: summary
status: unverified
title: api::locate
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::locate
anchors:
- sym:crates/myrn-engine/src/api.rs#api::locate@44309dbb4921a3dc
map_version: '2'
---
# api::locate

Finds the Git repository above a directory and works out its root, project id and worktree tag.

## Behaviour

- Outside a repository -> error saying myrn works per repository
- Bare repository -> error; no cached id and no root commit -> id uncommitted- plus 12 hex characters, provisional true
- A project id cached in config wins over the root commit

## Terms

repository discovery, project id, root commit, worktree, uncommitted repository, canonical path

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:147-179`
- **Signature:** `fn locate(cwd: &Path, config: Option<&Config>) -> Result<Located>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[git-root-commit]], [[indexer-content-hash]]
- **Called by:** [[engine-init]], [[engine-open]]
