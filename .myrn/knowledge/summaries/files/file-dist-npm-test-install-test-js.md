---
id: file-dist-npm-test-install-test-js
type: summary
status: unverified
title: dist/npm/test/install.test.js
source: myrn summarize
about: file:dist/npm/test/install.test.js
anchors:
- file:dist/npm/test/install.test.js@9fba09904350ee5d
---
# dist/npm/test/install.test.js

Node test file for dist/npm/install.js, run with `cd dist/npm && npm test`. It covers platform-to-archive name mapping, checksum lookup by file name, and untar of a gzipped ustar archive that the test builds in memory.

## Connections

- **Location:** `dist/npm/test/install.test.js`
- **In:** [[dir-dist-npm-test]]
- **Defines:** [[reads-a-ustar-archive-header]]
- **Imports:** [[file-dist-npm-install-js]]
