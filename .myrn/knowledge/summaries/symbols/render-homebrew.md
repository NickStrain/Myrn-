---
id: render-homebrew
type: summary
status: unverified
title: render::homebrew
source: myrn summarize
about: sym:dist/render.py#dist.render.homebrew
anchors:
- sym:dist/render.py#dist.render.homebrew@4af90cae6c705915
map_version: '2'
---
# render::homebrew

Returns the text of a Ruby Homebrew formula for class Myrn, with one on_<os> block per OS containing on_arm and on_intel download blocks for the four UNIX archives. It reads the global REPO for the homepage, so it must run after main has set REPO.

## Behaviour

- UNIX list of 4 targets -> two on_macos and two on_linux blocks, each holding on_arm and on_intel with url and sha256
- archive url is base + "/myrn-<target>.tar.gz"; the hash comes from need, so a missing archive exits
- the formula's test block asserts version.to_s appears in "myrn --version" output

## Terms

Homebrew formula, Ruby formula, brew tap, on_macos, on_linux, rb file, Formula/myrn.rb, class Myrn

## Connections

- **Location:** `dist/render.py:56-84`
- **Signature:** `def homebrew(version, sums, base)`
- **In:** [[file-dist-render-py]]
- **Called by:** [[render-main]]
