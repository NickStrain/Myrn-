//! myrn trace <id>: where a record came from (session, commits, tests) and the
//! code it depends on, like `git blame` for knowledge.

use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::capture::episode::clip;

use super::log::datetime;
use super::Ctx;

pub fn run(ctx: &Ctx, id: &str) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let t = engine.trace(id)?;
    ctx.emit(&t, || {
        let r = &t.record;
        let mut out = format!("{}  {}  {}\n", r.id, r.ntype, r.title);
        let state = if r.status == r.written_status {
            r.status.clone()
        } else {
            format!("{} (written as {})", r.status, r.written_status)
        };
        out += &format!("status    {state}\n");
        out += &format!("file      {} ({})\n", r.file, r.visibility);
        match (&t.session, &r.source) {
            (Some(e), _) => {
                out += &format!(
                    "session   {} ({}), {} UTC\n",
                    e.session,
                    e.agent,
                    datetime(e.started)
                );
                if let Some(task) = &e.task {
                    out += &format!(
                        "  task    {}\n",
                        clip(task.lines().next().unwrap_or(""), 90)
                    );
                }
                out += &format!("  outcome {}: {}\n", e.outcome(), e.summary());
            }
            (None, Some(s)) => out += &format!("source    {s} (session log not found)\n"),
            (None, None) => out += "source    written by hand\n",
        }
        for c in &t.commits {
            out += &format!(
                "commit    {} {}  {}  ({} UTC)\n",
                c.rel,
                &c.sha[..12],
                c.subject,
                datetime(c.time)
            );
        }
        for x in &t.tests {
            out += &format!("test      {x}\n");
        }
        for x in &t.test_runs {
            out += &format!("test run  {x}\n");
        }
        for a in &t.anchors {
            out += &format!(
                "anchor    {:<8} {}  {}\n",
                a.state,
                a.target,
                a.location.as_deref().unwrap_or("")
            );
        }
        for x in &t.supersedes {
            out += &format!("replaces  {x}\n");
        }
        for x in &t.superseded_by {
            out += &format!("replaced by {x}\n");
        }
        for x in &t.contradicts {
            out += &format!("conflicts {x}\n");
        }
        if !t.note_history.is_empty() {
            out += "history\n";
            for c in &t.note_history {
                out += &format!(
                    "  {}  {}  {:<8} {}\n",
                    datetime(c.time),
                    &c.sha[..12],
                    c.rel,
                    c.subject
                );
            }
        }
        out += &format!("\n{}\n", t.body);
        out
    })
}
