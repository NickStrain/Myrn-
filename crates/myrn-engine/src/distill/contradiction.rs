//! Add contradicts edges and rank both records down until one is superseded.
//!
//! Without a model the check is deliberately narrow: two facts or decisions
//! about the same code that say nearly the same words, where one negates what
//! the other states ("call X before Y" / "never call X before Y"). The agent can
//! also state a contradiction itself with a `contradicts` relation.

use std::collections::HashSet;

use anyhow::Result;
use rusqlite::{params, Connection};

/// Share of the non-negation words two statements must have in common.
const SAME_CLAIM: f64 = 0.6;

const NEGATIONS: [&str; 16] = [
    "not", "no", "never", "dont", "doesnt", "isnt", "arent", "cannot", "cant", "shouldnt",
    "mustnt", "wont", "without", "neither", "nor", "avoid",
];

fn words(text: &str) -> (HashSet<String>, usize) {
    let mut set = HashSet::new();
    let mut negations = 0;
    for w in text
        .to_lowercase()
        .replace(['\'', '’'], "")
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|w| !w.is_empty())
    {
        if NEGATIONS.contains(&w) {
            negations += 1;
        } else if w.len() > 2 {
            set.insert(w.to_string());
        }
    }
    (set, negations)
}

/// Do these two statements say opposite things about the same subject?
pub fn opposite(a: &str, b: &str) -> bool {
    let (wa, na) = words(a);
    let (wb, nb) = words(b);
    if wa.is_empty() || wb.is_empty() || (na % 2) == (nb % 2) {
        return false;
    }
    let common = wa.intersection(&wb).count() as f64;
    common / wa.union(&wb).count() as f64 >= SAME_CLAIM
}

/// Facts and decisions about any of `about` that `body` contradicts.
/// Returns (id, title) pairs.
pub fn find(conn: &Connection, about: &[String], body: &str) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut stmt = conn.prepare(
        "SELECT n.id, ifnull(n.title, ''), ifnull(n.doc, '') FROM edges e JOIN nodes n ON n.id = e.src
         WHERE e.dst = ?1 AND e.rel = 'about' AND n.type IN ('fact', 'decision')
           AND ifnull(n.status, '') != 'superseded'",
    )?;
    for target in about {
        let rows = stmt.query_map(params![target], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })?;
        for row in rows {
            let (id, title, doc) = row?;
            if seen.insert(id.clone()) && opposite(body, &doc) {
                out.push((id, title));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negated_claims_conflict() {
        assert!(opposite(
            "Call TokenStore::lookup before Session::new.",
            "Never call TokenStore::lookup before Session::new."
        ));
        assert!(opposite(
            "The refresh window is configurable.",
            "The refresh window isn't configurable."
        ));
        assert!(!opposite(
            "The refresh window is configurable.",
            "The refresh window is configurable in project.toml."
        ));
        assert!(!opposite(
            "Never call lookup before new.",
            "Retries do not apply to uploads larger than the chunk size."
        ));
    }
}
