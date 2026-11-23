---
id: engine-publisher
type: summary
status: unverified
title: Engine::publisher
source: myrn summarize
about: sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publisher
anchors:
- sym:crates/myrn-engine/src/sharing.rs#sharing::Engine::publisher@b700944fcf93b6ae
map_version: '2'
---
# Engine::publisher

Returns the git user.name configured for the repository, or unknown if it cannot be read. Used as the published_by value.

## Behaviour

- user.name of ' name ' -> 'name' (trimmed)
- user.name missing, empty, or only spaces -> 'unknown'
- repository cannot be opened -> 'unknown'

## Terms

git user, author name, publisher name, identity, fallback, published by

## Connections

- **Location:** `crates/myrn-engine/src/sharing.rs:209-217`
- **Signature:** `fn publisher(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-sharing-rs]]
- **Called by:** [[engine-publish-plan]]
