---
id: codex-detect
type: summary
status: unverified
title: Codex::detect
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::detect
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::detect@14c6216da72f85df
map_version: '2'
---
# Codex::detect

Reports whether Codex is present: a 'codex' binary on PATH, or a directory at CODEX_HOME or ~/.codex. It returns a description or None.

## Behaviour

- codex found on PATH -> '`codex` at <path>'
- no binary, CODEX_HOME set to an existing directory -> '<dir> exists'
- CODEX_HOME set but not a directory -> None; ~/.codex is not tried

## Terms

installed check, codex present, environment probe, home directory

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:164-172`
- **Signature:** `fn detect(&self) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[detect-home]], [[detect-which]]
