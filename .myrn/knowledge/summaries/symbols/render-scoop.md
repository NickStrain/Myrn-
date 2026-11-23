---
id: render-scoop
type: summary
status: unverified
title: render::scoop
source: myrn summarize
about: sym:dist/render.py#dist.render.scoop
anchors:
- sym:dist/render.py#dist.render.scoop@228a601314fa5eef
map_version: '2'
---
# render::scoop

Returns a dict for a Scoop JSON manifest with per-architecture zip URLs and hashes for the two Windows targets, plus autoupdate templates and bin set to myrn.exe.

## Behaviour

- WINDOWS list -> architecture keys "64bit" and "arm64", each with url and hash
- hash is passed through need unchanged, so its original case is kept
- autoupdate url and hash regex are literal Scoop $version/$baseurl/$basename templates and are not filled in here

## Terms

Scoop manifest, scoop bucket, Windows zip, autoupdate, checkver, 64bit arm64, json manifest, portable app

## Connections

- **Location:** `dist/render.py:87-109`
- **Signature:** `def scoop(version, sums, base)`
- **In:** [[file-dist-render-py]]
- **Calls:** [[render-need]]
- **Called by:** [[render-main]]
