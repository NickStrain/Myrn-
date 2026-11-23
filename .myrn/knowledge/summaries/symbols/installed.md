---
id: installed
type: summary
status: unverified
title: installed
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#installed
anchors:
- sym:crates/myrn-embed/src/lib.rs#installed@f74a4e2188cf662f
map_version: '2'
---
# installed

Reports whether the first two MODEL_FILES, `model.safetensors` and `tokenizer.json`, exist in the model directory.

## Behaviour

- Both files present in model_dir -> true
- Either of those two files missing -> false
- `config.json` is not checked, and existence is tested only, not file contents or size

## Terms

model downloaded, model present, model files, is installed, enabled check, files exist

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:35-38`
- **Signature:** `pub fn installed(data_home: &Path) -> bool`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Calls:** [[model-dir]]
- **Called by:** [[client-embed]], [[engine-embeddings-on]], [[engine-enable-embeddings]], [[engine-embeddings-status]]
