---
id: episode-outcome
type: summary
status: unverified
title: Episode::outcome
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::outcome
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::outcome@d1097a0a7412b880
map_version: '2'
---
# Episode::outcome

Maps the episode to one of five status labels. The last test result takes priority over commits and edits.

## Behaviour

- last_test 'pass' -> tests_passed; any other last_test value -> tests_failed
- no test run and at least one commit -> committed, even if files were edited
- no test run, no commits, files edited -> edited; otherwise no_changes

## Terms

status, result, final state, verdict, session result, label

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:42-50`
- **Signature:** `pub fn outcome(&self) -> &'static str`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[episode-persist]], [[engine-attach-outcome]], [[engine-viz-sessions]], [[log-run]], [[log-render-one]], [[trace-run]]
- **Tests:** `folds_a_session`
