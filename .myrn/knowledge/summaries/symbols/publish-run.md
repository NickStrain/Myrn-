---
id: publish-run
type: summary
status: unverified
title: publish::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/publish.rs#cmd::publish::run
anchors:
- sym:crates/myrn/src/cmd/publish.rs#cmd::publish::run@46ea7a3f39ea18f5
map_version: '2'
---
# publish::run

Builds a publish plan for the record id and either prints it (dry run), prompts for confirmation, or publishes it and prints the moved paths plus a git add hint. Nothing is committed.

## Behaviour

- dry_run true -> emits the plan via ctx.emit and returns without writing
- yes false and stdin not a terminal -> bails with an error asking for --dry-run review then --yes
- yes false on a terminal -> answer other than y, Y, or yes prints "Nothing published." and returns exit code 1

## Terms

publish, promote record, share with team, dry run, confirmation prompt, non-interactive guard, --yes flag, commit knowledge

## Connections

- **Location:** `crates/myrn/src/cmd/publish.rs:15-48`
- **Signature:** `pub fn run(ctx: &Ctx, id: &str, yes: bool, dry_run: bool) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-publish-rs]]
- **Calls:** [[engine-root]], [[engine-publish-plan]], [[engine-publish]], [[ctx-engine]], [[ctx-emit]], [[publish-rel]], [[publish-show]]
- **Called by:** [[main-main]]
