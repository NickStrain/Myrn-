//! Nodes and edges: reads, writes and traversal.

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub ntype: String,
    pub origin: String,
    pub kind: Option<String>,
    pub title: Option<String>,
    pub qname: Option<String>,
    pub summary: Option<String>,
    pub doc: Option<String>,
    pub signature: Option<String>,
    pub path: Option<String>,
    pub lang: Option<String>,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    pub body_hash: Option<String>,
    pub status: Option<String>,
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Edge {
    pub src: String,
    pub rel: String,
    pub dst: String,
    pub status: Option<String>,
}

pub const NODE_COLUMNS: &str = "id, type, origin, kind, title, qname, summary, doc, signature, \
     path, lang, start_line, end_line, body_hash, status, extra";

impl Node {
    pub fn from_row(r: &Row<'_>) -> rusqlite::Result<Node> {
        Ok(Node {
            id: r.get(0)?,
            ntype: r.get(1)?,
            origin: r.get(2)?,
            kind: r.get(3)?,
            title: r.get(4)?,
            qname: r.get(5)?,
            summary: r.get(6)?,
            doc: r.get(7)?,
            signature: r.get(8)?,
            path: r.get(9)?,
            lang: r.get(10)?,
            start_line: r.get(11)?,
            end_line: r.get(12)?,
            body_hash: r.get(13)?,
            status: r.get(14)?,
            extra: r.get(15)?,
        })
    }

    /// `path:start-end`, or just the path.
    pub fn location(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        match (self.start_line, self.end_line) {
            (Some(s), Some(e)) if s != e => format!("{path}:{s}-{e}"),
            (Some(s), _) => format!("{path}:{s}"),
            _ => path.to_string(),
        }
    }

    /// Best human-readable name.
    pub fn display_name(&self) -> &str {
        self.qname
            .as_deref()
            .or(self.title.as_deref())
            .or(self.path.as_deref())
            .unwrap_or(&self.id)
    }
}

/// Insert or replace a node; returns its rowid (used as the search rowid).
pub fn upsert_node(conn: &Connection, n: &Node) -> Result<i64> {
    conn.execute(
        "INSERT OR REPLACE INTO nodes (id, type, origin, kind, title, qname, summary, doc,
             signature, path, lang, start_line, end_line, body_hash, status, extra)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![
            n.id,
            n.ntype,
            n.origin,
            n.kind,
            n.title,
            n.qname,
            n.summary,
            n.doc,
            n.signature,
            n.path,
            n.lang,
            n.start_line,
            n.end_line,
            n.body_hash,
            n.status,
            n.extra
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn insert_edge(
    conn: &Connection,
    src: &str,
    rel: &str,
    dst: &str,
    status: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO edges (src, rel, dst, status) VALUES (?1, ?2, ?3, ?4)",
        params![src, rel, dst, status],
    )?;
    Ok(())
}

pub fn node(conn: &Connection, id: &str) -> Result<Option<Node>> {
    Ok(conn
        .query_row(
            &format!("SELECT {NODE_COLUMNS} FROM nodes WHERE id = ?1"),
            [id],
            Node::from_row,
        )
        .optional()?)
}

pub fn nodes_where(
    conn: &Connection,
    clause: &str,
    args: &[&dyn rusqlite::ToSql],
) -> Result<Vec<Node>> {
    let mut stmt = conn.prepare(&format!("SELECT {NODE_COLUMNS} FROM nodes WHERE {clause}"))?;
    let rows = stmt.query_map(args, Node::from_row)?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Out,
    In,
}

/// Neighbours over one relation, with the edge status. Dangling edges are skipped.
pub fn neighbors(
    conn: &Connection,
    id: &str,
    rel: &str,
    dir: Dir,
) -> Result<Vec<(Node, Option<String>)>> {
    let (this, other) = match dir {
        Dir::Out => ("src", "dst"),
        Dir::In => ("dst", "src"),
    };
    let cols: String = NODE_COLUMNS
        .split(", ")
        .map(|c| format!("n.{c}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut stmt = conn.prepare(&format!(
        "SELECT {cols}, e.status FROM edges e JOIN nodes n ON n.id = e.{other}
         WHERE e.{this} = ?1 AND e.rel = ?2
         ORDER BY n.path, n.start_line"
    ))?;
    let rows = stmt.query_map(params![id, rel], |r| {
        Ok((Node::from_row(r)?, r.get::<_, Option<String>>(16)?))
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// All edges touching a node, in either direction.
pub fn edges_of(conn: &Connection, id: &str) -> Result<Vec<Edge>> {
    let mut stmt = conn.prepare(
        "SELECT src, rel, dst, status FROM edges WHERE src = ?1
         UNION ALL
         SELECT src, rel, dst, status FROM edges WHERE dst = ?1",
    )?;
    let rows = stmt.query_map([id], |r| {
        Ok(Edge {
            src: r.get(0)?,
            rel: r.get(1)?,
            dst: r.get(2)?,
            status: r.get(3)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

pub fn count(conn: &Connection, sql: &str) -> Result<i64> {
    Ok(conn.query_row(sql, [], |r| r.get(0))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Db;

    fn sym(id: &str, path: &str) -> Node {
        Node {
            id: id.into(),
            ntype: "symbol".into(),
            origin: "derived".into(),
            path: Some(path.into()),
            ..Default::default()
        }
    }

    #[test]
    fn neighbours_both_ways() {
        let db = Db::open_in_memory().unwrap();
        let c = &db.conn;
        upsert_node(c, &sym("a", "x.rs")).unwrap();
        upsert_node(c, &sym("b", "x.rs")).unwrap();
        insert_edge(c, "a", "calls", "b", Some("resolved")).unwrap();
        insert_edge(c, "a", "calls", "missing", Some("resolved")).unwrap();
        let out = neighbors(c, "a", "calls", Dir::Out).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0.id, "b");
        assert_eq!(out[0].1.as_deref(), Some("resolved"));
        let inc = neighbors(c, "b", "calls", Dir::In).unwrap();
        assert_eq!(inc[0].0.id, "a");
    }
}
