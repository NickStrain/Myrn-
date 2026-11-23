---
id: client-request
type: summary
status: unverified
title: client::request
source: myrn summarize
about: sym:crates/myrn-embed/src/client.rs#client::request
anchors:
- sym:crates/myrn-embed/src/client.rs#client::request@3d33443d8961da95
map_version: '2'
---
# client::request

Sends texts to the running helper over 127.0.0.1 with its token and returns the vectors, or None if any step fails.

## Behaviour

- no helper info file (read_info fails) -> None without connecting
- connect takes more than 200 ms or read takes more than 10 s -> None
- response vector count differs from texts length -> None; empty texts match an empty response

## Terms

helper call, socket request, loopback, IPC, token, fetch vectors, TCP client

## Connections

- **Location:** `crates/myrn-embed/src/client.rs:13-32`
- **Signature:** `pub fn request(data_home: &Path, texts: &[String]) -> Option<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-embed-src-client-rs]]
- **Calls:** [[helper-read-info]]
- **Called by:** [[client-embed]], [[client-start]], [[helper-serve]], [[engine-embeddings-status]]
