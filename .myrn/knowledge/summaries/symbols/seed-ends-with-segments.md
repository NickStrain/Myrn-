---
id: seed-ends-with-segments
type: summary
status: unverified
title: seed::ends_with_segments
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::ends_with_segments
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::ends_with_segments@7ea86e0e1f28e0a2
---
# seed::ends_with_segments

Returns true if the qualname's segments end with the wanted segments (suffix match on segments, not substring).

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:112-115`
- **Signature:** `fn ends_with_segments(qname: &str, want: &[String]) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-segments]]
- **Called by:** [[seed-find]]
- **Tests:** `segment_matching`
