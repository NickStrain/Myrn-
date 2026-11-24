//! Aggregate runs, compare conditions, and decide Gate 1.
//!
//! Runs are paired by (task, run number): the same task, the same attempt
//! index, once without and once with myrn. Token changes are reported as the
//! mean paired difference with a 95% confidence interval.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::Serialize;

use crate::metrics::{ci95, mean, median};
use crate::runner::{Condition, RunRecord};
use crate::task::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Pass,
    /// Tokens lower on average but the interval includes no change.
    Inconclusive,
    Fail,
    NoData,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Verdict::Pass => "PASS",
            Verdict::Inconclusive => "INCONCLUSIVE",
            Verdict::Fail => "FAIL",
            Verdict::NoData => "NO DATA",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Gate {
    pub verdict: Verdict,
    pub pairs: usize,
    pub token_diff_mean: Option<f64>,
    pub token_diff_ci: Option<f64>,
    pub token_change_pct: Option<f64>,
    pub success_without: Option<f64>,
    pub success_with: Option<f64>,
    /// Pairs where `b` used fewer tokens than `a`.
    pub cheaper: usize,
    /// Two-sided exact sign test over the pairs (ties dropped).
    pub sign_p: Option<f64>,
    pub reason: String,
}

/// Two-sided exact sign test: `k` successes out of `n` under p = 0.5.
pub fn sign_test(k: usize, n: usize) -> Option<f64> {
    if n == 0 {
        return None;
    }
    let tail = k.min(n - k);
    // ln C(n, i), summed in log space to stay exact enough for n in the hundreds.
    let ln_fact = |m: usize| (1..=m).map(|x| (x as f64).ln()).sum::<f64>();
    let p: f64 = (0..=tail)
        .map(|i| (ln_fact(n) - ln_fact(i) - ln_fact(n - i) - n as f64 * 2f64.ln()).exp())
        .sum();
    Some((2.0 * p).min(1.0))
}

/// A per-run token count.
type TokenMeasure = dyn Fn(&RunRecord) -> f64;

/// One value per run, or `None` when the metric does not apply.
type Metric = Box<dyn Fn(&RunRecord) -> Option<f64>>;

fn valid(r: &RunRecord) -> bool {
    r.error.is_none()
}

/// Tokens for the task itself. myrn sessions end with a request to record what
/// was learned; that phase is reported separately (Recording), so the gate
/// measures what injection does to the task.
fn tokens(r: &RunRecord) -> f64 {
    r.session.task_tokens() as f64
}

/// Pairs of runs of the same task and run number under conditions `a` and `b`.
fn pairs_of(runs: &[RunRecord], a: Condition, b: Condition) -> Vec<(&RunRecord, &RunRecord)> {
    let mut by: BTreeMap<(&str, usize), [Option<&RunRecord>; 2]> = BTreeMap::new();
    for r in runs.iter().filter(|r| valid(r)) {
        let i = if r.condition == a {
            0
        } else if r.condition == b {
            1
        } else {
            continue;
        };
        by.entry((r.task.as_str(), r.run)).or_default()[i] = Some(r);
    }
    by.into_values()
        .filter_map(|[x, y]| Some((x?, y?)))
        .collect()
}

fn rate(xs: &[&RunRecord]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().filter(|r| r.success).count() as f64 / xs.len() as f64)
    }
}

/// Paired comparison of `b` against `a` on task tokens.
pub fn compare(runs: &[RunRecord], a: Condition, b: Condition) -> Gate {
    compare_by(runs, a, b, &tokens)
}

/// Total tokens, the recording phase included: what a session costs.
fn total_tokens(r: &RunRecord) -> f64 {
    r.session.tokens.total() as f64
}

/// Paired comparison of `b` against `a` on any per-run token measure.
pub fn compare_by(runs: &[RunRecord], a: Condition, b: Condition, tokens: &TokenMeasure) -> Gate {
    let p = pairs_of(runs, a, b);
    let diffs: Vec<f64> = p.iter().map(|(x, y)| tokens(y) - tokens(x)).collect();
    let base: Vec<f64> = p.iter().map(|(x, _)| tokens(x)).collect();
    let cheaper = diffs.iter().filter(|d| **d < 0.0).count();
    let untied = diffs.iter().filter(|d| **d != 0.0).count();
    let xs: Vec<&RunRecord> = p.iter().map(|(x, _)| *x).collect();
    let ys: Vec<&RunRecord> = p.iter().map(|(_, y)| *y).collect();
    let (sw, sm) = (rate(&xs), rate(&ys));
    let dm = mean(&diffs);
    let ci = ci95(&diffs);
    let pct = match (dm, mean(&base)) {
        (Some(d), Some(b)) if b > 0.0 => Some(100.0 * d / b),
        _ => None,
    };
    let mut g = Gate {
        verdict: Verdict::NoData,
        pairs: p.len(),
        token_diff_mean: dm,
        token_diff_ci: ci,
        token_change_pct: pct,
        success_without: sw,
        success_with: sm,
        cheaper,
        sign_p: sign_test(cheaper, untied),
        reason: String::new(),
    };
    let (Some(dm), Some(sw), Some(sm)) = (dm, sw, sm) else {
        g.reason = format!("no complete {}/{} pairs", a.as_str(), b.as_str());
        return g;
    };
    let upper = ci.map(|c| dm + c).unwrap_or(f64::INFINITY);
    (g.verdict, g.reason) = if sm < sw {
        (
            Verdict::Fail,
            format!(
                "task success dropped from {:.0}% to {:.0}%",
                sw * 100.0,
                sm * 100.0
            ),
        )
    } else if dm >= 0.0 {
        (Verdict::Fail, "task tokens did not fall".into())
    } else if upper >= 0.0 {
        (
            Verdict::Inconclusive,
            "tokens fell on average, but the 95% interval includes no change; run more pairs"
                .into(),
        )
    } else if g.sign_p.is_none_or(|p| p >= 0.05) {
        (
            Verdict::Inconclusive,
            "the mean fell (95% interval below zero), but too few pairs were cheaper for the \
             sign test (p >= 0.05); run more pairs"
                .into(),
        )
    } else {
        (
            Verdict::Pass,
            "fewer tokens (95% interval below zero, sign test p < 0.05) and no drop in task success"
                .into(),
        )
    };
    g
}

/// Gate 1: myrn with injection on against no myrn.
pub fn gate(runs: &[RunRecord]) -> Gate {
    compare(runs, Condition::Without, Condition::With)
}

fn fmt_mean(xs: &[f64], digits: usize) -> String {
    match (mean(xs), ci95(xs)) {
        (Some(m), Some(c)) => format!("{m:.digits$} ± {c:.digits$}"),
        (Some(m), None) => format!("{m:.digits$}"),
        _ => "–".into(),
    }
}

fn fmt_k(xs: &[f64]) -> String {
    let k: Vec<f64> = xs.iter().map(|x| x / 1000.0).collect();
    match (mean(&k), ci95(&k)) {
        (Some(m), Some(c)) => format!("{m:.1}k ± {c:.1}k"),
        (Some(m), None) => format!("{m:.1}k"),
        _ => "–".into(),
    }
}

fn pct(x: Option<f64>) -> String {
    x.map(|v| format!("{:.0}%", v * 100.0))
        .unwrap_or_else(|| "–".into())
}

fn signed_pct(x: Option<f64>) -> String {
    x.map(|v| format!("{v:+.0}%")).unwrap_or_else(|| "–".into())
}

fn diff_cell(g: &Gate) -> String {
    match (g.token_diff_mean, g.token_diff_ci) {
        (Some(m), Some(c)) => format!(
            "{:+.1}k ± {:.1}k ({})",
            m / 1000.0,
            c / 1000.0,
            signed_pct(g.token_change_pct)
        ),
        (Some(m), None) => format!("{:+.1}k ({})", m / 1000.0, signed_pct(g.token_change_pct)),
        _ => "–".into(),
    }
}

const ORDER: [Condition; 5] = [
    Condition::Without,
    Condition::Off,
    Condition::Fixed,
    Condition::With,
    Condition::Hint,
];

/// The Markdown report.
pub fn render(runs: &[RunRecord], title: &str) -> String {
    let g = gate(runs);
    let conds: Vec<Condition> = ORDER
        .into_iter()
        .filter(|c| runs.iter().any(|r| r.condition == *c))
        .collect();
    let mut out = String::new();
    let _ = writeln!(out, "# {title}\n");
    let models: std::collections::BTreeSet<String> = runs
        .iter()
        .map(|r| r.model.clone().unwrap_or_else(|| "default".into()))
        .collect();
    let tasks: std::collections::BTreeSet<&str> = runs.iter().map(|r| r.task.as_str()).collect();
    let max_run = runs.iter().map(|r| r.run).max().unwrap_or(0);
    let failed = runs.iter().filter(|r| !valid(r)).count();
    let _ = writeln!(
        out,
        "{} tasks, up to {} runs per condition, conditions: {}, model: {}. {} runs total, {} excluded because the run itself failed.\n",
        tasks.len(),
        max_run,
        conds.iter().map(|c| c.label()).collect::<Vec<_>>().join(" / "),
        models.into_iter().collect::<Vec<_>>().join(", "),
        runs.len(),
        failed
    );
    let _ = writeln!(
        out,
        "## Gate 1 (inject on vs without myrn): {}\n",
        g.verdict.label()
    );
    let _ = writeln!(out, "{}.\n", g.reason);
    let learned = compare(runs, Condition::Fixed, Condition::With);
    if learned.pairs > 0 {
        let _ = writeln!(
            out,
            "## Step 20 (learned vs fixed ranking): {}\n\n{}.\n",
            learned.verdict.label(),
            learned.reason
        );
    }

    // Every paired comparison present, on task tokens and on total tokens.
    let _ = writeln!(out, "## Paired comparisons\n");
    let _ = writeln!(
        out,
        "| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |\n|---|---|---|---|---|---|---|---|"
    );
    let measures: [(&str, &TokenMeasure); 2] = [
        ("task", &tokens),
        ("total, recording included", &total_tokens),
    ];
    for (a, b) in [
        (Condition::Without, Condition::With),
        (Condition::Without, Condition::Off),
        (Condition::Off, Condition::With),
        (Condition::Without, Condition::Fixed),
        (Condition::Fixed, Condition::With),
        (Condition::Without, Condition::Hint),
        (Condition::With, Condition::Hint),
    ] {
        for (name, f) in measures {
            let c = compare_by(runs, a, b, f);
            if c.pairs == 0 {
                continue;
            }
            let _ = writeln!(
                out,
                "| {} → {} | {name} | {} | {} | {} of {} | {} | {} → {} | {} |",
                a.label(),
                b.label(),
                c.pairs,
                diff_cell(&c),
                c.cheaper,
                c.pairs,
                c.sign_p.map(|p| format!("{p:.3}")).unwrap_or("–".into()),
                pct(c.success_without),
                pct(c.success_with),
                c.verdict.label()
            );
        }
    }
    let _ = writeln!(
        out,
        "\nTask tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.\n"
    );

    let by_cond = |c: Condition| -> Vec<&RunRecord> {
        runs.iter()
            .filter(|r| valid(r) && r.condition == c)
            .collect()
    };
    let groups: Vec<Vec<&RunRecord>> = conds.iter().map(|c| by_cond(*c)).collect();
    let col = |rs: &[&RunRecord], f: &dyn Fn(&RunRecord) -> Option<f64>| -> Vec<f64> {
        rs.iter().filter_map(|r| f(r)).collect()
    };
    let _ = writeln!(out, "## All tasks\n");
    let _ = writeln!(
        out,
        "| Metric | {} |\n|---|{}",
        conds
            .iter()
            .map(|c| c.label())
            .collect::<Vec<_>>()
            .join(" | "),
        "---|".repeat(conds.len())
    );
    let rows: Vec<(&str, Metric, usize)> = vec![
        ("Task tokens", Box::new(|r| Some(tokens(r))), 0),
        (
            "Total tokens, recording included",
            Box::new(|r| Some(r.session.tokens.total() as f64)),
            0,
        ),
        (
            "Output tokens",
            Box::new(|r| Some(r.session.tokens.output as f64)),
            0,
        ),
        (
            "Cost (USD), recording included",
            Box::new(|r| r.session.cost_usd),
            4,
        ),
        (
            "Reads and searches before first correct edit",
            Box::new(|r| r.session.exploration_before_edit.map(|x| x as f64)),
            1,
        ),
        (
            "Reads and searches, whole task",
            Box::new(|r| Some(r.session.exploration_total as f64)),
            1,
        ),
        (
            "Time to passing tests (s)",
            Box::new(|r| {
                (r.success && r.kind != Kind::Question).then(|| r.wall_ms as f64 / 1000.0)
            }),
            1,
        ),
        (
            "Turns",
            Box::new(|r| r.session.num_turns.map(|x| x as f64)),
            1,
        ),
    ];
    for (name, f, digits) in &rows {
        let cells: Vec<String> = groups
            .iter()
            .map(|g| {
                let v = col(g, f.as_ref());
                if *digits == 0 {
                    fmt_k(&v)
                } else {
                    fmt_mean(&v, *digits)
                }
            })
            .collect();
        let _ = writeln!(out, "| {name} | {} |", cells.join(" | "));
    }
    let cells: Vec<String> = groups.iter().map(|g| pct(rate(g))).collect();
    let _ = writeln!(out, "| Task success | {} |", cells.join(" | "));
    let cells: Vec<String> = groups
        .iter()
        .map(|g| g.iter().filter(|r| r.timed_out).count().to_string())
        .collect();
    let _ = writeln!(out, "| Timed out | {} |\n", cells.join(" | "));

    // myrn conditions: injection, tools, hooks, recording.
    let myrn: Vec<(Condition, &Vec<&RunRecord>)> = conds
        .iter()
        .zip(&groups)
        .filter(|(c, _)| c.uses_myrn())
        .map(|(c, g)| (*c, g))
        .collect();
    if !myrn.is_empty() {
        let _ = writeln!(out, "## myrn sessions\n");
        let _ = writeln!(
            out,
            "| | {} |\n|---|{}",
            myrn.iter()
                .map(|(c, _)| c.label())
                .collect::<Vec<_>>()
                .join(" | "),
            "---|".repeat(myrn.len())
        );
        let per = |g: &[&RunRecord], f: &dyn Fn(&RunRecord) -> f64| -> String {
            if g.is_empty() {
                "–".into()
            } else {
                format!(
                    "{:.1}",
                    g.iter().map(|r| f(r)).sum::<f64>() / g.len() as f64
                )
            }
        };
        let line = |name: &str, f: &dyn Fn(&[&RunRecord]) -> String| -> String {
            format!(
                "| {name} | {} |",
                myrn.iter()
                    .map(|(_, g)| f(g))
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        };
        let mut lines = vec![
            line("Items injected per run", &|g| {
                per(g, &|r| r.injected.len() as f64)
            }),
            line("Injection use (items whose file the agent opened)", &|g| {
                let inj: usize = g.iter().map(|r| r.injected.len()).sum();
                let used: usize = g.iter().map(|r| r.injected_used).sum();
                if inj == 0 {
                    "–".into()
                } else {
                    format!("{:.0}%", 100.0 * used as f64 / inj as f64)
                }
            }),
            line("Learned records injected per run", &|g| {
                per(g, &|r| r.injected_records as f64)
            }),
            line(
                "Stale injections (records shown for code that changed since)",
                &|g| {
                    let inj: usize = g.iter().map(|r| r.injected_records).sum();
                    let stale: usize = g.iter().map(|r| r.injected_stale).sum();
                    if inj == 0 {
                        "– (no records injected)".into()
                    } else {
                        format!(
                            "{:.0}% ({stale} of {inj}, all labelled possibly outdated)",
                            100.0 * stale as f64 / inj as f64
                        )
                    }
                },
            ),
            line("myrn tool calls during the task", &|g| {
                per(g, &|r| r.session.myrn_tool_calls as f64)
            }),
            line("Sessions asked to record", &|g| {
                format!(
                    "{} of {}",
                    g.iter().filter(|r| r.session.asked_to_record).count(),
                    g.len()
                )
            }),
            line("Recording tokens per asked session", &|g| {
                let v: Vec<f64> = g
                    .iter()
                    .filter(|r| r.session.asked_to_record)
                    .map(|r| r.session.record_tokens as f64)
                    .collect();
                fmt_k(&v)
            }),
            line("Recording share of the session's tokens", &|g| {
                let rec: u64 = g.iter().map(|r| r.session.record_tokens).sum();
                let all: u64 = g.iter().map(|r| r.session.tokens.total()).sum();
                if all == 0 {
                    "–".into()
                } else {
                    format!("{:.0}%", 100.0 * rec as f64 / all as f64)
                }
            }),
            line("record calls per asked session (rejected)", &|g| {
                let asked: Vec<&&RunRecord> =
                    g.iter().filter(|r| r.session.asked_to_record).collect();
                if asked.is_empty() {
                    return "–".into();
                }
                let calls: usize = asked.iter().map(|r| r.session.record_calls).sum();
                let rej: usize = asked.iter().map(|r| r.session.record_rejected).sum();
                format!(
                    "{:.1} ({rej} of {calls} rejected)",
                    calls as f64 / asked.len() as f64
                )
            }),
            line("Records stored per run", &|g| per(g, &|r| r.records as f64)),
        ];
        let mut hooks: Vec<u64> = myrn
            .iter()
            .flat_map(|(_, g)| g.iter().flat_map(|r| r.hook_ms.iter().copied()))
            .collect();
        let hook_max = hooks.iter().copied().max();
        lines.push(format!(
            "| Prompt hook latency (all myrn runs) | median {} ms, max {} ms |{}",
            median(&mut hooks)
                .map(|x| x.to_string())
                .unwrap_or("–".into()),
            hook_max.map(|x| x.to_string()).unwrap_or("–".into()),
            " |".repeat(myrn.len() - 1)
        ));
        for l in lines {
            let _ = writeln!(out, "{l}");
        }
        out.push('\n');
    }

    // Per task.
    let _ = writeln!(out, "## Per task\n");
    let _ =
        writeln!(
        out,
        "| Task | Kind | Success {} | {} | Change (inject on vs without) |\n|---|---|---|{}---|",
        conds.iter().map(|c| c.as_str()).collect::<Vec<_>>().join(" / "),
        conds.iter().map(|c| format!("Task tokens {}", c.as_str())).collect::<Vec<_>>().join(" | "),
        "---|".repeat(conds.len())
    );
    let mut by_task: BTreeMap<&str, Vec<&RunRecord>> = BTreeMap::new();
    for r in runs.iter().filter(|r| valid(r)) {
        by_task.entry(r.task.as_str()).or_default().push(r);
    }
    for (task, rs) in &by_task {
        let of = |c: Condition| -> Vec<&RunRecord> {
            rs.iter().copied().filter(|r| r.condition == c).collect()
        };
        let toks = |c: Condition| -> Vec<f64> { of(c).iter().map(|r| tokens(r)).collect() };
        let change = match (
            mean(&toks(Condition::Without)),
            mean(&toks(Condition::With)),
        ) {
            (Some(x), Some(y)) if x > 0.0 => Some(100.0 * (y - x) / x),
            _ => None,
        };
        let kind = rs.first().map(|r| r.kind.as_str()).unwrap_or("");
        let _ = writeln!(
            out,
            "| `{task}` | {kind} | {} | {} | {} |",
            conds
                .iter()
                .map(|c| pct(rate(&of(*c))))
                .collect::<Vec<_>>()
                .join(" / "),
            conds
                .iter()
                .map(|c| fmt_k(&toks(*c)))
                .collect::<Vec<_>>()
                .join(" | "),
            signed_pct(change)
        );
    }
    if failed > 0 {
        let _ = writeln!(out, "\n## Runs that failed to run\n");
        for r in runs.iter().filter(|r| !valid(r)) {
            let _ = writeln!(
                out,
                "- `{}` {} #{}: {}",
                r.task,
                r.condition.as_str(),
                r.run,
                r.error
                    .as_deref()
                    .unwrap_or("")
                    .lines()
                    .next()
                    .unwrap_or("")
            );
        }
    }
    let _ = writeln!(
        out,
        "\n## Method\n\nEach run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change."
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{Session, Tokens};

    fn rec(task: &str, cond: Condition, run: usize, total: u64, success: bool) -> RunRecord {
        RunRecord {
            task: task.into(),
            repo: "r".into(),
            kind: Kind::Bugfix,
            condition: cond,
            run,
            model: None,
            started: 0,
            success,
            timed_out: false,
            wall_ms: 1000,
            session: Session {
                tokens: Tokens {
                    input: total,
                    ..Default::default()
                },
                ..Default::default()
            },
            injected: vec![],
            injected_used: 0,
            injected_records: 0,
            injected_stale: 0,
            hook_ms: vec![15],
            records: 0,
            check_output: String::new(),
            error: None,
        }
    }

    #[test]
    fn sign_tests() {
        assert!((sign_test(10, 10).unwrap() - 0.001953).abs() < 1e-5);
        assert!((sign_test(9, 10).unwrap() - 0.02148).abs() < 1e-4);
        assert_eq!(sign_test(5, 10), Some(1.0));
        assert_eq!(sign_test(0, 0), None);
    }

    #[test]
    fn verdicts() {
        use Condition::*;
        // Clear, consistent saving (6 pairs: enough for the sign test).
        let runs: Vec<RunRecord> = (1..=6)
            .flat_map(|i| {
                [
                    rec("t", Without, i, 100_000 + i as u64 * 1000, true),
                    rec("t", With, i, 70_000 + i as u64 * 1500, true),
                ]
            })
            .collect();
        let g = gate(&runs);
        assert_eq!(g.verdict, Verdict::Pass, "{}", g.reason);
        assert_eq!(g.pairs, 6);
        assert!(g.token_change_pct.unwrap() < -25.0);
        // The same saving on 4 pairs: the sign test cannot reach p < 0.05.
        assert_eq!(gate(&runs[..8]).verdict, Verdict::Inconclusive);

        // Lower success fails the gate even with fewer tokens.
        let mut worse = runs.clone();
        worse[1].success = false;
        assert_eq!(gate(&worse).verdict, Verdict::Fail);

        // Noisy: mean lower, interval crosses zero.
        let noisy = vec![
            rec("t", Without, 1, 100_000, true),
            rec("t", With, 1, 50_000, true),
            rec("t", Without, 2, 100_000, true),
            rec("t", With, 2, 140_000, true),
        ];
        assert_eq!(gate(&noisy).verdict, Verdict::Inconclusive);

        // Errored runs are excluded and unpaired runs ignored.
        let mut broken = runs.clone();
        broken[0].error = Some("boom".into());
        assert_eq!(gate(&broken).pairs, 5);
        assert_eq!(gate(&[]).verdict, Verdict::NoData);

        let md = render(&runs, "Gate 1");
        assert!(
            md.contains("## Gate 1 (inject on vs without myrn): PASS"),
            "{md}"
        );
        assert!(md.contains("| `t` | bugfix | 100% / 100% |"), "{md}");

        // A third condition adds its comparisons and columns.
        let mut three = runs.clone();
        three.extend((1..=4).map(|i| rec("t", Off, i, 90_000, true)));
        let md = render(&three, "Gate 1");
        assert!(
            md.contains("| Without myrn → myrn, inject off | task | 4 |"),
            "{md}"
        );
        assert!(
            md.contains("| myrn, inject off → myrn, inject on | task | 4 |"),
            "{md}"
        );
        assert!(md.contains("| `t` | bugfix | 100% / 100% / 100% |"), "{md}");
    }
}
