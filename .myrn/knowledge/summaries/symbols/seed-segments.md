---
id: seed-segments
type: summary
status: unverified
title: seed::segments
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::segments
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::segments@ca2efea14b6f943a
---
# seed::segments

Splits a qualified name into segments on `::` and `.`, dropping empty parts and a trailing `#<digits>` collision suffix. Note the `#n` strip only applies when the suffix is all digits.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:100-110`
- **Signature:** `pub fn segments(q: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[engine-pick]], [[seed-find]], [[seed-ends-with-segments]], [[engine-summarize]], [[summarize-title-of]]
- **Tests:** `segment_matching`
