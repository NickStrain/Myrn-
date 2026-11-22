//! myrn log: the history of project knowledge, newest first: recorded
//! sessions (episodes) and learned records (bugs, decisions, facts, workflows),
//! filtered by type and state. `myrn log <session>` shows one session in full.

use std::process::ExitCode;

use anyhow::{bail, Result};
use myrn_engine::capture::episode::{clip, Episode};
use myrn_engine::knowledge::{RecordFilter, RecordRow};
use serde::Serialize;

use super::Ctx;

pub fn run(
    ctx: &Ctx,
    session: Option<&str>,
    limit: usize,
    ntype: Option<&str>,
    status: Option<&str>,
    map: bool,
) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    if let Some(id) = session {
        let Some((ep, entries)) = engine.episode_log(id)? else {
            if engine.trace(id).is_ok() {
                bail!("`{id}` is a record: see `myrn show {id}` or `myrn trace {id}`");
            }
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

    let want_sessions = ntype.is_none_or(|t| t == "episode" || t == "session") && status.is_none();
    let want_records = !matches!(ntype, Some("episode" | "session"));
    let eps = if want_sessions {
        engine.episodes(limit)?
    } else {
        Vec::new()
    };
    let filter = RecordFilter {
        ntype: ntype.map(|t| {
            if t == "root-cause" {
                "root_cause".into()
            } else {
                t.to_string()
            }
        }),
        status: status.map(str::to_string),
        map,
    };
    let recs = if want_records {
        engine.records(&filter, limit)?
    } else {
        Vec::new()
    };
    let consent = engine.state()?.capture_consent;

    #[derive(Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum Item<'a> {
        Session(&'a Episode),
        Record(&'a RecordRow),
    }
    let mut items: Vec<(i64, Item)> = eps
        .iter()
        .map(|e| (e.started, Item::Session(e)))
        .chain(recs.iter().map(|r| (r.time, Item::Record(r))))
        .collect();
    items.sort_by_key(|(t, _)| std::cmp::Reverse(*t));
    items.truncate(limit);
    let list: Vec<&Item> = items.iter().map(|(_, i)| i).collect();
    ctx.emit(&list, || {
        if items.is_empty() {
            return if want_sessions && !consent {
                "No sessions recorded. Recording is off until you agree to it in `myrn connect`.\n"
                    .into()
            } else if ntype.is_some() || status.is_some() {
                "Nothing matches.\n".into()
            } else {
                "No sessions or records yet. They appear here after your agent's next session.\n"
                    .into()
            };
        }
        let mut out = String::new();
        for (t, item) in &items {
            match item {
                Item::Session(e) => {
                    out += &format!(
                        "{}  session  {:<14}  {:<12}  {}\n",
                        datetime(*t),
                        clip(&e.session, 14),
                        e.outcome(),
                        e.task
                            .as_deref()
                            .map(|t| clip(t.lines().next().unwrap_or(""), 60))
                            .unwrap_or_else(|| "(no prompt)".into())
                    );
                    out += &format!("{:>27}  {}\n", "", e.summary());
                }
                Item::Record(r) => {
                    let state = if r.status == r.written_status {
                        r.status.clone()
                    } else {
                        format!("{} ({})", r.status, r.written_status)
                    };
                    out += &format!(
                        "{}  {:<7}  {:<14}  {:<12}  {}\n",
                        datetime(*t),
                        short_type(&r.ntype),
                        clip(&r.id, 14),
                        state,
                        clip(&r.title, 60)
                    );
                }
            }
        }
        out += "\nTimes are UTC. `myrn log <session>` shows a session; `myrn trace <id>` where a record came from.\n";
        out
    })
}

fn short_type(t: &str) -> &str {
    match t {
        "root_cause" => "cause",
        "decision" => "decide",
        "workflow" => "workflw",
        "summary" => "summary",
        other => other,
    }
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
pub(crate) fn datetime(t: i64) -> String {
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
