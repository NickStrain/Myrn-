//! Optional embeddings: a small static CPU model (Model2Vec `potion-base-8M`,
//! about 30 MB) that turns text into vectors for semantic search.
//!
//! - The model is downloaded only on `myrn embeddings enable`.
//! - A helper process loads it once, serves requests over localhost, and
//!   exits when idle ([`helper`]); hooks talk to it through [`client`].
//! - Nothing depends on it: when it is missing or not running, callers get
//!   `None` and search is lexical and graph-based as before.

pub mod client;
pub mod helper;
#[cfg(feature = "model")]
pub mod model;

use std::path::{Path, PathBuf};

/// Hugging Face repository of the model.
pub const MODEL_REPO: &str = "minishlab/potion-base-8M";
/// Files the model needs.
pub const MODEL_FILES: [&str; 3] = ["model.safetensors", "tokenizer.json", "config.json"];

/// Where the model is kept (shared by every project): `<data home>/models/potion-base-8M`.
pub fn model_dir(data_home: &Path) -> PathBuf {
    data_home
        .join("models")
        .join(MODEL_REPO.rsplit('/').next().unwrap_or(MODEL_REPO))
}

/// This build can run the model.
pub fn supported() -> bool {
    cfg!(feature = "model")
}

/// The model is downloaded.
pub fn installed(data_home: &Path) -> bool {
    let dir = model_dir(data_home);
    MODEL_FILES[..2].iter().all(|f| dir.join(f).exists())
}

/// L2-normalised vectors: cosine similarity is the dot product.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

pub fn to_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

pub fn from_bytes(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_round_trip() {
        let v = vec![0.5, -1.25, 3.0];
        assert_eq!(from_bytes(&to_bytes(&v)), v);
        assert_eq!(dot(&[0.6, 0.8], &[0.6, 0.8]), 1.0);
    }
}
