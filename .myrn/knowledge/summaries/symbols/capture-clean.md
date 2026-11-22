---
id: capture-clean
type: summary
status: unverified
title: capture::clean
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::clean
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::clean@e88d942ead3ac156
---
# capture::clean

Applies hide_ignored, then redact::redact, then episode::clip with the given max, in that order, to prompts, commands and errors.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:101-104`
- **Signature:** `fn clean(text: &str, max: usize, root: &Path, ignore: &Gitignore) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[episode-clip]], [[capture-hide-ignored]], [[redact-redact]]
- **Called by:** [[capture-to-entry]]
