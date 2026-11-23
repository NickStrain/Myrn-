---
id: detect-which
type: summary
status: unverified
title: detect::which
source: myrn summarize
about: sym:crates/myrn-adapters/src/detect.rs#detect::which
anchors:
- sym:crates/myrn-adapters/src/detect.rs#detect::which@e0dbf29275155c22
map_version: '2'
---
# detect::which

Searches PATH directories in order and returns the first path where name, with a suffix, is an existing file. Returns None if PATH is unset or nothing matches.

## Behaviour

- PATH unset, or no match in any entry -> None
- Unix, name=ls, /usr/bin contains ls -> Some(/usr/bin/ls); only is_file is checked, not the execute bit
- Windows, PATHEXT=.EXE;.CMD -> tries name.exe, name.cmd, then bare name; directories with a matching name are skipped

## Terms

executable lookup, PATH search, find binary, command lookup, PATHEXT suffix, binary on PATH

## Connections

- **Location:** `crates/myrn-adapters/src/detect.rs:12-33`
- **Signature:** `pub fn which(name: &str) -> Option<PathBuf>`
- **In:** [[file-crates-myrn-adapters-src-detect-rs]]
- **Called by:** [[claude-code-claude-cli]], [[claudecode-detect]], [[codex-detect]]
