---
id: archivename
type: summary
status: unverified
title: archiveName
source: myrn summarize
about: sym:dist/npm/install.js#archiveName
anchors:
- sym:dist/npm/install.js#archiveName@4effe6a3d32d4112
map_version: '2'
---
# archiveName

Builds the release archive filename for a target triple, using .zip for Windows triples and .tar.gz otherwise.

## Behaviour

- 'x86_64-pc-windows-msvc' -> 'myrn-x86_64-pc-windows-msvc.zip'
- 'x86_64-unknown-linux-musl' -> 'myrn-x86_64-unknown-linux-musl.tar.gz'
- Choice is a substring test for 'windows' in the triple, not a check of process.platform

## Terms

asset name, tarball name, zip name, archive filename, file extension, release file

## Connections

- **Location:** `dist/npm/install.js:42-44`
- **Signature:** `function archiveName(t)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
- **Tests:** `every supported platform maps to a release archive`
