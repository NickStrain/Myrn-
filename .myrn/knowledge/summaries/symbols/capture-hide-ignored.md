---
id: capture-hide-ignored
type: summary
status: unverified
title: capture::hide_ignored
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::hide_ignored
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::hide_ignored@0e9415cb88671ca2
---
# capture::hide_ignored

Replaces each space-separated word whose core (after trimming quotes and punctuation) contains '/' or '.' and matches the ignore rules with [ignored path], resolving relative candidates against root. Returns the text unchanged when the ignore set is empty or nothing matched.

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:107-135`
- **Signature:** `fn hide_ignored(text: &str, root: &Path, ignore: &Gitignore) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Calls:** [[capture-is-ignored]], [[capture-relative]]
- **Called by:** [[capture-clean]]
