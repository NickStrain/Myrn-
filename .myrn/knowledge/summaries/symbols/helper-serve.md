---
id: helper-serve
type: summary
status: unverified
title: helper::serve
source: myrn summarize
about: sym:crates/myrn-embed/src/helper.rs#helper::serve
anchors:
- sym:crates/myrn-embed/src/helper.rs#helper::serve@4511094fd0b00699
map_version: '2'
---
# helper::serve

Loads the model, binds a random 127.0.0.1 port, publishes helper.json, then answers connections until none arrive for idle, and finally removes helper.json if it still belongs to this pid. If another helper already answers a ping on data_home, it returns Ok immediately without loading the model.

## Behaviour

- Existing helper answering ping -> Ok(()) with no bind and no file write.
- helper.json is written to a temp file named with the pid, then renamed into place so readers never see a partial file.
- Idle timer resets on each accepted connection; a non-WouldBlock accept error sleeps 20 ms and the loop continues.

## Terms

background server, embedding daemon, idle timeout, singleton check, shutdown, port publication, keep-alive loop

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs:44-85`
- **Signature:** `pub fn serve(data_home: &Path, idle: Duration) -> Result<()>`
- **In:** [[file-crates-myrn-embed-src-helper-rs]]
- **Calls:** [[client-request]], [[helper-info-path]], [[helper-read-info]], [[helper-answer]], [[helper-token]], [[model-dir]], [[staticmodel-load]]
- **Called by:** [[main-main]]
