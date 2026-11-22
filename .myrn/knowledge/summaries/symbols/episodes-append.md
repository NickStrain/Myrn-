---
id: episodes-append
type: summary
status: unverified
title: episodes::append
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::append
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::append@7f7971a1af96cfc0
---
# episodes::append

Creates dir if missing, serializes the entry as one JSON line, and writes it to the session file opened in create+append mode, returning the path. Errors if serialization or open/write fails. The single write_all per line, combined with append mode, is what the code relies on so concurrent hooks don't interleave lines.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:80-92`
- **Signature:** `pub fn append(dir: &Path, session: &str, entry: &Entry) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Calls:** [[episodes-path-for]]
- **Called by:** [[engine-capture]], [[engine-records-request]], [[session]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `append_and_read_round_trip`
