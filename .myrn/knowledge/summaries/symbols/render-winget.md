---
id: render-winget
type: summary
status: unverified
title: render::winget
source: myrn summarize
about: sym:dist/render.py#dist.render.winget
anchors:
- sym:dist/render.py#dist.render.winget@afc80919638e4c6f
map_version: '2'
---
# render::winget

Returns a dict of three winget manifest filenames to YAML text (version, installer, and en-US locale), all sharing a header with PackageIdentifier and PackageVersion. The installer section is a portable zip whose nested myrn.exe gets the myrn command alias.

## Behaviour

- WINDOWS list -> two Installers entries, Architecture x64 and arm64, each with InstallerUrl and InstallerSha256
- InstallerSha256 is uppercased, unlike scoop which keeps the hash's original case
- all three manifests use ManifestVersion 1.6.0 and the identifier NickStrain.Myrn

## Terms

winget manifest, winget-pkgs, installer yaml, locale yaml, PackageIdentifier, portable installer, PortableCommandAlias, Windows package manager

## Connections

- **Location:** `dist/render.py:112-146`
- **Signature:** `def winget(version, sums, base)`
- **In:** [[file-dist-render-py]]
- **Called by:** [[render-main]]
