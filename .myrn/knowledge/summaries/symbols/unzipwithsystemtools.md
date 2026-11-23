---
id: unzipwithsystemtools
type: summary
status: unverified
title: unzipWithSystemTools
source: myrn summarize
about: sym:dist/npm/install.js#unzipWithSystemTools
anchors:
- sym:dist/npm/install.js#unzipWithSystemTools@338f38b05c220795
map_version: '2'
---
# unzipWithSystemTools

Extracts a zip into dest by running tar -xf, falling back to PowerShell Expand-Archive. Throws if both fail. Output goes to the console.

## Behaviour

- tar exits 0 -> returns, PowerShell is not run
- tar fails or is missing -> runs Expand-Archive -Force
- zipPath or dest containing a single quote breaks the PowerShell command, since paths are not escaped

## Terms

zip extraction, unzip, bsdtar, Expand-Archive, system tar, powershell unpack

## Connections

- **Location:** `dist/npm/install.js:94-104`
- **Signature:** `function unzipWithSystemTools(zipPath, dest)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
