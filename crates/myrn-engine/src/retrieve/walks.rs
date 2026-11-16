//! Graph walks: neighbours, members, and bounded caller traversal for impact.

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use myrn_store::graph::{self, Dir, Node};
use rusqlite::Connection;

/// Methods and nested items of a type, class or module-like symbol.
pub fn members(conn: &Connection, n: &Node) -> Result<Vec<Node>> {
    let (Some(path), Some(qname)) = (&n.path, &n.qname) else {
        return Ok(Vec::new());
    };
    if !matches!(
        n.kind.as_deref(),
        Some("struct" | "enum" | "class" | "trait" | "interface" | "union" | "type")
    ) {
        return Ok(Vec::new());
    }
    let sep = if n.lang.as_deref() == Some("rust") {
        "::"
    } else {
        "."
    };
    let prefix = format!("{qname}{sep}");
    let like = format!("{}%", prefix.replace('%', "\\%").replace('_', "\\_"));
    let mut v = graph::nodes_where(
        conn,
        "type = 'symbol' AND qname LIKE ?1 ESCAPE '\\' ORDER BY path = ?2 DESC, start_line",
        &[&like, path],
    )?;
    // Direct members only.
    v.retain(|m| !m.qname.as_deref().unwrap_or("")[prefix.len()..].contains(sep));
    Ok(v)
}

/// Tests that call this symbol directly.
pub fn direct_tests(conn: &Connection, id: &str) -> Result<Vec<Node>> {
    Ok(graph::neighbors(conn, id, "tests", Dir::In)?
        .into_iter()
        .map(|(n, _)| n)
        .collect())
}

pub struct Reach {
    /// Callers with their distance (1 = direct).
    pub callers: Vec<(Node, usize, Option<String>)>,
    /// Tests that reach the targets through callers, with distance.
    pub tests: Vec<(Node, usize)>,
    pub truncated: bool,
}

/// Walk incoming `calls` edges breadth-first from the targets.
pub fn callers(
    conn: &Connection,
    targets: &[String],
    max_depth: usize,
    cap: usize,
) -> Result<Reach> {
    let mut seen: HashSet<String> = targets.iter().cloned().collect();
    let mut queue: VecDeque<(String, usize)> = targets.iter().map(|t| (t.clone(), 0)).collect();
    let mut callers = Vec::new();
    let mut tests: HashMap<String, (Node, usize)> = HashMap::new();
    let mut truncated = false;
    while let Some((id, d)) = queue.pop_front() {
        if d >= max_depth {
            continue;
        }
        for (n, status) in graph::neighbors(conn, &id, "calls", Dir::In)? {
            if !seen.insert(n.id.clone()) {
                continue;
            }
            if n.ntype == "test" {
                tests.entry(n.id.clone()).or_insert((n, d + 1));
                continue;
            }
            if callers.len() >= cap {
                truncated = true;
                continue;
            }
            queue.push_back((n.id.clone(), d + 1));
            callers.push((n, d + 1, status));
        }
    }
    let mut tests: Vec<(Node, usize)> = tests.into_values().collect();
    tests.sort_by(|a, b| (a.1, &a.0.path, a.0.start_line).cmp(&(b.1, &b.0.path, b.0.start_line)));
    Ok(Reach {
        callers,
        tests,
        truncated,
    })
}

/// Number of incoming call edges: a rough measure of how central a symbol is.
pub fn in_degree(conn: &Connection, id: &str) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COUNT(*) FROM edges WHERE dst = ?1 AND rel = 'calls'",
        [id],
        |r| r.get(0),
    )?)
}
