---
id: lang-rust-module
type: summary
status: unverified
title: lang::rust_module
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::rust_module
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::rust_module@828da3a0f56447b4
map_version: '2'
---
# lang::rust_module

Splits a Rust file path into its crate directory and its module path inside the crate. The last src, tests, examples or benches directory marks the crate root, and mod.rs contributes only its parent directory name.

## Behaviour

- crates/a/src/auth/session.rs -> crate crates/a, module [auth, session]
- src/bin/tool.rs -> crate empty, module [], since binary roots have no module path
- build.rs with no root directory -> its own directory as crate, module []

## Terms

crate root, module path, module name, crate directory, Rust module, source root, mod.rs

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:95-117`
- **Signature:** `pub fn rust_module(path: &str) -> (String, Vec<String>)`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-module-prefix]], [[resolver-new]], [[resolver-rust-path]], [[resolver-lookup-in-crate]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reachable]], [[resolver-is-top-level]], [[symbols-rust-crates]]
- **Tests:** `rust_modules`
