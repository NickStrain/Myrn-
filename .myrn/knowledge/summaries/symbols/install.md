---
id: install
type: summary
status: unverified
title: install
source: myrn summarize
about: sym:dist/npm/install.js#install
anchors:
- sym:dist/npm/install.js#install@5dc5843ca0b3b189
map_version: '2'
---
# install

Makes the myrn binary available. Returns MYRN_BINARY_PATH or an existing cached binary; otherwise downloads the archive and sha256sums.txt in parallel, verifies the checksum, and extracts into bin-cache.

## Behaviour

- MYRN_BINARY_PATH set -> returns it immediately, no network access
- Binary already at binaryPath() -> returns it, no network access
- Checksum mismatch throws before anything is written to bin-cache; quiet suppresses only the progress log, not errors

## Terms

install binary, postinstall, download and verify, setup binary, fetch binary, cache check

## Connections

- **Location:** `dist/npm/install.js:106-139`
- **Signature:** `async function install({ quiet = false } = {})`
- **In:** [[file-dist-npm-install-js]]
- **Calls:** [[target]], [[archivename]], [[binarypath]], [[baseurl]], [[download]], [[expectedsum]], [[untar]], [[unzipwithsystemtools]]
