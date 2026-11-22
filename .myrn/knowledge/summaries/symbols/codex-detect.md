---
id: codex-detect
type: summary
status: unverified
title: Codex::detect
source: myrn summarize
about: sym:crates/myrn-adapters/src/codex.rs#codex::Codex::detect
anchors:
- sym:crates/myrn-adapters/src/codex.rs#codex::Codex::detect@14c6216da72f85df
---
# Codex::detect

Returns a description if the codex binary is on PATH or the CODEX_HOME directory (default ~/.codex) exists, and None otherwise. It does not check the repository.

## Connections

- **Location:** `crates/myrn-adapters/src/codex.rs:164-172`
- **Signature:** `fn detect(&self) -> Option<String>`
- **In:** [[file-crates-myrn-adapters-src-codex-rs]]
- **Calls:** [[detect-home]], [[detect-which]]
