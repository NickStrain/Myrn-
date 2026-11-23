---
id: helper-answer
type: summary
status: unverified
title: helper::answer
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::answer
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::answer@4cd0a10e9b1117df
map_version: '2'
---
# helper::answer

Handles one connection: reads one JSON line, drops the request if the token is wrong, otherwise encodes the texts and writes the Response line back.

## Behaviour

- Wrong token -> Ok(()) with no reply written.
- Socket is blocking with a 5-second read timeout; a client that sends no newline makes read_line fail after 5 s.
- Malformed JSON or an encode error -> Err, which serve discards.

## Terms

request handler, connection handler, auth check, read timeout, one line protocol, reply writer

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:93-108`
- **Signature:** `fn answer(stream: TcpStream, token: &str, model: &crate::model::StaticModel) -> Result<()>`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Calls:** [[staticmodel-encode]]
- **Called by:** [[helper-serve]]
