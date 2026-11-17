//! myrn debug: developer tools. `review` builds the Gate 2 checklist of learned
//! records and scores it once marked.

use std::path::Path;
use std::process::ExitCode;

use anyhow::{Context, Result};
use myrn_engine::review;

use super::Ctx;

/// `myrn debug review --sample N [--out FILE] [--seed S]`, or `--score FILE`.
pub fn review(
    ctx: &Ctx,
    sample: usize,
    seed: u64,
    out: Option<&Path>,
    score: Option<&Path>,
) -> Result<ExitCode> {
    if let Some(file) = score {
        let text =
            std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
        let s = review::score(&text);
        return ctx.emit(&s, || render_score(&s));
    }
    let engine = ctx.engine()?;
    let all = engine.review_items()?;
    let total = all.len();
    let items = review::sample(all, sample, seed);
    if ctx.json {
        println!("{}", serde_json::to_string_pretty(&items)?);
        return Ok(ExitCode::SUCCESS);
    }
    let text = review::checklist(
        &items,
        &format!(
            "Record review: {} of {total} records (seed {seed})",
            items.len()
        ),
    );
    match out {
        Some(p) => {
            std::fs::write(p, &text).with_context(|| format!("writing {}", p.display()))?;
            println!("Wrote {} records to {}", items.len(), p.display());
        }
        None => print!("{text}"),
    }
    Ok(ExitCode::SUCCESS)
}

pub fn render_score(s: &review::Score) -> String {
    let pct = |x: Option<f64>| {
        x.map(|v| format!("{:.0}%", v * 100.0))
            .unwrap_or("–".into())
    };
    let mut out = format!(
        "precision   {} correct of {} marked ({}); not wrong {}\n\
         marks       {} correct, {} vague, {} wrong; {} unmarked, {} with more than one box\n",
        s.correct,
        s.correct + s.vague + s.wrong,
        pct(s.precision),
        pct(s.not_wrong),
        s.correct,
        s.vague,
        s.wrong,
        s.unmarked,
        s.ambiguous
    );
    for (t, c, m) in &s.by_type {
        out += &format!("  {t:<11} {c} of {m} correct\n");
    }
    out
}
