---
id: absolute
type: summary
status: unverified
title: absolute
source: myrn summarize
about: sym:bench/src/main.rs#absolute
anchors:
- sym:bench/src/main.rs#absolute@c808a6a9d663bff5
---
# absolute

Returns the path unchanged if it is absolute, otherwise joins it onto the current directory; it does not canonicalize or check that the path exists.

## Connections

- **Location:** `bench/src/main.rs:442-448`
- **Signature:** `fn absolute(p: &Path) -> Result<PathBuf>`
- **In:** [[file-bench-src-main-rs]]
- **Called by:** [[real-main]]
