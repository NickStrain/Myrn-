---
id: ui-request
type: summary
status: unverified
title: ui::Request
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::Request
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::Request@af96a96f7f2d24ed
map_version: '2'
---
# ui::Request

Parsed HTTP request: the percent-decoded path, the query map, and headers keyed by lower-cased name.

## Behaviour

- GET /a%20b?x=1 -> path '/a b', query {x: '1'}
- repeated header name -> last value wins
- header reading stops at blank line, EOF, or more than 100 distinct header names

## Terms

parsed request, query string, url parameters, http headers, request fields

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:46-50`
- **Signature:** `struct Request`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
