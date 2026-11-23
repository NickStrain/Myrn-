---
id: untar-str
type: summary
status: unverified
title: untar::str
source: myrn summarize
about: sym:dist/npm/install.js#untar.str
anchors:
- sym:dist/npm/install.js#untar.str@877120014e1b385a
map_version: '2'
---
# untar::str

Inner helper defined inside untar, not exported. Decodes a fixed-width header field as UTF-8 and cuts it at the first NUL byte.

## Behaviour

- Name field holding 'myrn' followed by NUL bytes -> 'myrn'
- Field with no NUL byte -> full decoded text
- Field of only NUL bytes -> empty string

## Terms

header field, null-terminated string, tar field reader, NUL trimming, fixed width field, header decode

## Connections

- **Location:** `dist/npm/install.js:79`
- **Signature:** `str = (a, b)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[untar]]
