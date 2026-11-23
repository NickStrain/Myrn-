---
id: download
type: summary
status: unverified
title: download
source: myrn summarize
about: sym:dist/npm/install.js#download
anchors:
- sym:dist/npm/install.js#download@fb021fbe1e362b5b
map_version: '2'
---
# download

Fetches a URL, following redirects, and returns the response body as a Buffer. Throws with the URL and HTTP status when the response is not ok.

## Behaviour

- 200 response -> Buffer of the full body
- 404 response -> throws 'download failed: <url> (404 Not Found)'
- No timeout or retry is configured

## Terms

http get, fetch, release asset download, redirect follow, request body, network request

## Connections

- **Location:** `dist/npm/install.js:58-62`
- **Signature:** `async function download(url)`
- **In:** [[file-dist-npm-install-js]]
- **Called by:** [[install]]
