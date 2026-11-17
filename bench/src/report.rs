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
    pub reason: String,
}

/// One value per run, or `None` when the metric does not apply.
type Metric = Box<dyn Fn(&RunRecord) -> Option<f64>>;

fn valid(r: &RunRecord) -> bool {
    r.error.is_none()
}

fn tokens(r: &RunRecord) -> f64 {
    r.session.tokens.total() as f64
}

/// Pairs of (without, with) for the same task and run number.
fn pairs(runs: &[RunRecord]) -> Vec<(&RunRecord, &RunRecord)> {
    let mut by: BTreeMap<(&str, usize), [Option<&RunRecord>; 2]> = BTreeMap::new();
    for r in runs.iter().filter(|r| valid(r)) {
        let slot = by.entry((r.task.as_str(), r.run)).or_default();
        let i = match r.condition {
            Condition::Without => 0,
            Condition::With => 1,
        };
        slot[i] = Some(r);
    }
    by.into_values()
        .filter_map(|[a, b]| Some((a?, b?)))
        .collect()
}

fn rate(xs: &[&RunRecord]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().filter(|r| r.success).count() as f64 / xs.len() as f64)
    }
}

pub fn gate(runs: &[RunRecord]) -> Gate {
    let p = pairs(runs);
    let diffs: Vec<f64> = p.iter().map(|(a, b)| tokens(b) - tokens(a)).collect();
    let base: Vec<f64> = p.iter().map(|(a, _)| tokens(a)).collect();
    let without: Vec<&RunRecord> = p.iter().map(|(a, _)| *a).collect();
    let with: Vec<&RunRecord> = p.iter().map(|(_, b)| *b).collect();
    let (sw, sm) = (rate(&without), rate(&with));
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
        reason: String::new(),
    };
    let (Some(dm), Some(sw), Some(sm)) = (dm, sw, sm) else {
        g.reason = "no complete with/without pairs".into();
        return g;
    };
    let success_ok = sm >= sw;
    let upper = ci.map(|c| dm + c).unwrap_or(f64::INFINITY);
    (g.verdict, g.reason) = if !success_ok {
        (
            Verdict::Fail,
            format!(
                "task success dropped from {:.0}% to {:.0}%",
                sw * 100.0,
                sm * 100.0
            ),
        )
    } else if dm >= 0.0 {
        (Verdict::Fail, "total tokens did not fall".into())
    } else if upper >= 0.0 {
        (
            Verdict::Inconclusive,
            "tokens fell on average, but the 95% interval includes no change; run more pairs"
                .into(),
        )
    } else {
        (
            Verdict::Pass,
            "fewer total tokens (95% interval below zero) and no drop in task success".into(),
        )
    };
    g
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

/// The Markdown report.
pub fn render(runs: &[RunRecord], title: &str) -> String {
    let g = gate(runs);
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
        "{} tasks, up to {} runs per condition, model: {}. {} runs total, {} excluded because the run itself failed.\n",
        tasks.len(),
        max_run,
        models.into_iter().collect::<Vec<_>>().join(", "),
        runs.len(),
        failed
    );

    let _ = writeln!(out, "## Gate 1: {}\n", g.verdict.label());
    let _ = writeln!(out, "{}.\n", g.reason);
    let _ = writeln!(out, "| | Value |\n|---|---|");
    let _ = writeln!(out, "| Paired runs | {} |", g.pairs);
    let _ = writeln!(
        out,
        "| Token change per task (with − without) | {} ({}) |",
        match (g.token_diff_mean, g.token_diff_ci) {
            (Some(m), Some(c)) => format!("{:+.1}k ± {:.1}k", m / 1000.0, c / 1000.0),
            (Some(m), None) => format!("{:+.1}k", m / 1000.0),
            _ => "–".into(),
        },
        signed_pct(g.token_change_pct)
    );
    let _ = writeln!(
        out,
        "| Task success without → with | {} → {} |\n",
        pct(g.success_without),
        pct(g.success_with)
    );
    let _ = writeln!(
        out,
        "Automatic injection becomes the default only on PASS. Values are means ± 95% confidence intervals.\n"
    );

    // Aggregate by condition.
    let by_cond = |c: Condition| -> Vec<&RunRecord> {
        runs.iter()
            .filter(|r| valid(r) && r.condition == c)
            .collect()
    };
    let (wo, wi) = (by_cond(Condition::Without), by_cond(Condition::With));
    let col = |rs: &[&RunRecord], f: &dyn Fn(&RunRecord) -> Option<f64>| -> Vec<f64> {
        rs.iter().filter_map(|r| f(r)).collect()
    };
    let _ = writeln!(out, "## All tasks\n");
    let _ = writeln!(out, "| Metric | Without myrn | With myrn |\n|---|---|---|");
    let rows: Vec<(&str, Metric, bool)> = vec![
        ("Total tokens", Box::new(|r| Some(tokens(r))), true),
        (
            "Output tokens",
            Box::new(|r| Some(r.session.tokens.output as f64)),
            true,
        ),
        ("Cost (USD)", Box::new(|r| r.session.cost_usd), false),
        (
            "Reads and searches before first correct edit",
            Box::new(|r| r.session.exploration_before_edit.map(|x| x as f64)),
            false,
        ),
        (
            "Reads and searches, whole session",
            Box::new(|r| Some(r.session.exploration_total as f64)),
            false,
        ),
        (
            "Time to passing tests (s)",
            Box::new(|r| {
                (r.success && r.kind != Kind::Question).then(|| r.wall_ms as f64 / 1000.0)
            }),
            false,
        ),
        (
            "Turns",
            Box::new(|r| r.session.num_turns.map(|x| x as f64)),
            false,
        ),
    ];
    for (name, f, k) in &rows {
        let (a, b) = (col(&wo, f.as_ref()), col(&wi, f.as_ref()));
        let (a, b) = if *k {
            (fmt_k(&a), fmt_k(&b))
        } else if name.starts_with("Cost") {
            (fmt_mean(&a, 4), fmt_mean(&b, 4))
        } else {
            (fmt_mean(&a, 1), fmt_mean(&b, 1))
        };
        let _ = writeln!(out, "| {name} | {a} | {b} |");
    }
    let _ = writeln!(
        out,
        "| Task success | {} | {} |",
        pct(rate(&wo)),
        pct(rate(&wi))
    );
    let timeouts = |rs: &[&RunRecord]| rs.iter().filter(|r| r.timed_out).count();
    let _ = writeln!(
        out,
        "| Timed out | {} | {} |\n",
        timeouts(&wo),
        timeouts(&wi)
    );

    // Injection and hooks.
    let injected: usize = wi.iter().map(|r| r.injected.len()).sum();
    let used: usize = wi.iter().map(|r| r.injected_used).sum();
    let myrn_calls: usize = wi.iter().map(|r| r.session.myrn_tool_calls).sum();
    let mut hooks: Vec<u64> = wi.iter().flat_map(|r| r.hook_ms.iter().copied()).collect();
    let hook_max = hooks.iter().copied().max();
    let _ = writeln!(out, "## Injection and hooks\n");
    let _ = writeln!(out, "| | Value |\n|---|---|");
    let _ = writeln!(
        out,
        "| Items injected per run | {} |",
        if wi.is_empty() {
            "–".into()
        } else {
            format!("{:.1}", injected as f64 / wi.len() as f64)
        }
    );
    let _ = writeln!(
        out,
        "| Injection use (items whose file the agent then opened) | {} |",
        if injected == 0 {
            "–".into()
        } else {
            format!("{:.0}%", 100.0 * used as f64 / injected as f64)
        }
    );
    let _ = writeln!(
        out,
        "| myrn tool calls per run | {} |",
        if wi.is_empty() {
            "–".into()
        } else {
            format!("{:.1}", myrn_calls as f64 / wi.len() as f64)
        }
    );
    let _ = writeln!(
        out,
        "| Prompt hook latency | median {} ms, max {} ms |\n",
        median(&mut hooks)
            .map(|x| x.to_string())
            .unwrap_or("–".into()),
        hook_max.map(|x| x.to_string()).unwrap_or("–".into())
    );

    // Per task.
    let _ = writeln!(out, "## Per task\n");
    let _ = writeln!(
        out,
        "| Task | Kind | Success without / with | Tokens without | Tokens with | Change | Exploration without / with |\n|---|---|---|---|---|---|---|"
    );
    let mut by_task: BTreeMap<&str, Vec<&RunRecord>> = BTreeMap::new();
    for r in runs.iter().filter(|r| valid(r)) {
        by_task.entry(r.task.as_str()).or_default().push(r);
    }
    for (task, rs) in &by_task {
        let a: Vec<&RunRecord> = rs
            .iter()
            .copied()
            .filter(|r| r.condition == Condition::Without)
            .collect();
        let b: Vec<&RunRecord> = rs
            .iter()
            .copied()
            .filter(|r| r.condition == Condition::With)
            .collect();
        let ta: Vec<f64> = a.iter().map(|r| tokens(r)).collect();
        let tb: Vec<f64> = b.iter().map(|r| tokens(r)).collect();
        let change = match (mean(&ta), mean(&tb)) {
            (Some(x), Some(y)) if x > 0.0 => Some(100.0 * (y - x) / x),
            _ => None,
        };
        let ex = |rs: &[&RunRecord]| -> String {
            let v: Vec<f64> = rs
                .iter()
                .filter_map(|r| {
                    r.session
                        .exploration_before_edit
                        .or(Some(r.session.exploration_total))
                })
                .map(|x| x as f64)
                .collect();
            mean(&v).map(|m| format!("{m:.1}")).unwrap_or("–".into())
        };
        let kind = rs.first().map(|r| r.kind.as_str()).unwrap_or("");
        let _ = writeln!(
            out,
            "| `{task}` | {kind} | {} / {} | {} | {} | {} | {} / {} |",
            pct(rate(&a)),
            pct(rate(&b)),
            fmt_k(&ta),
            fmt_k(&tb),
            signed_pct(change),
            ex(&a),
            ex(&b)
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
        "\n## Method\n\nEach run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user settings and MCP servers off (`--setting-sources project,local --strict-mcp-config`). *With myrn*: `myrn init`, `myrn connect` hooks, `[context] inject = true`, and myrn's MCP server. *Without*: neither. Total tokens are input + cache creation + cache reads + output from the session result, so injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change."
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
            hook_ms: vec![15],
            check_output: String::new(),
            error: None,
        }
    }

    #[test]
    fn verdicts() {
        use Condition::*;
        // Clear, consistent saving.
        let runs: Vec<RunRecord> = (1..=4)
            .flat_map(|i| {
                [
                    rec("t", Without, i, 100_000 + i as u64 * 1000, true),
                    rec("t", With, i, 70_000 + i as u64 * 1500, true),
                ]
            })
            .collect();
        let g = gate(&runs);
        assert_eq!(g.verdict, Verdict::Pass, "{}", g.reason);
        assert_eq!(g.pairs, 4);
        assert!(g.token_change_pct.unwrap() < -25.0);

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
        assert_eq!(gate(&broken).pairs, 3);
        assert_eq!(gate(&[]).verdict, Verdict::NoData);

        let md = render(&runs, "Gate 1");
        assert!(md.contains("## Gate 1: PASS"), "{md}");
        assert!(md.contains("| `t` | bugfix | 100% / 100% |"), "{md}");
    }
}
