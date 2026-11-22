---
id: lang-rust-module
type: summary
status: unverified
title: lang::rust_module
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::rust_module
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::rust_module@828da3a0f56447b4
---
# lang::rust_module

Takes a repo-relative path to a Rust file and returns (crate directory, module segments). It uses the rightmost src, tests, examples or benches directory as the crate root, drops lib.rs, main.rs and mod.rs stems, and returns an empty module path for src/bin binaries. Files outside any root directory return their parent directory and an empty path.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:95-117`
- **Signature:** `pub fn rust_module(path: &str) -> (String, Vec<String>)`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[lang-module-prefix]], [[resolver-new]], [[resolver-rust-path]], [[resolver-lookup-in-crate]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reachable]], [[resolver-is-top-level]], [[symbols-rust-crates]]
- **Tests:** `rust_modules`
