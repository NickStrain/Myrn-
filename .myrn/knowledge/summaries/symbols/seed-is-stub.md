---
id: seed-is-stub
type: summary
status: unverified
title: seed::is_stub
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::is_stub
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::is_stub@58996eb09d8d22d8
map_version: '2'
---
# seed::is_stub

Reports whether a path is a type declaration stub (.pyi, .d.ts, .d.mts) that only declares what an implementation defines.

## Behaviour

- pkg/mod.pyi -> true
- types/index.d.ts -> true
- src/index.ts or types.d.tsx -> false (.d.tsx and .d.cts are not listed)

## Terms

type stub, declaration file, typings, pyi, d.ts, stub file

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:77-79`
- **Signature:** `pub fn is_stub(path: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
