---
id: task-files-with-ext
type: summary
status: unverified
title: task::files_with_ext
source: myrn summarize
about: sym:bench/src/task.rs#task::files_with_ext
anchors:
- sym:bench/src/task.rs#task::files_with_ext@afb3d44276a03629
---
# task::files_with_ext

Returns the sorted paths of non-recursive entries in dir whose extension equals ext. ext is given without the dot. Unreadable directory entries are silently skipped.

## Connections

- **Location:** `bench/src/task.rs:165-173`
- **Signature:** `fn files_with_ext(dir: &Path, ext: &str) -> Result<Vec<PathBuf>>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-repos]]
