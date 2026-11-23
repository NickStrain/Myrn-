---
id: show-run
type: summary
status: unverified
title: show::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/show.rs#cmd::show::run
anchors:
- sym:crates/myrn/src/cmd/show.rs#cmd::show::run@c6b379a846d9a872
map_version: '2'
---
# show::run

Looks up the node with the given id through the engine and emits it, using a text renderer that lists its non-empty fields and relations. Errors from the engine lookup are returned with ?.

## Behaviour

- origin "earned" with extra as a JSON object -> prints file, source, and one evidence line per entry as "kind: value", or just the value when kind is empty
- extra that is missing, invalid JSON, or not an object -> prints the raw extra string under the label "extra"
- edge whose src equals the node id -> "->" then rel and dst; otherwise "<-" then rel and src; status is appended in parentheses when set

## Terms

show node, node details, node fields, relations, edges, earned evidence, text render, node lookup

## Connections

- **Location:** `crates/myrn/src/cmd/show.rs:9-71`
- **Signature:** `pub fn run(ctx: &Ctx, id: &str) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-show-rs]]
- **Calls:** [[kind-as-str]], [[engine-show]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
