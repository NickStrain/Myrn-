---
id: render-main
type: summary
status: unverified
title: render::main
source: myrn summarize
about: sym:dist/render.py#dist.render.main
anchors:
- sym:dist/render.py#dist.render.main@c52d747dd65846d7
map_version: '2'
---
# render::main

CLI entry point. Parses --version, --sums, --repo and --out, sets the global REPO, creates the homebrew, scoop and winget subdirectories, writes all manifests and the npm package, then prints a summary line.

## Behaviour

- --version "v0.1.0" -> version "0.1.0" and base "https://github.com/<repo>/releases/download/v0.1.0" (lstrip("v") removes all leading v characters)
- --repo omitted -> default "NickStrain/Myrn-"
- writes files and prints "rendered manifests for <version> into <out>" to stdout

## Terms

CLI, command line, argparse, release version, output directory, v prefix, render release, entry point

## Connections

- **Location:** `dist/render.py:166-187`
- **Signature:** `def main()`
- **In:** [[file-dist-render-py]]
- **Calls:** [[render-read-sums]], [[render-homebrew]], [[render-scoop]], [[render-winget]], [[render-npm]]
