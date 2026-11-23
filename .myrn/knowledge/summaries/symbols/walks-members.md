---
id: walks-members
type: summary
status: unverified
title: walks::members
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::members
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::members@4d6c1702cef9992d
map_version: '2'
---
# walks::members

Lists the direct child symbols of a struct, enum, class, trait, interface, union or type node, found by qname prefix. Rust uses :: as the separator and every other language uses a dot.

## Behaviour

- Node with no path or qname, or a kind outside the type list (e.g. function) -> empty Vec, no query run
- Rust qname a::S -> matches a::S::x and a::S::y, drops a::S::x::z because the remainder still contains ::
- Results from the node's own path come first, then ordered by start_line; % and _ in qname are escaped so they match literally

## Terms

methods, nested items, direct members, child symbols, class members, inner items, contained symbols

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:10-35`
- **Signature:** `pub fn members(conn: &Connection, n: &Node) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
- **Calls:** [[graph-nodes-where]]
- **Called by:** [[engine-symbol-card]], [[engine-impact]]
