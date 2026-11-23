---
id: init-run
type: summary
status: unverified
title: init::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/init.rs#cmd::init::run
anchors:
- sym:crates/myrn/src/cmd/init.rs#cmd::init::run@bef1010607eb9db1
map_version: '2'
---
# init::run

Initialises the engine in the current directory, indexes the repository, and emits the report. The text output lists indexed counts, optional link counts, a temporary-id note, syntax-error count, and the shared-config and private-store paths.

## Behaviour

- current_dir failure -> returns Err before any indexing happens
- report.edges None -> the links line is omitted
- engine.provisional_id true -> adds a note that a temporary id is used until the first commit

## Terms

index repository, setup, initialise, create store, first run, bootstrap, syntax errors, temporary id

## Connections

- **Location:** `crates/myrn/src/cmd/init.rs:10-48`
- **Signature:** `pub fn run(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-init-rs]]
- **Calls:** [[engine-init]], [[engine-root]], [[engine-status]], [[ctx-emit]], [[cmd-thousands]]
- **Called by:** [[main-main]]
