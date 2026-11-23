---
id: staticmodel-encode
type: summary
status: unverified
title: StaticModel::encode
source: myrn summarize
about: sym:crates/myrn-embed/src/model.rs#model::StaticModel::encode
anchors:
- sym:crates/myrn-embed/src/model.rs#model::StaticModel::encode@07398bb1a1cee344
map_version: '2'
---
# StaticModel::encode

Tokenizes each text without special tokens, sums the rows of known token ids (scaled by per-token weights when present), and returns one L2-normalised vector per text. Input order is kept.

## Behaviour

- text with no known token ids, or empty text -> zero vector of length dim
- only the first 512 token ids are used; ids >= vocab are skipped
- weights sum to 0 or less -> vector is returned un-normalised; norm 0 leaves it zero

## Terms

embed text, vectorize, text to vector, mean pooling, normalise, zero vector, batch embedding, truncation, unknown tokens

## Connections

- **Location:** `crates/myrn-embed/src/model.rs:53-87`
- **Signature:** `pub fn encode(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-embed-src-model-rs]]
- **Calls:** [[staticmodel-encode]]
- **Called by:** [[client-local]], [[helper-answer]], [[staticmodel-encode]]
