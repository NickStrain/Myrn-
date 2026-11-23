---
id: dir-dist
type: summary
status: unverified
title: dist/
source: myrn summarize
about: module:dist
anchors:
- module:dist@8a65278c5bed5c8b
---
# dist/

dist/ holds the packaging for one myrn release. render.py reads a sha256sums file and writes the Homebrew formula, Scoop JSON, three winget YAML manifests, and a versioned npm package under --out. npm/ is the npm wrapper that ships the native myrn binary: install.js runs at postinstall, downloads and checksum-verifies the platform archive into bin-cache (from the GitHub release or MYRN_DOWNLOAD_BASE), and bin/myrn.js runs that binary with inherited stdio and exits with its status. test/ holds the Node tests for install.js.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-dist-npm]]
- **Files:** [[file-dist-render-py]]
