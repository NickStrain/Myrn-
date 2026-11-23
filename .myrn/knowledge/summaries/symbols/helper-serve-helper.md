---
id: helper-serve-helper
type: summary
status: unverified
title: helper::serve
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::serve#2
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::serve#2@aad00dc266fdb72b
map_version: '2'
---
# helper::serve

Stub used when the model feature is disabled. It returns an error saying this build has no embedding support and does nothing else.

## Behaviour

- Any arguments -> Err with 'this build of myrn has no embedding support'.
- Nothing is bound, loaded or written to disk.

## Terms

disabled build, feature off, no-op, unsupported build, fallback stub

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:88-90`
- **Signature:** `pub fn serve(_data_home: &Path, _idle: Duration) -> Result<()>`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Called by:** [[main-main]]
