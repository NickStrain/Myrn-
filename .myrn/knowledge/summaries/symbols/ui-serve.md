---
id: ui-serve
type: summary
status: unverified
title: ui::serve
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::serve
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::serve@3089f440d8d3bf5e
map_version: '2'
---
# ui::serve

Handles one connection: checks GET, Host and token, then serves the embedded page at / or dispatches /api/<name> to an engine query. Names: overview, map, graph, notes, records, record, sessions, session, node, search, explain.

## Behaviour

- Host not 127.0.0.1:port or localhost:port, or token missing or wrong (x-myrn-token header or ?t=) -> 403
- path outside /api/ or unknown api name -> 404; engine error -> 500 with its message
- unparseable or non-GET request -> 405; 10 s read timeout returns an Err with no response written

## Terms

routing, dispatch, auth check, endpoint, api route, request guard, dns rebinding guard, handler

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:93-184`
- **Signature:** `fn serve(engine: &mut Engine, mut stream: TcpStream, port: u16, token: &str) -> Result<()>`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Calls:** [[engine-refresh-if-older]], [[engine-find]], [[engine-explain]], [[engine-episode-log]], [[engine-status]], [[node-location]], [[ui-read-request]], [[ui-json]], [[ui-respond]]
- **Called by:** [[ui-run]]
