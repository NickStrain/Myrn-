---
id: sharing-publishitem
type: summary
status: unverified
title: sharing::PublishItem
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::PublishItem
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::PublishItem@45e7100f44b73308
map_version: '2'
---
# sharing::PublishItem

Describes one file to publish: source path, destination path under the knowledge directory, the exact text to commit, and what was stripped or trimmed. The verified_here flag is not serialized.

## Behaviour

- destination is knowledge dir, then folder for the note type, then the file name for the id
- from differs from to -> publish deletes the source file after writing
- stripped lists removed directive text, trimmed lists before and after evidence values

## Terms

commit content, target path, source path, stripped text, trimmed evidence, file to write

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:38-52`
- **Signature:** `pub struct PublishItem`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
