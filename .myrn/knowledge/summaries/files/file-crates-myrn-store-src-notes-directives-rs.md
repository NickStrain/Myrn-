---
id: file-crates-myrn-store-src-notes-directives-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/directives.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/directives.rs
anchors:
- file:crates/myrn-store/src/notes/directives.rs@8812617446009245
---
# crates/myrn-store/src/notes/directives.rs

Removes directive-shaped text from note records, such as prompt-override attempts, second-person orders to the agent, chat-role and tool-call markup, tool-call JSON, and download-piped-to-shell commands, when a note is published and again when it is shown. Main parts are the Stripped result type, the lazily compiled regex list, the is_directive check, the sentence splitter, and the strip function that works line by line.

## Connections

- **Location:** `crates/myrn-store/src/notes/directives.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[directives-stripped]], [[directives-patterns]], [[directives-is-directive]], [[directives-sentences]], [[directives-strip]]
- **Imported by:** [[file-crates-myrn-engine-src-pack-render-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
