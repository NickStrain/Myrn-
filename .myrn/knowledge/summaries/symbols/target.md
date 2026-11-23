---
id: target
type: summary
status: unverified
title: target
source: myrn summarize
about: sym:dist/npm/install.js#target
anchors:
- sym:dist/npm/install.js#target@3ed894c6118f691d
map_version: '2'
---
# target

Maps platform and arch to the Rust target triple used in release asset names. Throws if no prebuilt binary exists for that pair.

## Behaviour

- ('linux', 'x64') -> 'x86_64-unknown-linux-musl'
- ('darwin', 'arm64') -> 'aarch64-apple-darwin'
- ('freebsd', 'x64') -> throws with a cargo install --git hint

## Terms

platform triple, rust target, prebuilt binary lookup, arch mapping, unsupported platform, target triple

## Connections

- **Location:** `dist/npm/install.js:31-40`
- **Signature:** `function target(platform = process.platform, arch = process.arch)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
- **Tests:** `every supported platform maps to a release archive`
