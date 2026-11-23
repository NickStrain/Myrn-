---
id: ui-run
type: summary
status: unverified
title: ui::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::run
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::run@5b9a2f29f22fc676
map_version: '2'
---
# ui::run

Binds 127.0.0.1 on the given port, prints the tokened URL, optionally opens it, then serves connections one at a time until the listener stops. Errors from individual connections are printed and the loop continues.

## Behaviour

- port 0 -> OS picks a free port and the printed URL uses the real port
- bind failure -> error with context 'listening on 127.0.0.1:<port>'
- accept error is skipped silently; serve error is printed to stderr

## Terms

start server, launch, main loop, listen, bind port, entry point, startup

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:24-44`
- **Signature:** `pub fn run(ctx: &Ctx, port: u16, open: bool) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Calls:** [[engine-root]], [[ctx-engine]], [[ui-serve]], [[ui-token]], [[ui-open-browser]]
- **Called by:** [[main-main]]
