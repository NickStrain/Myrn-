---
id: render-read-sums
type: summary
status: unverified
title: render::read_sums
source: myrn summarize
about: sym:dist/render.py#dist.render.read_sums
anchors:
- sym:dist/render.py#dist.render.read_sums@9f1911c882645a82
map_version: '2'
---
# render::read_sums

Parses a sha256sum-style text file into a dict mapping archive filename to hash. A leading asterisk (binary-mode marker) is removed from the filename.

## Behaviour

- "<hash> *myrn-x.tar.gz" line -> {"myrn-x.tar.gz": "<hash>"}
- lines without exactly two whitespace-separated fields (blank lines, extra columns) are skipped
- a filename that appears twice keeps the later line's hash

## Terms

checksum file, sha256sums, digest map, hash lookup, asterisk binary marker, parse checksums

## Connections

- **Location:** `dist/render.py:41-47`
- **Signature:** `def read_sums(path)`
- **In:** [[file-dist-render-py]]
- **Called by:** [[render-main]]
