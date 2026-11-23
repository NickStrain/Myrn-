---
id: reads-a-ustar-archive-header
type: summary
status: unverified
title: reads a ustar archive::header
source: myrn summarize
about: sym:dist/npm/test/install.test.js#reads a ustar archive.header
anchors:
- sym:dist/npm/test/install.test.js#reads a ustar archive.header@c2d62b01f7b7cfb3
map_version: '2'
---
# reads a ustar archive::header

Test-local helper that builds a 512-byte ustar header block with the given name, mode 0000755, zero-padded octal size, type flag "0" and ustar magic. It is used to construct a synthetic tar for untar.

## Behaviour

- header("./myrn", 5) -> 512-byte Buffer with "./myrn" at offset 0, "0000755" NUL at 100, 11-digit octal size at 124, "0" at 156, "ustar" NUL at 257
- size 0 -> size field is "00000000000" followed by NUL
- name is written without a length check, so a name longer than 100 bytes overwrites the mode field and later fields; checksum field at 148 is left zero

## Terms

tar block, octal size, mode field, ustar magic, typeflag, fixture builder, test helper, synthetic tar entry

## Connections

- **Location:** `dist/npm/test/install.test.js:24-32`
- **Signature:** `header = (name, size)`
- **In:** [[file-dist-npm-test-install-test-js]]
- **Tests:** `reads a ustar archive`
