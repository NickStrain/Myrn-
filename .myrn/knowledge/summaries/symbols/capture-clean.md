---
id: capture-clean
type: summary
status: unverified
title: capture::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::clean
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::clean@e88d942ead3ac156
map_version: '2'
---
# capture::clean

Hides ignored paths, redacts secrets, then clips the text to `max` characters via episode::clip. Used with limits of 4000 for prompts, 1000 for commands and 2000 for errors.

## Behaviour

- "cat secrets/prod.env" with secrets/ ignored -> "cat [ignored path]" before redaction and clipping
- "GITHUB_TOKEN=abc123xyz gh pr list" -> "GITHUB_TOKEN=[REDACTED:secret] gh pr list"
- text longer than max -> clipped by episode::clip; the unit is not shown in this file

## Terms

sanitize, scrub text, mask secrets, truncate, length limit, private text, redacted line

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:101-104`
- **Signature:** `fn clean(text: &str, max: usize, root: &Path, ignore: &Gitignore) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[episode-clip]], [[capture-hide-ignored]], [[redact-redact]]
- **Called by:** [[capture-to-entry]]
