---
id: publish-show
type: summary
status: unverified
title: publish::show
source: myrn summarize
about: sym:crates/myrn/src/cmd/publish.rs#cmd::publish::show
anchors:
- sym:crates/myrn/src/cmd/publish.rs#cmd::publish::show@2722b3854440bb9f
map_version: '2'
---
# publish::show

Renders the preview text for each planned item: target path, source path, the exact text to be written, removed directive-shaped lines, trimmed evidence, then any plan warnings. Returns an empty string for an empty plan.

## Behaviour

- item text without trailing newline -> newline appended before the closing separator
- item with stripped entries -> one "removed (directive-shaped): ..." line per entry
- plan with warnings -> each printed as "note: ..." after all items

## Terms

preview, review output, stripped directives, trimmed evidence, diff preview, warnings listing, dry-run output

## Connections

- **Location:** `crates/myrn/src/cmd/publish.rs:57-82`
- **Signature:** `fn show(plan: &PublishPlan, root: &Path) -> String`
- **In:** [[file-crates-myrn-src-cmd-publish-rs]]
- **Calls:** [[publish-rel]]
- **Called by:** [[publish-run]]
