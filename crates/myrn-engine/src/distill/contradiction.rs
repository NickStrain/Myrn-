//! Add contradicts edges and rank both records down until one is superseded.
//!
//! Without a model the check is deliberately narrow: two facts or decisions
//! about the same code that say nearly the same words, where one negates what
//! the other states ("call X before Y" / "never call X before Y"). The agent can
//! also state a contradiction itself with a `contradicts` relation.

use std::collections::HashSet;

use anyhow::Result;
pub use myrn_store::notes::claims::opposite;
use rusqlite::{params, Connection};

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
