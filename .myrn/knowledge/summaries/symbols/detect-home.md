---
id: detect-home
type: summary
status: unverified
title: detect::home
source: myrn summarize
about: sym:crates/myrn-adapters/src/detect.rs#detect::home
anchors:
- sym:crates/myrn-adapters/src/detect.rs#detect::home@3d7893973e91a158
---
# detect::home

Returns the HOME environment variable as a PathBuf, falling back to USERPROFILE if HOME is unset, or None if neither is set. It does not check that the directory exists, and an empty HOME value is returned as Some("") without falling back to USERPROFILE.

## Connections

- **Location:** `crates/myrn-adapters/src/detect.rs:5-9`
- **Signature:** `pub fn home() -> Option<PathBuf>`
- **In:** [[file-crates-myrn-adapters-src-detect-rs]]
- **Called by:** [[claudecode-detect]], [[codex-detect]]
