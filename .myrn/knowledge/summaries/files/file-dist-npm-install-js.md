---
id: file-dist-npm-install-js
type: summary
status: unverified
title: dist/npm/install.js
source: myrn summarize
about: file:dist/npm/install.js
anchors:
- file:dist/npm/install.js@f445fbdb83d36e68
---
# dist/npm/install.js

Postinstall script that downloads the myrn binary for this platform from the GitHub release or MYRN_DOWNLOAD_BASE, verifies its sha256 against sha256sums.txt, and unpacks it into bin-cache. Its parts are the target, URL and path helpers, download, checksum parsing, a minimal tar reader, zip extraction via system tools, and install; when run directly it logs errors instead of failing npm install.

## Connections

- **Location:** `dist/npm/install.js`
- **In:** [[dir-dist-npm]]
- **Defines:** [[target]], [[archivename]], [[binarypath]], [[baseurl]], [[download]], [[expectedsum]], [[untar]], [[untar-str]], [[unzipwithsystemtools]], [[install]]
- **Imported by:** [[file-dist-npm-bin-myrn-js]], [[file-dist-npm-test-install-test-js]]
