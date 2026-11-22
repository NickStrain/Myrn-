---
id: outcome-commits-since
type: summary
status: unverified
title: outcome::commits_since
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::commits_since
anchors:
- sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::commits_since@863107c8e79b0af8
---
# outcome::commits_since

Opens the repo containing root via Repository::discover and walks from HEAD in time-sorted order, returning up to limit entries formatted as 'sha summary', newest first. It stops at the first commit whose committer time is before since, and returns an empty Vec on any git error (no repo, no HEAD, failed walk).

## Connections

- **Location:** `crates/myrn-engine/src/capture/outcome.rs:62-84`
- **Signature:** `pub fn commits_since(root: &Path, since: i64, limit: usize) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-capture-outcome-rs]]
- **Calls:** [[episode-summary]]
- **Called by:** [[engine-capture]]
