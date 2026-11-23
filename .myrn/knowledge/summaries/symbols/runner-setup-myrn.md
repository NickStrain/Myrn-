---
id: runner-setup-myrn
type: summary
status: unverified
title: runner::setup_myrn
source: myrn summarize
about: sym:bench/src/runner.rs#runner::setup_myrn
anchors:
- sym:bench/src/runner.rs#runner::setup_myrn@844b9c93b3261844
map_version: '2'
---
# runner::setup_myrn

Initializes and connects myrn in the work directory, rewrites the inject, ask_at_session_end and mode lines in .myrn/project.toml, and copies in the project map and embedding model.

## Behaviour

- fixed_ranking true -> mode "fixed", false -> mode "learned"
- A missing inject, ask_at_session_end or mode line in project.toml -> error
- Embedding model is copied only if model.safetensors is absent, then embeddings enable runs

## Terms

configure myrn, project toml, init, connect, injection setting, ranking mode, semantic search, project map copy

## Connections

- **Location:** `bench/src/runner.rs:461-528`
- **Signature:** `pub fn setup_myrn( myrn: &Path, work: &Path, home: &Path, inject: bool, fixed_ranking: bool, map: Option<&Path>, embeddings: Option<&Path>, ) -> Result<()>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-copy-dir]]
- **Called by:** [[retrieval-one]], [[runner-run-inner]]
