---
id: engine-test-needles
type: summary
status: unverified
title: Engine::test_needles
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::test_needles
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::test_needles@7a89aaad2d7700c0
map_version: '2'
---
# Engine::test_needles

Turns a cited test target into a label and the strings a passing command must contain to count.

## Behaviour

- Target resolves to a test node -> title, path, and file stem (length 4 or more, not test, tests, mod, lib) as needles
- Target not found -> the target string itself is the only needle
- Label is: cited test `target` passed

## Terms

match strings, file stem, cited target, test name

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:297-316`
- **Signature:** `fn test_needles(&self, target: &str) -> Result<(String, Vec<String>)>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Called by:** [[engine-verified-by-test]]
