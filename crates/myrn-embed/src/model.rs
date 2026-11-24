//! The embedding models. Two kinds, told apart by the model's `config.json`:
//! - static (Model2Vec): one vector per token, averaged; no network runs;
//! - transformer (BERT family, e.g. BGE): a real encoder run on the CPU with
//!   candle, pooled from the first token. Slower, but it reads word order and
//!   context, and ranks better.

use std::io::Read;
use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use safetensors::{Dtype, SafeTensors};

use crate::{MODEL_FILES, MODEL_REPO};

/// Either kind of model.
pub enum Model {
    Static(StaticModel),
    Transformer(Box<TransformerModel>),
}

impl Model {
    pub fn load(dir: &Path) -> Result<Model> {
        if is_transformer(dir) {
            Ok(Model::Transformer(Box::new(TransformerModel::load(dir)?)))
        } else {
            Ok(Model::Static(StaticModel::load(dir)?))
        }
    }

    pub fn dim(&self) -> usize {
        match self {
            Model::Static(m) => m.dim,
            Model::Transformer(m) => m.dim,
        }
    }

    pub fn encode(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        match self {
            Model::Static(m) => m.encode(texts),
            Model::Transformer(m) => m.encode(texts),
        }
    }
}

/// A BERT-family model (`"model_type": "bert"` in config.json).
fn is_transformer(dir: &Path) -> bool {
    std::fs::read_to_string(dir.join("config.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v["model_type"].as_str().map(|s| s == "bert"))
        .unwrap_or(false)
}

/// A BERT encoder; the text's vector is the first ([CLS]) token's final state,
/// normalised, as BGE and Arctic Embed are trained.
pub struct TransformerModel {
    tokenizer: tokenizers::Tokenizer,
    model: candle_transformers::models::bert::BertModel,
    device: candle_core::Device,
    pub dim: usize,
}

/// Longest input in tokens; longer texts are cut (summaries are short).
const MAX_TOKENS: usize = 256;
/// Texts encoded in one forward pass.
const BATCH: usize = 16;

impl TransformerModel {
    pub fn load(dir: &Path) -> Result<TransformerModel> {
        use candle_transformers::models::bert::{BertModel, Config};
        let device = candle_core::Device::Cpu;
        let config: Config = serde_json::from_str(
            &std::fs::read_to_string(dir.join("config.json")).context("reading config.json")?,
        )
        .context("parsing config.json")?;
        let mut tokenizer = tokenizers::Tokenizer::from_file(dir.join("tokenizer.json"))
            .map_err(|e| anyhow!("loading the tokenizer: {e}"))?;
        tokenizer
            .with_truncation(Some(tokenizers::TruncationParams {
                max_length: MAX_TOKENS,
                ..Default::default()
            }))
            .map_err(|e| anyhow!("tokenizer truncation: {e}"))?;
        tokenizer.with_padding(Some(tokenizers::PaddingParams::default()));
        // SAFETY: the file is only read, and not changed while mapped (the
        // download writes a temporary file and renames it into place).
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[dir.join("model.safetensors")],
                candle_core::DType::F32,
                &device,
            )?
        };
        let model = BertModel::load(vb, &config).context("loading the model weights")?;
        Ok(TransformerModel {
            tokenizer,
            model,
            device,
            dim: config.hidden_size,
        })
    }

    pub fn encode(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        use candle_core::{IndexOp, Tensor};
        let mut out = Vec::with_capacity(texts.len());
        for chunk in texts.chunks(BATCH) {
            let encs = self
                .tokenizer
                .encode_batch(chunk.to_vec(), true)
                .map_err(|e| anyhow!("tokenizing: {e}"))?;
            let ids = encs
                .iter()
                .map(|e| Tensor::new(e.get_ids(), &self.device))
                .collect::<candle_core::Result<Vec<_>>>()?;
            let mask = encs
                .iter()
                .map(|e| Tensor::new(e.get_attention_mask(), &self.device))
                .collect::<candle_core::Result<Vec<_>>>()?;
            let ids = Tensor::stack(&ids, 0)?;
            let mask = Tensor::stack(&mask, 0)?;
            let types = ids.zeros_like()?;
            let hidden = self.model.forward(&ids, &types, Some(&mask))?;
            let cls = hidden.i((.., 0))?; // [batch, dim]
            let norm = cls.sqr()?.sum_keepdim(1)?.sqrt()?;
            let cls = cls.broadcast_div(&norm)?;
            out.extend(cls.to_vec2::<f32>()?);
        }
        Ok(out)
    }
}

pub struct StaticModel {
    tokenizer: tokenizers::Tokenizer,
    /// Row-major `[vocab, dim]`.
    table: Vec<f32>,
    /// Optional per-token weights (newer Model2Vec models).
    weights: Option<Vec<f32>>,
    pub dim: usize,
    vocab: usize,
}

impl StaticModel {
    pub fn load(dir: &Path) -> Result<StaticModel> {
        let tokenizer = tokenizers::Tokenizer::from_file(dir.join("tokenizer.json"))
            .map_err(|e| anyhow!("loading the tokenizer: {e}"))?;
        let bytes = std::fs::read(dir.join("model.safetensors"))
            .with_context(|| format!("reading {}", dir.join("model.safetensors").display()))?;
        let st = SafeTensors::deserialize(&bytes)?;
        let emb = st
            .tensor("embeddings")
            .or_else(|_| st.tensor("embedding.weight"))
            .map_err(|e| anyhow!("no embeddings tensor in the model: {e}"))?;
        let shape = emb.shape().to_vec();
        if shape.len() != 2 {
            bail!("embeddings tensor has shape {shape:?}, expected [vocab, dim]");
        }
        let table = to_f32(emb.dtype(), emb.data())?;
        let weights = match st.tensor("weights") {
            Ok(w) => Some(to_f32(w.dtype(), w.data())?),
            Err(_) => None,
        };
        Ok(StaticModel {
            tokenizer,
            table,
            weights,
            vocab: shape[0],
            dim: shape[1],
        })
    }

    /// One normalised vector per text (zero vector for text with no known tokens).
    pub fn encode(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut out = Vec::with_capacity(texts.len());
        for t in texts {
            let enc = self
                .tokenizer
                .encode(t.as_str(), false)
                .map_err(|e| anyhow!("tokenizing: {e}"))?;
            let mut v = vec![0f32; self.dim];
            let mut total = 0f32;
            for &id in enc.get_ids().iter().take(512) {
                let id = id as usize;
                if id >= self.vocab {
                    continue;
                }
                let w = self
                    .weights
                    .as_ref()
                    .and_then(|w| w.get(id).copied())
                    .unwrap_or(1.0);
                let row = &self.table[id * self.dim..(id + 1) * self.dim];
                for (a, b) in v.iter_mut().zip(row) {
                    *a += w * b;
                }
                total += w;
            }
            if total > 0.0 {
                let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 0.0 {
                    v.iter_mut().for_each(|x| *x /= norm);
                }
            }
            out.push(v);
        }
        Ok(out)
    }
}

fn to_f32(dtype: Dtype, data: &[u8]) -> Result<Vec<f32>> {
    Ok(match dtype {
        Dtype::F32 => data
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect(),
        Dtype::F16 => data
            .chunks_exact(2)
            .map(|c| f16_to_f32(u16::from_le_bytes([c[0], c[1]])))
            .collect(),
        Dtype::BF16 => data
            .chunks_exact(2)
            .map(|c| f32::from_bits((u16::from_le_bytes([c[0], c[1]]) as u32) << 16))
            .collect(),
        other => bail!("unsupported weight type {other:?}"),
    })
}

fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) as u32) << 31;
    let exp = ((h >> 10) & 0x1f) as u32;
    let frac = (h & 0x3ff) as u32;
    let bits = match exp {
        0 if frac == 0 => sign,
        0 => {
            // Subnormal: normalise.
            let mut e = 127 - 15 + 1;
            let mut f = frac;
            while f & 0x400 == 0 {
                f <<= 1;
                e -= 1;
            }
            sign | ((e as u32) << 23) | ((f & 0x3ff) << 13)
        }
        0x1f => sign | 0x7f80_0000 | (frac << 13),
        _ => sign | ((exp + 127 - 15) << 23) | (frac << 13),
    };
    f32::from_bits(bits)
}

/// Download the model files into `dir` (over HTTPS from Hugging Face).
/// `progress` gets one line per file.
pub fn download(dir: &Path, progress: &dyn Fn(&str)) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    for f in MODEL_FILES {
        let url = format!("https://huggingface.co/{MODEL_REPO}/resolve/main/{f}");
        progress(&format!("downloading {f}"));
        let resp = ureq::get(&url)
            .call()
            .with_context(|| format!("downloading {url}"))?;
        // Streamed to disk: the weights are hundreds of megabytes.
        let tmp = dir.join(format!("{f}.part"));
        let mut file = std::fs::File::create(&tmp)?;
        std::io::copy(
            &mut resp.into_body().into_reader().take(1024 * 1024 * 1024),
            &mut file,
        )?;
        drop(file);
        std::fs::rename(&tmp, dir.join(f))?;
    }
    // Check it loads before calling it installed.
    Model::load(dir).context("the downloaded model does not load")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_floats() {
        assert_eq!(f16_to_f32(0x3c00), 1.0);
        assert_eq!(f16_to_f32(0xc000), -2.0);
        assert_eq!(f16_to_f32(0x0000), 0.0);
        assert!((f16_to_f32(0x0001) - 5.96e-8).abs() < 1e-9);
    }
}
