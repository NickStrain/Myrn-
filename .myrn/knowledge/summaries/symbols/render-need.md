---
id: render-need
type: summary
status: unverified
title: render::need
source: myrn summarize
about: sym:dist/render.py#dist.render.need
anchors:
- sym:dist/render.py#dist.render.need@25e719462143184e
map_version: '2'
---
# render::need

Returns the hash for a required filename, or aborts the whole process if that filename is not in the checksums.

## Behaviour

- known name -> its hash string, returned unchanged (case preserved, format not validated)
- unknown name -> sys.exit with "render.py: <name> is missing from the checksums"; nothing is returned
- used by every manifest builder, so one missing archive stops rendering

## Terms

required checksum, missing archive, abort render, hash guard, mandatory lookup, fail fast

## Connections

- **Location:** `dist/render.py:50-53`
- **Signature:** `def need(sums, name)`
- **In:** [[file-dist-render-py]]
- **Called by:** [[render-scoop]]
