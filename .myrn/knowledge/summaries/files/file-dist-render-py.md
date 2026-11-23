---
id: file-dist-render-py
type: summary
status: unverified
title: dist/render.py
source: myrn summarize
about: file:dist/render.py
anchors:
- file:dist/render.py@d3cd7238124feede
---
# dist/render.py

Renders the package manifests for one release (Homebrew formula, Scoop JSON, three winget YAML files, and a versioned npm wrapper package) from a sha256sums file, writing them under --out. read_sums parses the checksums, need looks up a required archive hash, homebrew, scoop and winget build the manifest text, npm copies and stamps the npm package, and main is the CLI entry point.

## Connections

- **Location:** `dist/render.py`
- **In:** [[dir-dist]]
- **Defines:** [[render-read-sums]], [[render-need]], [[render-homebrew]], [[render-scoop]], [[render-winget]], [[render-npm]], [[render-main]]
