---
id: baseurl
type: summary
status: unverified
title: baseUrl
source: myrn summarize
about: sym:dist/npm/install.js#baseUrl
anchors:
- sym:dist/npm/install.js#baseUrl@6dfeb0df7a354f16
map_version: '2'
---
# baseUrl

Returns the URL that release files are downloaded from: MYRN_DOWNLOAD_BASE if set, otherwise the GitHub release page for this package version. Trailing slashes are removed.

## Behaviour

- MYRN_DOWNLOAD_BASE=https://mirror.example/x/ -> 'https://mirror.example/x' (version is not appended)
- unset, package version 1.2.3 -> 'https://github.com/NickStrain/Myrn-/releases/download/v1.2.3'
- empty MYRN_DOWNLOAD_BASE falls back to the GitHub URL

## Terms

download url, release url, mirror, download base, release location, asset url

## Connections

- **Location:** `dist/npm/install.js:51-56`
- **Signature:** `function baseUrl()`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
