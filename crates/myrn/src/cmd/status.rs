//! myrn status: index health, record counts by state, connected agents.

use std::process::ExitCode;

use anyhow::Result;

use super::{thousands, Ctx};

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let s = engine.status()?;
    ctx.emit(&s, || {
        let n = |k: &str| s.edges.get(k).copied().unwrap_or(0);
        let langs: Vec<String> = s
            .files_by_language
            .iter()
            .map(|(l, c)| format!("{l} {c}"))
            .collect();
        let total: i64 = s.files_by_language.values().sum();
        let mut out = format!("myrn  {}\n", s.root);
        out += &format!(
            "project     {}{}\n",
            &s.project_id[..12.min(s.project_id.len())],
            if s.provisional_id { " (temporary)" } else { "" }
        );
        out += &format!("files       {} ({})", thousands(total), langs.join(", "));
        if s.files_with_errors > 0 {
            out += &format!(", {} with syntax errors", s.files_with_errors);
        }
        out += "\n";
        out += &format!(
            "graph       {} symbols, {} tests, {} modules, {} commits\n",
            thousands(s.symbols),
            thousands(s.tests),
            thousands(s.modules),
            thousands(s.commits)
        );
        out += &format!(
            "links       {} calls ({} inferred), {} imports, {} test links, {} commit links\n",
            thousands(n("calls")),
            thousands(s.calls_inferred),
            thousands(n("imports")),
            thousands(n("tests")),
            thousands(n("touches"))
        );
        let rec: Vec<String> = s
            .records_by_state
            .iter()
            .map(|(k, v)| format!("{k} {v}"))
            .collect();
        out += &format!("records     {}\n", rec.join(", "));
        let nt = &s.notes;
        out += &format!("notes       {} private, {} shared", nt.private, nt.shared);
        let mut flags = Vec::new();
        if nt.malformed > 0 {
            flags.push(format!("{} malformed", nt.malformed));
        }
        if nt.dangling > 0 {
            flags.push(format!("{} dangling links", nt.dangling));
        }
        if nt.duplicates > 0 {
            flags.push(format!("{} duplicate ids", nt.duplicates));
        }
        if !flags.is_empty() {
            out += &format!(" ({}; see `myrn doctor`)", flags.join(", "));
        }
        out += "\n";
        if !s.conflicts.is_empty() {
            out += &format!(
                "conflicts   {} pair(s) of records disagree; keep one \
                 (`myrn forget <id>`, or have the agent `supersede` it):\n",
                s.conflicts.len()
            );
            for c in &s.conflicts {
                out += &format!(
                    "            {} ({}) {}\n              vs {} ({}) {}\n",
                    c.a, c.a_visibility, c.a_title, c.b, c.b_visibility, c.b_title
                );
            }
        }
        out += &format!(
            "agents      {}\n",
            if s.connected_agents.is_empty() {
                "none connected".to_string()
            } else {
                s.connected_agents.join(", ")
            }
        );
        out += &format!("store       {}\n", s.db_path);
        out
    })
}
