---
id: dir-dist-npm-bin
type: summary
status: unverified
title: dist/npm/bin/
source: myrn summarize
about: module:dist/npm/bin
anchors:
- module:dist/npm/bin@ace5a0d26572297e
---
# dist/npm/bin/

Node.js entry point for the `myrn` CLI. `myrn.js` loads the native binary path via `../install.js`, spawns that binary with this process's arguments and inherited stdio, and exits with the child's exit code. Exits 1 if installation fails, the spawn errors, or the child is killed by a signal (null status).

## Connections

- **In:** [[dir-dist-npm]]
- **Files:** [[file-dist-npm-bin-myrn-js]]
