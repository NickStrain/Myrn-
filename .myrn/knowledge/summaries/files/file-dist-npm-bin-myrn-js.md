---
id: file-dist-npm-bin-myrn-js
type: summary
status: unverified
title: dist/npm/bin/myrn.js
source: myrn summarize
about: file:dist/npm/bin/myrn.js
anchors:
- file:dist/npm/bin/myrn.js@bf55228f87b49515
---
# dist/npm/bin/myrn.js

Node.js launcher for the `myrn` CLI: it resolves the native binary through `../install.js`, runs that binary with this process's arguments and inherited stdio, and exits with the child's exit status. Exits 1 on install failure, spawn error, or signal termination (null status).

## Connections

- **Location:** `dist/npm/bin/myrn.js`
- **In:** [[dir-dist-npm-bin]]
- **Imports:** [[file-dist-npm-install-js]]
