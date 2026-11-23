---
id: states-fix-landed
type: summary
status: unverified
title: states::fix_landed
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::fix_landed
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::fix_landed@497f5b5e373545eb
map_version: '2'
---
# states::fix_landed

Checks that a fixing commit's sha prefix is in the history and not reverted by a later commit, returning a label with a 12-character sha.

## Behaviour

- Target is trimmed, a commit: prefix and [[ ]] are stripped, and it is lowercased
- Prefix shorter than 7 characters -> None
- A newer commit message containing reverts commit <full sha> -> None

## Terms

fixing commit, revert, fix landed, reverted fix, commit prefix

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:338-354`
- **Signature:** `fn fix_landed(history: &[(String, String)], target: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Called by:** [[engine-promote]]
- **Tests:** `a_reverted_fix_does_not_count`
