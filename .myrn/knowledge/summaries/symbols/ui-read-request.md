---
id: ui-read-request
type: summary
status: unverified
title: ui::read_request
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::read_request
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::read_request@e2b6bf046a296231
map_version: '2'
---
# ui::read_request

Reads one HTTP request line and its headers from the stream. Returns None for EOF, a malformed request line, or any method other than GET.

## Behaviour

- 'GET /x?a=1&b HTTP/1.1' -> path '/x', query a='1', b=''
- empty stream, 'POST /', or missing target -> None
- query is split on raw & and = before decoding, so %26 in a value does not split it; pairs with empty key are dropped

## Terms

request line, http parse, method check, get only, query parsing, header parsing, read socket

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:52-91`
- **Signature:** `fn read_request(stream: &TcpStream) -> Result<Option<Request>>`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Calls:** [[ui-decode]]
- **Called by:** [[ui-serve]]
