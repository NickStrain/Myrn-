---
id: capture-hide-ignored
type: summary
status: unverified
title: capture::hide_ignored
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::hide_ignored
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::hide_ignored@0e9415cb88671ca2
map_version: '2'
---
# capture::hide_ignored

Replaces each whitespace-separated word that names an ignored path with "[ignored path]", keeping surrounding quotes and punctuation. Paths are resolved against the repo root, not the command's working directory.

## Behaviour

- empty ignore set -> text returned unchanged without checks
- "cat secrets/prod.env && ls" with secrets/ ignored -> "cat [ignored path] && ls"
- "secrets/x", (quoted, trailing comma) -> "[ignored path]", with the quotes and comma kept

## Terms

mask path, private path, gitignore word, secrets path, redact ignored, ignored file reference

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:107-135`
- **Signature:** `fn hide_ignored(text: &str, root: &Path, ignore: &Gitignore) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-is-ignored]], [[capture-relative]]
- **Called by:** [[capture-clean]]
