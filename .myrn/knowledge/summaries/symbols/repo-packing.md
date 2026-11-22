---
id: repo-packing
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/packing.rs#repo
anchors:
- sym:crates/myrn-engine/tests/packing.rs#repo@ca818524d3ba37df
---
# repo

Creates a temp dir with a `repo` subdirectory populated from the given files, git-commits everything as "first", initializes an Engine against it with a separate home dir, and returns the TempDir guard with the Engine, which must be kept alive for the test.

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs:17-32`
- **Signature:** `fn repo(files: &[(&str, &str)]) -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-engine-tests-packing-rs]]
- **Calls:** [[engine-init]], [[write-packing]]
- **Tests:** `python_namespace_calls_through_wildcard_reexports_link_tests`, `typescript_namespace_calls_through_export_star_link_tests`, `packs_are_small_carry_code_and_skip_noise`
