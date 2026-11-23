---
id: runner-build-map
type: summary
status: unverified
title: runner::build_map
source: myrn summarize
about: sym:bench/src/runner.rs#runner::build_map
anchors:
- sym:bench/src/runner.rs#runner::build_map@71e7026d9be34faa
map_version: '2'
---
# runner::build_map

Builds a repository's project map once per commit with myrn summarize, caches it, and returns the summaries directory and its cost.

## Behaviour

- Cache dir is <repo>-<first 12 chars of commit>; if summaries/project exists, returns it with cost 0
- Otherwise clears the work dirs, clones the checkout at the commit, runs myrn init and --json summarize -j 6
- Cost is cost_usd from the report, or 0 if absent; the temp src and home dirs are deleted afterwards

## Terms

summarize, summary cache, project summary, dollar cost, map cache, summaries directory, pre-build

## Connections

- **Location:** `bench/src/runner.rs:547-601`
- **Signature:** `pub fn build_map( checkout: &Path, repo: &Repo, myrn: &Path, cache: &Path, ) -> Result<(PathBuf, f64)>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-git]], [[runner-copy-dir]]
- **Called by:** [[real-main]]
