---
id: ui-respond
type: summary
status: unverified
title: ui::respond
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::respond
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::respond@834fcc50c778c23f
map_version: '2'
---
# ui::respond

Writes an HTTP/1.1 response with status line, security headers, Content-Length and body, then flushes. Every response sets Connection: close.

## Behaviour

- codes 200, 403, 404 and 405 get their reason phrase; any other code, including 500, is labelled 'Internal Server Error'
- every response sends no-store, nosniff, no-referrer and a CSP header
- Content-Length is the byte length of body

## Terms

http response, write reply, status line, response headers, content type, send bytes, csp header

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:190-209`
- **Signature:** `fn respond(stream: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) -> Result<()>`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Called by:** [[ui-serve]]
