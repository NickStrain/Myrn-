//! myrn publish <id>: move a private record into .myrn/knowledge/ after review; strips directive-shaped text.
//!
//! Shows the exact text that will be committed first, and asks. Nothing is
//! committed: the file is left for `git add` and a normal review.

use std::io::{BufRead, IsTerminal, Write};
use std::path::Path;
use std::process::ExitCode;

use anyhow::{bail, Result};
use myrn_engine::sharing::PublishPlan;

use super::Ctx;

pub fn run(ctx: &Ctx, id: &str, yes: bool, dry_run: bool) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let root = engine.root().to_path_buf();
    let plan = engine.publish_plan(id)?;
    if dry_run {
        return ctx.emit(&plan, || show(&plan, &root));
    }
    if !yes {
        if !std::io::stdin().is_terminal() {
            bail!(
                "`myrn publish` writes into the committed knowledge directory; review it with \
                 --dry-run, then pass --yes"
            );
        }
        eprint!("{}", show(&plan, &root));
        eprint!("Publish? [y/N] ");
        std::io::stderr().flush()?;
        let mut answer = String::new();
        std::io::stdin().lock().read_line(&mut answer)?;
        if !matches!(answer.trim(), "y" | "Y" | "yes") {
            eprintln!("Nothing published.");
            return Ok(ExitCode::from(1));
        }
    }
    let done = engine.publish(id)?;
    ctx.emit(&done, || {
        let mut out = String::new();
        for it in &done.items {
            out += &format!("Published {} -> {}\n", it.id, rel(&it.to, &root));
        }
        out += "Commit it for the team: git add .myrn/knowledge && git commit\n";
        out
    })
}

fn rel(p: &Path, root: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .replace('\\', "/")
}

fn show(plan: &PublishPlan, root: &Path) -> String {
    let mut out = String::new();
    for it in &plan.items {
        out += &format!(
            "── {} (moved from {})\n",
            rel(&it.to, root),
            it.from.display()
        );
        out += &it.text;
        if !it.text.ends_with('\n') {
            out.push('\n');
        }
        out += "──\n";
        for s in &it.stripped {
            out += &format!("removed (directive-shaped): {s}\n");
        }
        for t in &it.trimmed {
            out += &format!("evidence trimmed: {t}\n");
        }
        out.push('\n');
    }
    for w in &plan.warnings {
        out += &format!("note: {w}\n");
    }
    out
}
