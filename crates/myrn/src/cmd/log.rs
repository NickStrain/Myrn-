//! myrn log: recorded sessions (episodes). Decisions and learned records join
//! this list in Phase 2.

use std::process::ExitCode;

use anyhow::{bail, Result};
use myrn_engine::capture::episode::{clip, Episode};
use serde::Serialize;

use super::Ctx;

pub fn run(ctx: &Ctx, session: Option<&str>, limit: usize) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    if let Some(id) = session {
        let Some((ep, entries)) = engine.episode_log(id)? else {
            bail!("no recorded session matches `{id}`; see `myrn log`");
        };
        #[derive(Serialize)]
        struct Out<'a> {
            episode: &'a Episode,
            outcome: &'a str,
            events: &'a [myrn_store::episodes::Entry],
        }
        let out = Out {
            episode: &ep,
            outcome: ep.outcome(),
            events: &entries,
        };
        return ctx.emit(&out, || render_one(&ep, &entries));
    }
    let eps = engine.episodes(limit)?;
    let consent = engine.state()?.capture_consent;
    ctx.emit(&eps, || {
        if eps.is_empty() {
            return if consent {
                "No sessions recorded yet. They appear here after your agent's next session.\n"
                    .into()
            } else {
                "No sessions recorded. Recording is off until you agree to it in `myrn connect`.\n"
                    .into()
            };
        }
        let mut out = String::new();
        for e in &eps {
            out += &format!(
                "{}  {:<6}  {:<12}  {:<12}  {}\n",
                datetime(e.started),
                e.agent,
                clip(&e.session, 12),
                e.outcome(),
                e.task
                    .as_deref()
                    .map(|t| clip(t.lines().next().unwrap_or(""), 70))
                    .unwrap_or_else(|| "(no prompt)".into())
            );
            out += &format!("{:>18}  {}\n", "", e.summary());
        }
        out += "\nTimes are UTC. `myrn log <session>` shows one session in full.\n";
        out
    })
}

fn render_one(ep: &Episode, entries: &[myrn_store::episodes::Entry]) -> String {
    let mut out = format!("session   {}  ({})\n", ep.session, ep.agent);
    out += &format!("started   {} UTC\n", datetime(ep.started));
    if let Some(t) = ep.ended {
        out += &format!("ended     {} UTC\n", datetime(t));
    }
    if let Some(t) = &ep.task {
        out += &format!("task      {}\n", clip(t.lines().next().unwrap_or(""), 100));
    }
    out += &format!("outcome   {}: {}\n", ep.outcome(), ep.summary());
    let list = |label: &str, items: &mut dyn Iterator<Item = &String>| {
        let v: Vec<&String> = items.collect();
        if v.is_empty() {
            return String::new();
        }
        let mut s = format!("{label:<10}{}\n", v[0]);
        for i in &v[1..] {
            s += &format!("{:<10}{i}\n", "");
        }
        s
    };
    out += &list("edited", &mut ep.files_edited.iter());
    out += &list("symbols", &mut ep.symbols_touched.iter());
    out += &list("read", &mut ep.files_read.iter());
    out += &list("commits", &mut ep.commits.iter());
    out += "\nevents\n";
    for e in entries {
        let what = match e.event.as_str() {
            "prompt" => format!(
                "prompt    {}",
                clip(
                    e.text.as_deref().unwrap_or("").lines().next().unwrap_or(""),
                    90
                )
            ),
            "tool" => {
                let mut s = format!(
                    "{:<9} {}",
                    e.tool.as_deref().unwrap_or("tool"),
                    e.outcome.as_deref().unwrap_or("")
                );
                if let Some(c) = &e.command {
                    s += &format!("  $ {}", clip(c.lines().next().unwrap_or(""), 70));
                }
                if !e.files.is_empty() {
                    s += &format!("  {}", e.files.join(", "));
                }
                if let Some(t) = &e.test {
                    s += &format!("  [test {t}]");
                }
                if let Some(err) = &e.error {
                    s += &format!("\n{:>21}{}", "", clip(err.lines().next().unwrap_or(""), 90));
                }
                s
            }
            "session_start" => format!("start     {}", e.source.as_deref().unwrap_or("")),
            "session_end" => format!("end       {}", e.reason.as_deref().unwrap_or("")),
            other => other.to_string(),
        };
        out += &format!("  {}  {what}\n", &datetime(e.t)[11..]);
    }
    out
}

/// `YYYY-MM-DD HH:MM` in UTC.
fn datetime(t: i64) -> String {
    let z = t.div_euclid(86_400) + 719_468;
    let secs = t.rem_euclid(86_400);
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}",
        secs / 3600,
        (secs % 3600) / 60
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn datetimes() {
        assert_eq!(super::datetime(0), "1970-01-01 00:00");
        assert_eq!(super::datetime(1_794_787_200 + 3_660), "2026-11-16 01:01");
    }
}
