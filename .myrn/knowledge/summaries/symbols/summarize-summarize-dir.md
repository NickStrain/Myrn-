---
id: summarize-summarize-dir
type: summary
status: unverified
title: summarize::summarize_dir
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_dir
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_dir@4d6a4cad30f1ee1c
map_version: '2'
---
# summarize::summarize_dir

Asks the model for a short summary of one directory from its child listing, or of the whole project (empty path) with the README excerpt added.

## Behaviour

- project root asks for 2-4 sentences plus the README; other directories ask for 1-3 sentences with no README
- an answer with no non-blank summary key -> error 'no summary in the answer'
- the _cwd parameter is unused; the call goes through run_model, which runs in the temp directory

## Terms

directory summary, project summary, rollup prompt, readme context, listing prompt

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:752-786`
- **Signature:** `fn summarize_dir( command: &[String], _cwd: &Path, job: &DirJob, readme: Option<&str>, ) -> Result<(String, f64)>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-ask-json]], [[summarize-clean]]
- **Called by:** [[engine-summarize]]
