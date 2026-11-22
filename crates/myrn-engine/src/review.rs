//! Gate 2: hand review of learned records.
//!
//! Export records with their evidence and the code they point at, sample them
//! into a Markdown checklist a person marks correct / vague / wrong, and read
//! the marks back into a precision figure.

use std::fmt::Write as _;

use anyhow::Result;
use myrn_store::graph::{self, Dir, Node};
use serde::{Deserialize, Serialize};

use crate::api::Engine;

/// Longest code excerpt per symbol in the checklist.
const CODE_LINES: usize = 40;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    pub id: String,
    #[serde(rename = "type")]
    pub ntype: String,
    pub status: String,
    pub title: String,
    pub body: String,
    pub source: Option<String>,
    /// `kind: value` lines.
    pub evidence: Vec<String>,
    /// `rel target` lines for the typed links.
    pub links: Vec<String>,
    pub code: Vec<CodeRef>,
    /// Where it came from (benchmark task and run), filled in by the caller.
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRef {
    pub name: String,
    pub location: String,
    pub lang: String,
    pub source: String,
    /// The anchor's body hash no longer matches the code.
    pub changed: bool,
}

impl Engine {
    /// Every learned record (summaries included), with what it points at.
    pub fn review_items(&self) -> Result<Vec<ReviewItem>> {
        let c = &self.db.conn;
        let mut stmt =
            c.prepare("SELECT path, data FROM notes ORDER BY visibility = 'shared' DESC, path")?;
        let rows: Vec<(String, String)> = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (_, data) in rows {
            let Ok(note) = serde_json::from_str::<myrn_store::notes::Note>(&data) else {
                continue;
            };
            if !seen.insert(note.id.clone()) {
                continue;
            }
            // The project map is written by `myrn summarize`, not learned in sessions.
            if note.source.as_deref() == Some(crate::summarize::SOURCE) {
                continue;
            }
            // Code: anchors, plus anything the record links to that is code.
            let mut targets: Vec<(String, Option<String>)> = Vec::new();
            let mut astmt =
                c.prepare_cached("SELECT symbol_id, body_hash FROM anchors WHERE note_id = ?1")?;
            for row in astmt.query_map([&note.id], |r| {
                Ok((r.get::<_, Option<String>>(0)?, r.get(1)?))
            })? {
                if let (Some(id), h) = row? {
                    targets.push((id, h));
                }
            }
            let mut linked: Vec<Node> = Vec::new();
            for rel in ["about", "located_in"] {
                linked.extend(
                    graph::neighbors(c, &note.id, rel, Dir::Out)?
                        .into_iter()
                        .map(|x| x.0),
                );
            }
            for n in linked {
                if !targets.iter().any(|(t, _)| *t == n.id) {
                    targets.push((n.id, None));
                }
            }
            let mut code = Vec::new();
            for (id, hash) in targets {
                let Some(n) = graph::node(c, &id)? else {
                    continue;
                };
                if !matches!(n.ntype.as_str(), "symbol" | "test" | "file") {
                    continue;
                }
                code.push(CodeRef {
                    name: n.display_name().to_string(),
                    location: n.location(),
                    lang: n.lang.clone().unwrap_or_default(),
                    source: self.excerpt(&n),
                    changed: hash
                        .as_deref()
                        .is_some_and(|h| !n.body_hash.as_deref().is_some_and(|b| b.starts_with(h))),
                });
            }
            out.push(ReviewItem {
                title: note.display_title(),
                id: note.id.clone(),
                ntype: note.ntype.clone(),
                status: note.status.clone(),
                body: note.body.trim().to_string(),
                source: note.source.clone(),
                evidence: note
                    .evidence
                    .iter()
                    .map(|e| {
                        if e.kind.is_empty() {
                            e.value.clone()
                        } else {
                            format!("{}: {}", e.kind, e.value)
                        }
                    })
                    .collect(),
                links: note
                    .links
                    .iter()
                    .map(|l| {
                        if l.inverse {
                            format!("{} fixes this", l.target)
                        } else {
                            format!("{} {}", l.rel.as_str(), l.target)
                        }
                    })
                    .collect(),
                code,
                origin: String::new(),
            });
        }
        Ok(out)
    }

    fn excerpt(&self, n: &Node) -> String {
        let Some(path) = n.path.as_deref() else {
            return String::new();
        };
        let Ok(text) = std::fs::read_to_string(self.root().join(path)) else {
            return String::new();
        };
        let lines: Vec<&str> = text.lines().collect();
        let (start, end) = match (n.start_line, n.end_line) {
            (Some(s), Some(e)) if n.ntype != "file" => (s.max(1) as usize, e as usize),
            _ => (1, lines.len().min(CODE_LINES)),
        };
        let end = end.min(lines.len()).min(start + CODE_LINES - 1);
        let mut s = lines
            .get(start.saturating_sub(1)..end)
            .unwrap_or_default()
            .join("\n");
        if (n.end_line.unwrap_or(0) as usize) > end && n.ntype != "file" {
            s.push_str("\n…");
        }
        s
    }
}

/// Deterministic sample of `n` items (all of them when `n` is 0 or larger).
pub fn sample(mut items: Vec<ReviewItem>, n: usize, seed: u64) -> Vec<ReviewItem> {
    if n == 0 || n >= items.len() {
        return items;
    }
    // Fisher-Yates with a small xorshift generator: reproducible from the seed.
    let mut x = seed.max(1);
    for i in (1..items.len()).rev() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let j = (x % (i as u64 + 1)) as usize;
        items.swap(i, j);
    }
    items.truncate(n);
    items
}

pub const MARK_LINE: &str = "Mark: [ ] correct  [ ] vague  [ ] wrong";

/// The Markdown checklist.
pub fn checklist(items: &[ReviewItem], title: &str) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# {title}\n");
    let _ = writeln!(
        out,
        "{} records. For each one, put an `x` in exactly one box on its `Mark:` line:\n\n\
         - **correct**: true of this code, specific, and useful to a later session\n\
         - **vague**: not wrong, but too general or obvious to help\n\
         - **wrong**: false, misattributed, or about different code\n\n\
         Then run `myrn debug review --score <this file>`.\n",
        items.len()
    );
    for (i, it) in items.iter().enumerate() {
        let _ = writeln!(
            out,
            "---\n\n## {}. {} · {} · {}\n",
            i + 1,
            it.id,
            it.ntype,
            it.status
        );
        let _ = writeln!(out, "{MARK_LINE}\n");
        if !it.origin.is_empty() {
            let _ = writeln!(out, "From: {}  ", it.origin);
        }
        if let Some(s) = &it.source {
            let _ = writeln!(out, "Source: {s}  ");
        }
        let _ = writeln!(out, "Title: {}\n", it.title);
        for l in it.body.lines() {
            let _ = writeln!(out, "> {l}");
        }
        out.push('\n');
        if !it.evidence.is_empty() {
            let _ = writeln!(out, "Evidence: {}  ", it.evidence.join("; "));
        }
        if !it.links.is_empty() {
            let _ = writeln!(out, "Links: {}  ", it.links.join("; "));
        }
        if it.code.is_empty() {
            let _ = writeln!(out, "\nCode: none linked\n");
        }
        for c in &it.code {
            let _ = writeln!(
                out,
                "\nCode: `{}` {}{}\n\n```{}\n{}\n```",
                c.name,
                c.location,
                if c.changed {
                    " (changed since recorded)"
                } else {
                    ""
                },
                c.lang,
                c.source
            );
        }
        out.push('\n');
    }
    out
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Score {
    pub correct: usize,
    pub vague: usize,
    pub wrong: usize,
    pub unmarked: usize,
    /// More than one box ticked.
    pub ambiguous: usize,
    /// correct / marked
    pub precision: Option<f64>,
    /// (correct + vague) / marked: records that are at least not wrong.
    pub not_wrong: Option<f64>,
    /// Per record type: (type, correct, marked).
    pub by_type: Vec<(String, usize, usize)>,
}

/// Read the marks back from a checklist.
pub fn score(text: &str) -> Score {
    let mut s = Score::default();
    let mut ntype = String::new();
    let mut by: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            ntype = h.split(" · ").nth(1).unwrap_or("").to_string();
            continue;
        }
        let Some(rest) = line.trim().strip_prefix("Mark:") else {
            continue;
        };
        let ticked =
            |w: &str| rest.contains(&format!("[x] {w}")) || rest.contains(&format!("[X] {w}"));
        let marks = [ticked("correct"), ticked("vague"), ticked("wrong")];
        let entry = by.entry(ntype.clone()).or_default();
        match marks {
            [true, false, false] => {
                s.correct += 1;
                entry.0 += 1;
                entry.1 += 1;
            }
            [false, true, false] => {
                s.vague += 1;
                entry.1 += 1;
            }
            [false, false, true] => {
                s.wrong += 1;
                entry.1 += 1;
            }
            [false, false, false] => s.unmarked += 1,
            _ => s.ambiguous += 1,
        }
    }
    let marked = s.correct + s.vague + s.wrong;
    if marked > 0 {
        s.precision = Some(s.correct as f64 / marked as f64);
        s.not_wrong = Some((s.correct + s.vague) as f64 / marked as f64);
    }
    s.by_type = by.into_iter().map(|(k, (c, m))| (k, c, m)).collect();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str) -> ReviewItem {
        ReviewItem {
            id: id.into(),
            ntype: "fact".into(),
            status: "unverified".into(),
            title: "t".into(),
            body: "b".into(),
            source: None,
            evidence: vec![],
            links: vec![],
            code: vec![],
            origin: String::new(),
        }
    }

    #[test]
    fn sampling_is_reproducible() {
        let items: Vec<ReviewItem> = (0..50).map(|i| item(&format!("f{i}"))).collect();
        let a: Vec<String> = sample(items.clone(), 10, 7)
            .into_iter()
            .map(|i| i.id)
            .collect();
        let b: Vec<String> = sample(items.clone(), 10, 7)
            .into_iter()
            .map(|i| i.id)
            .collect();
        let c: Vec<String> = sample(items.clone(), 10, 8)
            .into_iter()
            .map(|i| i.id)
            .collect();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(sample(items, 0, 1).len(), 50);
    }

    #[test]
    fn marks_round_trip() {
        let items: Vec<ReviewItem> = (0..5).map(|i| item(&format!("f{i}"))).collect();
        let text = checklist(&items, "Review");
        let mut n = 0;
        let marked: String = text
            .lines()
            .map(|l| {
                if l == MARK_LINE {
                    n += 1;
                    match n {
                        1 | 2 => l.replace("[ ] correct", "[x] correct"),
                        3 => l.replace("[ ] vague", "[x] vague"),
                        4 => l.replace("[ ] wrong", "[X] wrong"),
                        _ => l.to_string(),
                    }
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let s = score(&marked);
        assert_eq!((s.correct, s.vague, s.wrong, s.unmarked), (2, 1, 1, 1));
        assert_eq!(s.precision, Some(0.5));
        assert_eq!(s.not_wrong, Some(0.75));
        assert_eq!(s.by_type, [("fact".to_string(), 2, 4)]);
    }
}
