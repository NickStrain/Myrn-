//! myrn diff [from] [to]: how shared knowledge changed between two points.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn run(ctx: &Ctx, from: Option<&str>, to: Option<&str>) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let d = engine.diff_knowledge(from.unwrap_or("HEAD"), to)?;
    ctx.emit(&d, || {
        let mut out = format!("knowledge from {} to {}\n\n", d.from, d.to);
        if d.added.is_empty() && d.removed.is_empty() && d.changed.is_empty() && d.map_changes == 0 {
            out += "No changes to shared knowledge.\n";
        }
        for i in &d.added {
            out += &format!("+ {:<16} {:<10} {}\n", i.id, i.ntype, i.title);
        }
        for i in &d.removed {
            out += &format!("- {:<16} {:<10} {}\n", i.id, i.ntype, i.title);
        }
        for i in &d.changed {
            out += &format!(
                "~ {:<16} {:<10} {}  [{}]\n",
                i.id,
                i.ntype,
                i.title,
                i.detail.as_deref().unwrap_or("")
            );
        }
        if d.map_changes > 0 {
            out += &format!("  project map: {} summaries added, removed or changed\n", d.map_changes);
        }
        out += "\nShared knowledge only (.myrn/knowledge); private notes are not versioned, see `myrn log`.\n";
        out
    })
}
