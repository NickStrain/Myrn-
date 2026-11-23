//! myrn embeddings enable|disable|status: optional semantic search with a
//! small local model. The model is downloaded only by `enable`.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn enable(ctx: &Ctx) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let quiet = ctx.json;
    let r = engine.enable_embeddings(&|line: &str| {
        if !quiet {
            eprintln!("{line}");
        }
    })?;
    ctx.emit(&r, || {
        format!(
            "Semantic search is on: {} vectors ({} embedded now) in {:.1}s.\n\
             Tasks now also match code and records by meaning. `myrn embeddings disable` turns it off.\n",
            r.total,
            r.embedded,
            r.millis as f64 / 1000.0
        )
    })
}

pub fn disable(ctx: &Ctx) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    engine.disable_embeddings()?;
    ctx.emit(&serde_json::json!({"enabled": false}), || {
        "Semantic search is off here; vectors dropped. The model stays downloaded for other projects.\n".into()
    })
}

pub fn status(ctx: &Ctx) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let s = engine.embeddings_status()?;
    ctx.emit(&s, || {
        if !s.supported {
            return "This build of myrn has no embedding support.\n".into();
        }
        format!(
            "semantic search  {}\nmodel            {} ({})\nmodel files      {}\nvectors          {}\nhelper           {}\n",
            if s.enabled { "on" } else { "off (`myrn embeddings enable`)" },
            s.model,
            if s.installed { "downloaded" } else { "not downloaded" },
            s.model_dir.display(),
            s.vectors,
            if s.helper_running { "running" } else { "not running (starts on first use)" }
        )
    })
}
