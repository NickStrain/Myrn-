---
id: untar
type: summary
status: unverified
title: untar
source: myrn summarize
about: sym:dist/npm/install.js#untar
anchors:
- sym:dist/npm/install.js#untar@82eee21a9a9019c2
map_version: '2'
---
# untar

Reads an uncompressed ustar buffer and returns a map from regular-file names to their contents. Directories and other entry types are skipped.

## Behaviour

- Stops at the first all-zero 512-byte block or at the end of the buffer
- Names starting with './' lose that prefix; a non-empty prefix field is joined to the name with '/'
- Values are subarray views into buf, not copies; size is parsed as octal

## Terms

tar extract, tar reader, ustar parser, unpack tar, archive entries, regular files only

## Connections

- **Location:** `dist/npm/install.js:73-92`
- **Signature:** `function untar(buf)`
- **In:** [[file-dist-npm-install-js]]
- **Calls:** [[untar-str]]
- **Called by:** [[install]]
- **Tests:** `reads a ustar archive`
