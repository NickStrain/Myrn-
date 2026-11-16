//! Indexer: code and Git history into derived nodes. No model involved.
//! Re-parses a file only when its content hash changes.

pub mod body_hash;
pub mod git;
pub mod lang;
pub mod parse;
pub mod symbols;
pub mod walk;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use myrn_model::id::{file_id, module_id, parent_dir, symbol_id, test_id};
use myrn_store::{fts, graph, graph::Node};
use rusqlite::{params, Connection};
use serde::Serialize;

use crate::config::Config;
use parse::{FileParser, Parsed};
use walk::FileStat;

#[derive(Debug, Default, Clone, Serialize)]
pub struct IndexReport {
    pub files: usize,
    pub parsed: usize,
    pub removed: usize,
    pub symbols: usize,
    pub edges: Option<symbols::EdgeCounts>,
    pub commits: usize,
    pub errors: Vec<String>,
    pub millis: u128,
}

impl IndexReport {
    pub fn changed(&self) -> bool {
        self.parsed > 0 || self.removed > 0 || self.commits > 0
    }
}

pub fn content_hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..16].to_string()
}

/// Bring the graph up to date with the working tree and Git history.
pub fn refresh(
    conn: &mut Connection,
    root: &Path,
    cfg: &Config,
    ignore_file: &Path,
) -> Result<IndexReport> {
    let started = Instant::now();
    let mut report = IndexReport::default();
    let stats = walk::walk(root, ignore_file, cfg.index.max_file_bytes)?;
    report.files = stats.len();

    let known: HashMap<String, (String, i64, i64)> = {
        let mut stmt = conn.prepare("SELECT path, content_hash, size, mtime FROM files")?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, (r.get(1)?, r.get(2)?, r.get(3)?))))?;
        rows.collect::<rusqlite::Result<_>>()?
    };

    let mut to_parse: Vec<(FileStat, Vec<u8>, String)> = Vec::new();
    let mut touched: Vec<&FileStat> = Vec::new();
    for st in &stats {
        if let Some((_, size, mtime)) = known.get(&st.path) {
            if *size == st.size as i64 && *mtime == st.mtime {
                continue;
            }
        }
        let Ok(bytes) = std::fs::read(&st.abs) else {
            continue;
        };
        let hash = content_hash(&bytes);
        match known.get(&st.path) {
            Some((h, _, _)) if *h == hash => touched.push(st),
            _ => to_parse.push((st.clone(), bytes, hash)),
        }
    }
    let present: HashSet<&str> = stats.iter().map(|s| s.path.as_str()).collect();
    let removed: Vec<&String> = known
        .keys()
        .filter(|p| !present.contains(p.as_str()))
        .collect();

    let parsed = parse_all(to_parse);
    let tx = conn.transaction()?;
    for path in &removed {
        delete_file(&tx, path)?;
    }
    for (st, hash, result) in &parsed {
        delete_file(&tx, &st.path)?;
        match result {
            Ok(p) => persist(&tx, st, hash, Some(p), None)?,
            Err(e) => {
                report.errors.push(format!("{}: {e}", st.path));
                persist(&tx, st, hash, None, Some(&e.to_string()))?
            }
        }
    }
    for st in touched {
        tx.execute(
            "UPDATE files SET mtime = ?2, size = ?3 WHERE path = ?1",
            params![st.path, st.mtime, st.size as i64],
        )?;
    }
    report.parsed = parsed.len();
    report.removed = removed.len();
    let never_resolved = tx
        .query_row("SELECT 1 FROM meta WHERE key = 'resolved'", [], |_| Ok(()))
        .is_err();
    if report.parsed > 0 || report.removed > 0 || never_resolved {
        gc_modules(&tx)?;
        report.edges = Some(symbols::resolve_all(&tx, root)?);
        tx.execute(
            "INSERT OR REPLACE INTO meta(key, value) VALUES ('resolved', '1')",
            [],
        )?;
    }
    tx.commit()?;

    match git::index_history(conn, root, cfg.index.history_depth) {
        Ok(n) => report.commits = n,
        Err(e) => report.errors.push(format!("git history: {e}")),
    }
    report.symbols = graph::count(
        conn,
        "SELECT COUNT(*) FROM nodes WHERE type IN ('symbol', 'test')",
    )? as usize;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    conn.execute(
        "INSERT OR REPLACE INTO meta(key, value) VALUES ('last_refresh', ?1)",
        [now.to_string()],
    )?;
    report.millis = started.elapsed().as_millis();
    Ok(report)
}

type ParseResult = (FileStat, String, Result<Parsed>);

/// Parse files on all cores.
fn parse_all(files: Vec<(FileStat, Vec<u8>, String)>) -> Vec<ParseResult> {
    if files.is_empty() {
        return Vec::new();
    }
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(files.len().div_ceil(16))
        .max(1);
    let chunk = files.len().div_ceil(threads);
    let mut chunks: Vec<Vec<(FileStat, Vec<u8>, String)>> = Vec::new();
    let mut it = files.into_iter().peekable();
    while it.peek().is_some() {
        chunks.push(it.by_ref().take(chunk).collect());
    }
    std::thread::scope(|s| {
        let handles: Vec<_> = chunks
            .into_iter()
            .map(|c| {
                s.spawn(move || {
                    let mut parser = FileParser::new();
                    c.into_iter()
                        .map(|(st, bytes, hash)| {
                            let r = parser.parse(st.lang, &st.path, &bytes);
                            (st, hash, r)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect()
    })
}

/// Remove everything derived from one file.
fn delete_file(conn: &Connection, path: &str) -> Result<()> {
    fts::remove_path(conn, path)?;
    conn.execute("DELETE FROM edges WHERE src IN (SELECT id FROM nodes WHERE path = ?1 AND type != 'module')", [path])?;
    conn.execute(
        "DELETE FROM nodes WHERE path = ?1 AND type != 'module'",
        [path],
    )?;
    conn.execute("DELETE FROM refs WHERE path = ?1", [path])?;
    conn.execute("DELETE FROM files WHERE path = ?1", [path])?;
    Ok(())
}

fn persist(
    conn: &Connection,
    st: &FileStat,
    hash: &str,
    parsed: Option<&Parsed>,
    error: Option<&str>,
) -> Result<()> {
    let fid = file_id(&st.path);
    let file_name = st.path.rsplit('/').next().unwrap_or(&st.path).to_string();
    let lang = st.lang.name();
    let rowid = graph::upsert_node(
        conn,
        &Node {
            id: fid.clone(),
            ntype: "file".into(),
            origin: "derived".into(),
            kind: Some(lang.into()),
            title: Some(file_name.clone()),
            qname: Some(st.path.clone()),
            path: Some(st.path.clone()),
            lang: Some(lang.into()),
            body_hash: Some(hash.into()),
            status: Some("current".into()),
            ..Default::default()
        },
    )?;
    fts::index_node(conn, rowid, &file_name, "", &st.path, "")?;

    // Module chain: file -> its directory -> parent directories.
    let mut dir = parent_dir(&st.path).to_string();
    graph::insert_edge(conn, &fid, "part_of", &module_id(&dir), None)?;
    loop {
        conn.execute(
            "INSERT OR IGNORE INTO nodes (id, type, origin, kind, title, qname, path, status)
             VALUES (?1, 'module', 'derived', 'directory', ?2, ?3, ?3, 'current')",
            params![
                module_id(&dir),
                dir.rsplit('/')
                    .next()
                    .filter(|s| !s.is_empty())
                    .unwrap_or("."),
                dir
            ],
        )?;
        if dir.is_empty() {
            break;
        }
        let parent = parent_dir(&dir).to_string();
        graph::insert_edge(conn, &module_id(&dir), "part_of", &module_id(&parent), None)?;
        dir = parent;
    }

    let mut n_syms = 0;
    if let Some(p) = parsed {
        let ids: Vec<String> = p
            .defs
            .iter()
            .map(|d| {
                if d.is_test {
                    test_id(&st.path, &d.qname)
                } else {
                    symbol_id(&st.path, &d.qname)
                }
            })
            .collect();
        for (d, id) in p.defs.iter().zip(&ids) {
            let rowid = graph::upsert_node(
                conn,
                &Node {
                    id: id.clone(),
                    ntype: if d.is_test { "test" } else { "symbol" }.into(),
                    origin: "derived".into(),
                    kind: Some(d.kind.into()),
                    title: Some(d.name.clone()),
                    qname: Some(d.qname.clone()),
                    doc: d.doc.clone(),
                    signature: Some(d.signature.clone()),
                    path: Some(st.path.clone()),
                    lang: Some(lang.into()),
                    start_line: Some(d.start_line),
                    end_line: Some(d.end_line),
                    body_hash: Some(d.body_hash.clone()),
                    status: Some("current".into()),
                    ..Default::default()
                },
            )?;
            fts::index_node(
                conn,
                rowid,
                &d.name,
                &d.qname,
                &st.path,
                d.doc.as_deref().unwrap_or(""),
            )?;
            graph::insert_edge(conn, id, "defined_in", &fid, None)?;
        }
        n_syms = p.defs.len();
        let mut stmt = conn.prepare_cached(
            "INSERT INTO refs (src, path, kind, name, qualifier, scope, line) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;
        for r in &p.refs {
            let d = &p.defs[r.from];
            stmt.execute(params![
                ids[r.from],
                st.path,
                r.kind.as_str(),
                r.name,
                r.qualifier,
                d.module,
                r.line
            ])?;
        }
        for i in &p.imports {
            stmt.execute(params![
                fid, st.path, "import", i.spec, i.alias, i.module, i.line
            ])?;
        }
    }
    let error = error.map(str::to_string).or_else(|| {
        parsed
            .filter(|p| p.has_errors)
            .map(|_| "syntax errors; indexed as far as possible".to_string())
    });
    conn.execute(
        "INSERT OR REPLACE INTO files (path, lang, content_hash, size, mtime, symbols, error)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            st.path,
            lang,
            hash,
            st.size as i64,
            st.mtime,
            n_syms as i64,
            error
        ],
    )?;
    Ok(())
}

/// Drop module nodes that no longer contain anything.
fn gc_modules(conn: &Connection) -> Result<()> {
    loop {
        let n = conn.execute(
            "DELETE FROM nodes WHERE type = 'module'
             AND id NOT IN (SELECT dst FROM edges WHERE rel = 'part_of')",
            [],
        )?;
        conn.execute(
            "DELETE FROM edges WHERE rel = 'part_of' AND src NOT IN (SELECT id FROM nodes)",
            [],
        )?;
        if n == 0 {
            return Ok(());
        }
    }
}
