---
id: checks-looks-like-sha
type: summary
status: unverified
title: checks::looks_like_sha
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::looks_like_sha
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::looks_like_sha@3794a121fa111407
map_version: '2'
---
# checks::looks_like_sha

Reports whether a string could be a git commit hash prefix.

## Behaviour

- "abc1234" -> true (7 hex chars)
- "abc123" -> false (under 7 chars)
- a non-hex character anywhere -> false; uppercase hex is accepted

## Terms

commit hash, sha, hex digits, short sha, hash prefix

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:65-67`
- **Signature:** `fn looks_like_sha(t: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-resolve-ref]]
