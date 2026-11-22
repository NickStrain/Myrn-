//! myrn gc: merge duplicate note copies, drop index entries for deleted code,
//! compact old session logs. Never removes an earned record.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn run(ctx: &Ctx, older_than: u64, dry_run: bool) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let r = engine.gc(older_than, dry_run)?;
    ctx.emit(&r, || {
        let would = if dry_run { "would be " } else { "" };
        let mut out = String::new();
        out += &format!(
            "duplicate note copies {would}removed: {}\n",
            r.duplicates_removed.len()
        );
        for k in &r.duplicates_differing {
            out += &format!("  differs from the indexed copy, left for you: {k}\n");
        }
        out += &format!("index rows {would}dropped: {}\n", r.index_rows_dropped);
        out += &format!(
            "session logs older than {older_than} days {would}compacted: {} ({} lines)\n",
            r.episodes_compacted, r.episode_lines_dropped
        );
        if !dry_run {
            out += &format!(
                "database: {:.1} MB -> {:.1} MB\n",
                r.bytes_before as f64 / 1e6,
                r.bytes_after as f64 / 1e6
            );
        }
        out
    })
}
