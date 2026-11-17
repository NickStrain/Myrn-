---
id: bug-7f3a91
type: bug
status: verified
caused_by: "[[rootcause-a12b08]]"
fixed_in: HEAD_SHA
anchors:
  - unique_everseen@BODY_HASH
evidence:
  - test: tests/test_recipes.py
tags: [iterators]
---
`unique_everseen` raised TypeError for unhashable items instead of falling back to a list.
Seen in [[fact-c0ffee]]; root cause in [[rootcause-a12b08]].
