//! myrn rebuild: recreate the database from the code and the notes (note files are only read).

use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::api::Engine;

use super::{thousands, Ctx};

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let mut engine = Engine::open(&cwd, ctx.data_dir.as_deref())?;
    let report = engine.rebuild()?;
    ctx.emit(&report, || {
        format!(
            "Rebuilt the index: {} files, {} symbols, {} commits, {} notes in {:.1}s\n",
            thousands(report.files as i64),
            thousands(report.symbols as i64),
            thousands(report.commits as i64),
            thousands(report.notes.notes as i64),
            report.millis as f64 / 1000.0
        )
    })
}
