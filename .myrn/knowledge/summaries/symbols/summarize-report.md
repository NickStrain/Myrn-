---
id: summarize-report
type: summary
status: unverified
title: summarize::Report
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Report
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Report@62f54f265c1f8fe1
map_version: '2'
---
# summarize::Report

Counts and failure list returned by Engine::summarize: map sizes, stale and updated counts, model calls, cost, notes written or removed, failures and the vault path.

## Behaviour

- stale_* counted before this run; updated_* counted for answers the model returned this run
- failures hold 'path: error' per file and './: error' or 'dir/: error' per directory; the run continues past them
- cost_usd sums total_cost_usd from the model's JSON output; stays 0.0 when the command reports none

## Terms

run statistics, counters, failure list, model cost, summary totals, dry run output

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:57-77`
- **Signature:** `pub struct Report`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
