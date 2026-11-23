---
id: render-npm
type: summary
status: unverified
title: render::npm
source: myrn summarize
about: sym:dist/render.py#dist.render.npm
anchors:
- sym:dist/render.py#dist.render.npm@c3541cd66092e62c
map_version: '2'
---
# render::npm

Copies the dist/npm package tree into <out>/npm, sets its package.json version and git repository URL, and copies README.md and LICENSE from the repo root when they exist. It writes to the filesystem and returns nothing.

## Behaviour

- an existing <out>/npm directory is deleted before the copy
- ignores .gitkeep, node_modules, test and bin-cache entries when copying
- README.md or LICENSE missing from the repo root is skipped silently

## Terms

npm wrapper, npm package, package.json, version stamp, repository field, copy tree, node module, npm publish

## Connections

- **Location:** `dist/render.py:149-163`
- **Signature:** `def npm(version, out)`
- **In:** [[file-dist-render-py]]
- **Called by:** [[render-main]]
