---
id: write-validity
type: summary
status: unverified
title: write
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#write
anchors:
- sym:crates/myrn-engine/tests/validity.rs#write@bf0595a78a1deb5b
map_version: '2'
---
# write

Writes each (relative path, content) pair under root, creating missing parent directories and overwriting existing files. Panics on any I/O error.

## Behaviour

- root + [("a/b.py", "x")] -> creates a/ and writes a/b.py
- existing file -> content replaced, other files untouched
- nested path with no parent dirs -> parent dirs created first

## Terms

create files, write fixtures, materialize files, make directories, overwrite file, write to disk

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:27-33`
- **Signature:** `fn write(root: &Path, files: &[(&str, &str)])`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[repo-48c3]], [[set-validity]]
- **Tests:** `worktrees_share_notes_but_judge_them_by_their_own_files`, `shared_records_from_a_teammate_apply_when_their_code_matches`, `a_reverted_fix_does_not_verify`
