---
id: task-files-with-ext
type: summary
status: unverified
title: task::files_with_ext
source: myrn summarize
about: sym:bench/src/task.rs#task::files_with_ext
anchors:
- sym:bench/src/task.rs#task::files_with_ext@afb3d44276a03629
map_version: '2'
---
# task::files_with_ext

Lists the entries directly inside dir whose extension equals ext, sorted by path. It is non-recursive and takes the extension without a dot.

## Behaviour

- ext toml matches a.toml and x.y.toml, but not A.TOML, because the comparison is case-sensitive.
- A directory named something.toml is included, because there is no is_file check.
- Entries that fail to read are skipped silently.

## Terms

extension filter, file listing, suffix match, sorted listing, non-recursive scan

## Connections

- **Location:** `bench/src/task.rs:165-173`
- **Signature:** `fn files_with_ext(dir: &Path, ext: &str) -> Result<Vec<PathBuf>>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-repos]]
