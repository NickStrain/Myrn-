---
id: summarize-summarize-file
type: summary
status: unverified
title: summarize::summarize_file
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_file
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::summarize_file@b34e2bffdfb1a72e
map_version: '2'
---
# summarize::summarize_file

Sends one file's source (cut past 160000 bytes) and its stale symbol list to the model with the style prompt, then parses the answer with ask_json into a FileOut.

## Behaviour

- source longer than 160000 bytes is cut at a character boundary and marked '... (cut: file continues)'
- reads cwd joined with the job path; an unreadable file fails with 'reading <path>' and makes no model call
- stale symbols are listed as qualified name, kind and line range; none -> the prompt asks for an empty symbols object

## Terms

file prompt, source truncation, stale symbols, per-file request, file summary, model prompt

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:702-750`
- **Signature:** `fn summarize_file(command: &[String], cwd: &Path, job: &FileJob) -> Result<FileOut>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-sym-out]], [[summarize-ask-json]]
- **Called by:** [[engine-summarize]]
