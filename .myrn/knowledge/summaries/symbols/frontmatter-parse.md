---
id: frontmatter-parse
type: summary
status: unverified
title: frontmatter::parse
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::parse
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::parse@a92f294521456283
---
# frontmatter::parse

Parses a file into a Parsed given its text, file stem and folder. Defaults: missing id uses the stem, missing type uses the folder's type or 'fact', missing status becomes 'unverified'. Unknown types fall back to the folder type; unknown statuses become 'unverified'; id must equal the lowercased stem; confidence must be 0 to 1; anchors without '@' are kept with hash None; 'fixed_in' creates an inverse Fixes link; unknown relation-like keys go to extra with a problem unless in the tolerated list (tags, aliases, created, etc.); evidence items must be a single-key mapping or a scalar. 'no id' and 'no type' problems only appear when a header exists.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:208-382`
- **Signature:** `pub fn parse(text: &str, stem: &str, folder: Option<&str>) -> Parsed`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[status-as-str]], [[status-parse]], [[relation-parse]], [[note-new]], [[frontmatter-split]], [[frontmatter-scalar]], [[frontmatter-strings]], [[layout-safe-stem]], [[wikilink-unwrap-link]]
- **Called by:** [[notes-write]], [[sync-sync]]
- **Tests:** `parses_the_design_example`, `round_trips`, `round_trips_lists_extra_and_crlf`, `malformed_headers_are_flagged_not_dropped`, `writes_into_type_folder_with_content_id`
