---
id: dir-dist-npm
type: summary
status: unverified
title: dist/npm/
source: myrn summarize
about: module:dist/npm
anchors:
- module:dist/npm@fbeda44ee3bb2e89
---
# dist/npm/

npm package wrapper that ships the myrn CLI as a native binary. install.js runs at postinstall, downloads the platform-specific archive from the GitHub release or MYRN_DOWNLOAD_BASE, verifies it against sha256sums.txt, and unpacks it into bin-cache. bin/myrn.js then spawns that cached binary with the caller's arguments and inherited stdio and exits with its status, and test/ holds the Node tests for install.js.

## Connections

- **In:** [[dir-dist]]
- **Directories:** [[dir-dist-npm-bin]], [[dir-dist-npm-test]]
- **Files:** [[file-dist-npm-install-js]]
