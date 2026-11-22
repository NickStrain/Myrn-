//! Full-text index over names, qualified-name terms, paths and docs.
//! Row ids match `nodes.rowid`, so a node's entry can be removed cheaply.

use anyhow::Result;
use rusqlite::{params, Connection};

/// Split identifiers into lowercase words: `TokenStore::lookup_by_id` -> token store lookup by id.
pub fn split_identifier(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut cur = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            if !cur.is_empty() {
                words.push(std::mem::take(&mut cur));
            }
            continue;
        }
        let boundary = c.is_uppercase()
            && !cur.is_empty()
            && (chars[i - 1].is_lowercase()
                || chars[i - 1].is_ascii_digit()
                || chars.get(i + 1).is_some_and(|n| n.is_lowercase()));
        if boundary {
            words.push(std::mem::take(&mut cur));
        }
        cur.extend(c.to_lowercase());
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

pub fn index_node(
    conn: &Connection,
    rowid: i64,
    name: &str,
    qname: &str,
    path: &str,
    doc: &str,
) -> Result<()> {
    let mut terms = split_identifier(qname);
    terms.extend(split_identifier(name));
    terms.dedup();
    conn.execute(
        "INSERT INTO search (rowid, name, terms, path, doc) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            rowid,
            name,
            terms.join(" "),
            split_identifier(path).join(" "),
            doc
        ],
    )?;
    Ok(())
}

/// Remove the search entries for every node of one file. Call before deleting the nodes.
pub fn remove_path(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM search WHERE rowid IN (SELECT rowid FROM nodes WHERE path = ?1)",
        [path],
    )?;
    Ok(())
}

/// Run an FTS query; returns (node id, bm25 score as a positive number, higher is better).
pub fn query(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<(String, f64)>> {
    let terms: Vec<String> = terms
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{}\"*", t.replace('"', "")))
        .collect();
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let q = terms.join(" OR ");
    let mut stmt = conn.prepare(
        "SELECT n.id, -bm25(search, 4.0, 2.0, 1.0, 0.5) AS score
         FROM search JOIN nodes n ON n.rowid = search.rowid
         WHERE search MATCH ?1
         ORDER BY score DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![q, limit as i64], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Search code summaries; returns node ids, best first.
pub fn query_summaries(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<String>> {
    let terms: Vec<String> = terms
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| format!("\"{}\"*", t.replace('"', "")))
        .collect();
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(
        "SELECT id FROM summary_search WHERE summary_search MATCH ?1
         ORDER BY bm25(summary_search) LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![terms.join(" OR "), limit as i64], |r| r.get(0))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_identifiers() {
        assert_eq!(
            split_identifier("TokenStore::lookup_by_id"),
            ["token", "store", "lookup", "by", "id"]
        );
        assert_eq!(split_identifier("HTTPServer"), ["http", "server"]);
        assert_eq!(
            split_identifier("src/auth/session.rs"),
            ["src", "auth", "session", "rs"]
        );
    }
}
