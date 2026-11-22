//! myrn summarize: write the project map (summary notes for the project, every
//! directory, file and function), asking the model only about code that changed.

use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::summarize::Options;

use super::{thousands, Ctx};

pub fn run(
    ctx: &Ctx,
    dry_run: bool,
    limit: Option<usize>,
    jobs: Option<usize>,
) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let cfg = engine.config.summarize.clone();
    let opts = Options {
        dry_run,
        limit,
        jobs: jobs.unwrap_or(cfg.jobs),
        command: cfg.command,
    };
    let quiet = ctx.json;
    let report = engine.summarize(&opts, &|line: &str| {
        if !quiet {
            eprintln!("{line}");
        }
    })?;
    let failed = !report.failures.is_empty();
    let _ = ctx.emit(&report, || {
        let mut out = String::new();
        if dry_run {
            out.push_str(&format!(
                "Out of date: {} of {} files, {} of {} functions and types, about {} directories\n",
                report.stale_files,
                thousands(report.files as i64),
                report.stale_symbols,
                thousands(report.symbols as i64),
                report.stale_dirs
            ));
            out.push_str("Run `myrn summarize` to write them.\n");
            return out;
        }
        out.push_str(&format!(
            "Project map: {} files, {} functions and types, {} directories\n",
            thousands(report.files as i64),
            thousands(report.symbols as i64),
            report.dirs
        ));
        out.push_str(&format!(
            "  updated {} files, {} functions and types, {} directories in {} model calls (${:.4})\n",
            report.updated_files,
            report.updated_symbols,
            report.updated_dirs,
            report.model_calls,
            report.cost_usd
        ));
        out.push_str(&format!(
            "  {} notes written, {} removed, in {}\n",
            report.notes_written,
            report.notes_removed,
            report.vault.join("summaries").display()
        ));
        if report.deferred_files > 0 {
            out.push_str(&format!(
                "  {} files left for the next run (--limit)\n",
                report.deferred_files
            ));
        }
        for f in &report.failures {
            out.push_str(&format!("  failed: {f}\n"));
        }
        out.push_str(&format!(
            "Open {} in Obsidian to browse the graph.\n",
            report.vault.display()
        ));
        out
    })?;
    Ok(if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}
