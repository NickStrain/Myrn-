---
id: seed-segments
type: summary
status: unverified
title: seed::segments
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::segments
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::segments@ca2efea14b6f943a
map_version: '2'
---
# seed::segments

Splits a qualified name on :: and . into segments, after removing a trailing #digits collision suffix.

## Behaviour

- auth::session::Session -> [auth, session, Session]
- Session.refresh#2 -> [Session, refresh]
- empty segments from a..b or a leading dot are dropped; f#x keeps its # because x is not digits

## Terms

split, qualified name, path segments, collision suffix, hash suffix, module path

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:100-110`
- **Signature:** `pub fn segments(q: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[engine-pick]], [[seed-find]], [[seed-ends-with-segments]], [[engine-summarize]], [[summarize-title-of]]
- **Tests:** `segment_matching`
