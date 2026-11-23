---
id: detect-home
type: summary
status: unverified
title: detect::home
source: myrn summarize
about: sym:crates/myrn-adapters/src/detect.rs#detect::home
anchors:
- sym:crates/myrn-adapters/src/detect.rs#detect::home@3d7893973e91a158
map_version: '2'
---
# detect::home

Returns the HOME environment variable as a path, falling back to USERPROFILE only when HOME is unset. Returns None only when neither variable is set.

## Behaviour

- HOME=/home/u -> Some(/home/u)
- HOME unset, USERPROFILE=C:/Users/u -> Some(C:/Users/u)
- HOME set to empty string -> Some(empty path), no fallback to USERPROFILE

## Terms

user directory, HOME variable, USERPROFILE, profile directory, user folder, HOME fallback

## Connections

- **Location:** `crates/myrn-adapters/src/detect.rs:5-9`
- **Signature:** `pub fn home() -> Option<PathBuf>`
- **In:** [[file-crates-myrn-adapters-src-detect-rs]]
- **Called by:** [[claudecode-detect]], [[codex-detect]]
