---
id: detect-which
type: summary
status: unverified
title: detect::which
source: myrn summarize
about: sym:crates/myrn-adapters/src/detect.rs#detect::which
anchors:
- sym:crates/myrn-adapters/src/detect.rs#detect::which@e0dbf29275155c22
---
# detect::which

Takes a name and returns the first regular file named that on PATH, or None if no match exists or PATH is unset. On Windows it tries each PATHEXT extension (default .EXE;.CMD;.BAT, lowercased) and then the bare name last; on other platforms it only tries the bare name. It checks is_file only, not execute permission.

## Connections

- **Location:** `crates/myrn-adapters/src/detect.rs:12-33`
- **Signature:** `pub fn which(name: &str) -> Option<PathBuf>`
- **In:** [[file-crates-myrn-adapters-src-detect-rs]]
- **Called by:** [[claude-code-claude-cli]], [[claudecode-detect]], [[codex-detect]]
