---
id: expectedsum
type: summary
status: unverified
title: expectedSum
source: myrn summarize
about: sym:dist/npm/install.js#expectedSum
anchors:
- sym:dist/npm/install.js#expectedSum@0c147ee713ca1abb
map_version: '2'
---
# expectedSum

Finds the line for a file name in sha256sums.txt text and returns its hash in lowercase. Throws if the file is not listed.

## Behaviour

- 'ABC123 myrn.tar.gz' with name 'myrn.tar.gz' -> 'abc123'
- 'deadbeef *myrn.zip' -> 'deadbeef' (leading * from binary mode is stripped)
- Matching is exact on the file field, so 'dist/myrn.zip' does not match 'myrn.zip'

## Terms

checksum lookup, sha256, sums file parse, hash for file, digest, checksum line

## Connections

- **Location:** `dist/npm/install.js:64-70`
- **Signature:** `function expectedSum(sums, name)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
- **Tests:** `checksums are looked up by file name`
