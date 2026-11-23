---
id: rank-learned
type: summary
status: unverified
title: rank::learned
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::learned
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::learned@8389c721cb922bd9
map_version: '2'
---
# rank::learned

Multiplies each candidate's score by its usage factor and by a note freshness factor based on the age of its latest notes mtime. It only runs when the config mode is 'learned'.

## Behaviour

- Ids with no usage record keep their score for the usage step; ids with no notes row get no freshness factor
- Note mtime is stored in nanoseconds and converted to days old relative to now; a clock error makes now 0.0
- Changes scores in place and returns Ok(()) with no other effect

## Terms

usefulness weighting, usage boost, note freshness, age decay, learned mode, stale notes, past usefulness

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:116-135`
- **Signature:** `fn learned(c: &rusqlite::Connection, out: &mut [Ranked], cfg: &RankingConfig) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
- **Calls:** [[usage-factor]], [[learn-usage-of]], [[learn-freshness]]
- **Called by:** [[rank-rank]]
