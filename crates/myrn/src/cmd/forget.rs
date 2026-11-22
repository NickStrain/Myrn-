//! myrn forget <id>: permanently remove a record and anything derived only from it.

use std::io::{BufRead, IsTerminal, Write};
use std::process::ExitCode;

use anyhow::{bail, Result};

use super::Ctx;

pub fn run(ctx: &Ctx, id: &str, yes: bool) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let plan = engine.forget_plan(id)?;
    if !yes {
        if !std::io::stdin().is_terminal() {
            bail!("`myrn forget` removes records permanently; pass --yes to confirm");
        }
        eprintln!("This permanently removes:");
        for i in &plan.ids {
            eprintln!("  record {i}");
        }
        for f in &plan.files {
            eprintln!("  file   {}", f.display());
        }
        for o in &plan.unlinked_from {
            eprintln!("  the link to it from {o}");
        }
        eprint!("Remove? [y/N] ");
        std::io::stderr().flush()?;
        let mut answer = String::new();
        std::io::stdin().lock().read_line(&mut answer)?;
        if !matches!(answer.trim(), "y" | "Y" | "yes") {
            eprintln!("Nothing removed.");
            return Ok(ExitCode::from(1));
        }
    }
    let done = engine.forget(id)?;
    ctx.emit(&done, || {
        let mut out = format!("Forgot {}.\n", done.ids.join(", "));
        if !done.unlinked_from.is_empty() {
            out += &format!(
                "Removed links to it from {}.\n",
                done.unlinked_from.join(", ")
            );
        }
        out +=
            "Shared notes are removed from the working tree; commit to remove them for the team. \
                Git history still has them.\n";
        out
    })
}
