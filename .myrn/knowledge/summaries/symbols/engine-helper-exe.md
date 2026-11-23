---
id: engine-helper-exe
type: summary
status: unverified
title: Engine::helper_exe
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::helper_exe
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::helper_exe@d0996b6a8bba3d92
map_version: '2'
---
# Engine::helper_exe

Returns the current executable path only when its file stem is exactly 'myrn'; otherwise None, so test and benchmark binaries load the model in-process.

## Behaviour

- Binary named myrn -> Some(path)
- Test binary such as myrn-engine-tests -> None
- current_exe fails or the name is not UTF-8 -> None

## Terms

helper binary, helper process, self exe, executable path, spawn helper, in-process model

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:59-63`
- **Signature:** `fn helper_exe() -> Option<PathBuf>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Called by:** [[engine-embed]]
