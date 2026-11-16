//! myrn init: index the repository and create the local store (code only, no session capture).

use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::api::Engine;

use super::{thousands, Ctx};

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let (engine, report) = Engine::init(&cwd, ctx.data_dir.as_deref())?;
    ctx.emit(&report, || {
        let s = engine.status().ok();
        let mut out = format!("Initialised myrn in {}\n", engine.root().display());
        out += &format!(
            "  indexed {} files: {} symbols, {} commits in {:.1}s\n",
            thousands(report.files as i64),
            thousands(report.symbols as i64),
            thousands(report.commits as i64),
            report.millis as f64 / 1000.0
        );
        if let Some(e) = &report.edges {
            out += &format!(
                "  links: {} calls ({} inferred), {} imports, {} test links\n",
                thousands(e.calls as i64),
                thousands(e.calls_inferred as i64),
                thousands(e.imports as i64),
                thousands(e.tests as i64)
            );
        }
        if engine.provisional_id {
            out += "  note: no commits yet; knowledge uses a temporary id until the first commit\n";
        }
        if let Some(s) = s {
            if s.files_with_errors > 0 {
                out += &format!(
                    "  {} files had syntax errors and were indexed as far as possible\n",
                    s.files_with_errors
                );
            }
        }
        out += &format!("  shared config: {}\n", engine.layout.repo_dir.display());
        out += &format!("  private store: {}\n", engine.layout.data_dir.display());
        out += "\nNo session data is recorded. Try `myrn explain <symbol>` or `myrn status`.\n";
        out
    })
}
