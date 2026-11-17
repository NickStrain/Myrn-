//! Expand seeds one hop along the graph and rank the result.

use std::collections::HashMap;

use anyhow::Result;
use myrn_store::graph::{self, Dir};
use myrn_store::Db;

use super::seed::Seed;
use super::walks;

/// Below this best-seed score nothing is injected: silence beats filler.
pub const MIN_SCORE: f64 = 3.0;

#[derive(Debug, Clone)]
pub struct Ranked {
    pub id: String,
    pub score: f64,
    pub why: &'static str,
}

pub fn rank(db: &Db, seeds: Vec<Seed>) -> Result<Vec<Ranked>> {
    let c = &db.conn;
    let mut acc: HashMap<String, Ranked> = HashMap::new();
    let mut add = |id: &str, score: f64, why: &'static str| {
        let e = acc.entry(id.to_string()).or_insert(Ranked {
            id: id.to_string(),
            score: 0.0,
            why,
        });
        if score > e.score {
            e.why = why;
        }
        // Several weak signals add up, but never beyond the strongest plus half.
        e.score = e.score.max(score) + 0.25 * e.score.min(score);
    };
    let mut seeds = seeds;
    seeds.sort_by(|a, b| b.score.total_cmp(&a.score));
    if seeds.first().is_none_or(|s| s.score < MIN_SCORE) {
        return Ok(Vec::new());
    }
    for s in &seeds {
        add(&s.id, s.score, s.why);
    }
    for s in seeds.iter().filter(|s| s.score >= MIN_SCORE).take(8) {
        if s.id.starts_with("file:") {
            let mut syms = graph::nodes_where(
                c,
                "type = 'symbol' AND path = (SELECT path FROM nodes WHERE id = ?1)",
                &[&s.id],
            )?;
            let mut scored: Vec<(i64, String)> = Vec::new();
            for n in syms.drain(..) {
                scored.push((walks::in_degree(c, &n.id)?, n.id));
            }
            scored.sort_by_key(|s| std::cmp::Reverse(s.0));
            for (_, id) in scored.into_iter().take(5) {
                add(&id, s.score * 0.5, "in named file");
            }
            continue;
        }
        for (n, _) in graph::neighbors(c, &s.id, "calls", Dir::Out)?
            .into_iter()
            .take(6)
        {
            add(&n.id, s.score * 0.35, "called by a match");
        }
        for (n, _) in graph::neighbors(c, &s.id, "calls", Dir::In)?
            .into_iter()
            .take(6)
        {
            if n.ntype != "test" {
                add(&n.id, s.score * 0.35, "calls a match");
            }
        }
        for n in walks::direct_tests(c, &s.id)?.into_iter().take(3) {
            add(&n.id, s.score * 0.3, "tests a match");
        }
        // Symbol <- about <- Fact, Decision; Symbol <- located_in <- Root cause <- caused_by <- Bug
        for rel in ["about", "located_in"] {
            for (n, _) in graph::neighbors(c, &s.id, rel, Dir::In)?
                .into_iter()
                .take(4)
            {
                if n.origin != "earned" {
                    continue;
                }
                add(&n.id, s.score * 0.6, "known about a match");
                for (bug, _) in graph::neighbors(c, &n.id, "caused_by", Dir::In)?
                    .into_iter()
                    .take(2)
                {
                    add(&bug.id, s.score * 0.55, "known bug in a match");
                }
            }
        }
    }
    let mut out: Vec<Ranked> = acc.into_values().collect();
    for r in out.iter_mut() {
        if r.id.starts_with("sym:") {
            r.score += 0.3 * (1.0 + walks::in_degree(c, &r.id)? as f64).ln();
        } else if let Some(f) = record_weight(c, &r.id)? {
            r.score *= f;
        }
    }
    out.retain(|r| r.score > 0.0);
    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    Ok(out)
}

/// Learned records rank by state: superseded ones drop out, contradicted and
/// suspect ones rank down, verified ones up. `None` for code nodes.
fn record_weight(c: &rusqlite::Connection, id: &str) -> Result<Option<f64>> {
    let Some(n) = graph::node(c, id)? else {
        return Ok(None);
    };
    if n.origin != "earned" || n.ntype == "episode" {
        return Ok(None);
    }
    let mut w = match n.status.as_deref() {
        Some("superseded") => return Ok(Some(0.0)),
        Some("verified") => 1.1,
        Some("suspect") => 0.7,
        _ => 1.0,
    };
    let contradicted: i64 = c.query_row(
        "SELECT COUNT(*) FROM edges WHERE rel = 'contradicts' AND (src = ?1 OR dst = ?1)",
        [id],
        |r| r.get(0),
    )?;
    if contradicted > 0 {
        w *= 0.5;
    }
    Ok(Some(w))
}
