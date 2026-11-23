//! `myrn-bench retrieval`: what myrn would inject for each task's prompt, and
//! whether it points at the right code, without running an agent (free).
//!
//! For bug fixes and features the right code is the task's `touches` files;
//! for questions it is the `answer_contains` terms appearing in the pack.

use std::fmt::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Result};

use crate::runner;
use crate::task::{Repo, Task};

pub struct Row {
    pub id: String,
    pub kind: String,
    /// Rank (1-based) of the first pack item inside a `touches` file.
    pub hit_rank: Option<usize>,
    /// Questions: answer terms found in the pack, of all.
    pub terms: Option<(usize, usize)>,
    pub items: usize,
    pub tokens: usize,
    pub top: String,
    pub text: String,
}

/// The file an item id points at: `sym:path#name`, `file:path`, `test:path#name`.
fn item_path(id: &str) -> &str {
    let rest = id.split_once(':').map_or(id, |(_, r)| r);
    rest.split('#').next().unwrap_or(rest)
}

pub fn one(
    task: &Task,
    repo: &Repo,
    checkout: &Path,
    myrn: &Path,
    map: Option<&Path>,
    scratch: &Path,
) -> Result<Row> {
    let work = scratch.join("work").join(&repo.name);
    let home = scratch.join("home");
    if home.exists() {
        std::fs::remove_dir_all(&home)?;
    }
    runner::prepare(checkout, repo, task, &work)?;
    runner::setup_myrn(myrn, &work, &home, true, false, map, None)?;
    let o = Command::new(myrn)
        .args(["--json", "context", &task.prompt])
        .current_dir(&work)
        .env("MYRN_HOME", &home)
        .env("MYRN_NO_AGENT_CLI", "1")
        .stdin(Stdio::null())
        .output()?;
    if !o.status.success() {
        bail!(
            "{}: myrn context failed: {}",
            task.id,
            String::from_utf8_lossy(&o.stderr)
        );
    }
    let v: serde_json::Value = serde_json::from_slice(&o.stdout)?;
    let items: Vec<String> = v["items"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let raw = v["text"].as_str().unwrap_or("").to_string();
    let text = raw.to_lowercase();
    let hit_rank = items
        .iter()
        .position(|id| {
            let p = item_path(id);
            task.touches.iter().any(|t| t == p)
        })
        .map(|i| i + 1);
    let terms = (!task.answer_contains.is_empty()).then(|| {
        let found = task
            .answer_contains
            .iter()
            .filter(|t| text.contains(&t.to_lowercase()))
            .count();
        (found, task.answer_contains.len())
    });
    Ok(Row {
        id: task.id.clone(),
        kind: task.kind.as_str().to_string(),
        hit_rank,
        terms,
        items: items.len(),
        tokens: v["tokens"].as_u64().unwrap_or(0) as usize,
        top: items.first().cloned().unwrap_or_default(),
        text: raw,
    })
}

pub fn render(rows: &[Row]) -> String {
    let mut s = String::from(
        "| Task | Kind | Right code | Items | Pack tokens | Top item |\n|---|---|---|---|---|---|\n",
    );
    for r in rows {
        let right = match (r.hit_rank, r.terms) {
            (_, Some((f, n))) => format!("{f}/{n} answer terms"),
            (Some(1), _) => "top 1".to_string(),
            (Some(k), _) => format!("rank {k}"),
            (None, _) => "**missed**".to_string(),
        };
        let _ = writeln!(
            s,
            "| `{}` | {} | {} | {} | {} | `{}` |",
            r.id, r.kind, right, r.items, r.tokens, r.top
        );
    }
    let code: Vec<&Row> = rows.iter().filter(|r| r.terms.is_none()).collect();
    let top1 = code.iter().filter(|r| r.hit_rank == Some(1)).count();
    let any = code.iter().filter(|r| r.hit_rank.is_some()).count();
    let (tf, tn) = rows
        .iter()
        .filter_map(|r| r.terms)
        .fold((0, 0), |(a, b), (f, n)| (a + f, b + n));
    let tokens: usize = rows.iter().map(|r| r.tokens).sum();
    let _ = writeln!(
        s,
        "\nCode tasks: right file top 1 in {top1}/{n}, in the pack in {any}/{n}. \
         Questions: {tf}/{tn} answer terms in the pack. Mean pack: {} tokens.",
        tokens / rows.len().max(1),
        n = code.len()
    );
    s
}
