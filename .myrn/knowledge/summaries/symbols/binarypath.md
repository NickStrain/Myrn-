---
id: binarypath
type: summary
status: unverified
title: binaryPath
source: myrn summarize
about: sym:dist/npm/install.js#binaryPath
anchors:
- sym:dist/npm/install.js#binaryPath@af1cadd3e3ea7e12
map_version: '2'
---
# binaryPath

Returns the location of the myrn executable: MYRN_BINARY_PATH if set, otherwise bin-cache/myrn (myrn.exe on win32). Does not check that the file exists.

## Behaviour

- MYRN_BINARY_PATH=/opt/myrn -> '/opt/myrn', nothing else checked
- unset on win32 -> CACHE/myrn.exe
- empty MYRN_BINARY_PATH is treated as unset

## Terms

binary location, cache path, exe path, MYRN_BINARY_PATH override, executable path, bin-cache

## Connections

- **Location:** `dist/npm/install.js:46-49`
- **Signature:** `function binaryPath()`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
