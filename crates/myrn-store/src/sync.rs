//! Keep notes and index in step: compare file hashes with the notes table and re-index only what changed.
//!
//! Two passes:
//! 1. **Scan**: stat every note file; re-read and re-parse only files whose
//!    size or mtime changed, and only re-store those whose content hash changed.
//!    Hand edits and `git pull`s are picked up the same way.
//! 2. **Link**: when any note or any code changed, rebuild the note nodes, their
//!    edges, anchors and summaries from the stored parses. Link targets are
//!    resolved against the current code graph, so a link that was dangling
//!    resolves once its target appears. This is cheap: it reads no files.
//!
//! A note is never dropped for being malformed. Whatever can be indexed is,
//! and every problem is recorded in `note_issues` for `status` and `doctor`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Result;
use myrn_model::{Relation, Visibility};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;

use crate::fts;
use crate::graph::{self, Node};
use crate::notes::{frontmatter, wikilink, Note};

/// A directory of notes: the private notes dir or the shared knowledge dir.
#[derive(Debug, Clone)]
pub struct NoteRoot {
    pub dir: PathBuf,
    pub visibility: Visibility,
    /// Prefix for the paths shown to people (`.myrn/knowledge`, `notes`).
    pub label: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct NotesReport {
    /// Note files found.
    pub notes: usize,
    /// Files re-parsed because their content changed.
    pub indexed: usize,
    pub removed: usize,
    /// Whether the link pass ran.
    pub relinked: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub path: String,
    pub note_id: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct NotesSummary {
    pub private: i64,
    pub shared: i64,
    /// Notes with header problems.
    pub malformed: i64,
    /// Links and anchors that resolve to nothing.
    pub dangling: i64,
    /// Files that repeat another file's id.
    pub duplicates: i64,
}

struct Found {
    key: String,
    abs: PathBuf,
    folder: Option<String>,
    stem: String,
    visibility: Visibility,
    size: i64,
    mtime: i64,
}

fn walk(root: &NoteRoot, out: &mut Vec<Found>) {
    fn rec(dir: &Path, rel: &str, root: &NoteRoot, out: &mut Vec<Found>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            // .obsidian/, .trash/, .gitkeep, editor temp files
            if name.starts_with('.') || name.ends_with(".tmp") {
                continue;
            }
            let Ok(meta) = e.metadata() else { continue };
            let child = if rel.is_empty() {
                name.clone()
            } else {
                format!("{rel}/{name}")
            };
            if meta.is_dir() {
                rec(&e.path(), &child, root, out);
            } else if let Some(stem) = name.strip_suffix(".md") {
                out.push(Found {
                    key: format!("{}/{child}", root.label),
                    abs: e.path(),
                    folder: child.split_once('/').map(|(f, _)| f.to_string()),
                    stem: stem.to_string(),
                    visibility: root.visibility,
                    size: meta.len() as i64,
                    mtime: meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_nanos() as i64)
                        .unwrap_or(0),
                });
            }
        }
    }
    rec(&root.dir, "", root, out);
}

pub fn content_hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..16].to_string()
}

/// Bring the note index up to date. `relink` forces the link pass (pass it
/// when the code graph changed, so links follow the code).
pub fn sync(conn: &mut Connection, roots: &[NoteRoot], relink: bool) -> Result<NotesReport> {
    let mut found = Vec::new();
    for r in roots {
        walk(r, &mut found);
    }
    let mut report = NotesReport {
        notes: found.len(),
        ..Default::default()
    };
    let known: HashMap<String, (String, i64, i64)> = {
        let mut stmt = conn.prepare("SELECT path, content_hash, size, mtime FROM notes")?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, (r.get(1)?, r.get(2)?, r.get(3)?))))?;
        rows.collect::<rusqlite::Result<_>>()?
    };
    let tx = conn.transaction()?;
    let present: HashSet<&str> = found.iter().map(|f| f.key.as_str()).collect();
    for key in known.keys().filter(|k| !present.contains(k.as_str())) {
        tx.execute("DELETE FROM notes WHERE path = ?1", [key])?;
        report.removed += 1;
    }
    for f in &found {
        if let Some((_, size, mtime)) = known.get(&f.key) {
            if *size == f.size && *mtime == f.mtime {
                continue;
            }
        }
        let Ok(bytes) = std::fs::read(&f.abs) else {
            continue;
        };
        let hash = content_hash(&bytes);
        if known.get(&f.key).is_some_and(|(h, _, _)| *h == hash) {
            tx.execute(
                "UPDATE notes SET size = ?2, mtime = ?3 WHERE path = ?1",
                params![f.key, f.size, f.mtime],
            )?;
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let mut parsed = frontmatter::parse(&text, &f.stem, f.folder.as_deref());
        if matches!(text, std::borrow::Cow::Owned(_)) {
            parsed
                .problems
                .push("not valid UTF-8; bad bytes replaced".into());
        }
        tx.execute(
            "INSERT OR REPLACE INTO notes
                 (path, id, type, visibility, content_hash, size, mtime, data, problems)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                f.key,
                parsed.note.id,
                parsed.note.ntype,
                f.visibility.as_str(),
                hash,
                f.size,
                f.mtime,
                serde_json::to_string(&parsed.note)?,
                serde_json::to_string(&parsed.problems)?,
            ],
        )?;
        report.indexed += 1;
    }
    let never_linked = tx
        .query_row("SELECT 1 FROM meta WHERE key = 'notes_linked'", [], |_| {
            Ok(())
        })
        .optional()?
        .is_none();
    if relink || never_linked || report.indexed > 0 || report.removed > 0 {
        link(&tx)?;
        tx.execute(
            "INSERT OR REPLACE INTO meta(key, value) VALUES ('notes_linked', '1')",
            [],
        )?;
        report.relinked = true;
    }
    tx.commit()?;
    Ok(report)
}

struct Row {
    path: String,
    visibility: String,
    note: Note,
    problems: Vec<String>,
}

/// Resolves header and body link targets to node ids.
struct Resolver<'a> {
    conn: &'a Connection,
    /// lowercase id -> id
    notes: HashMap<String, String>,
}

fn segments(q: &str) -> Vec<String> {
    q.replace("::", ".")
        .split('.')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn looks_like_sha(t: &str) -> bool {
    (7..=40).contains(&t.len()) && t.chars().all(|c| c.is_ascii_hexdigit())
}

impl Resolver<'_> {
    fn note(&self, t: &str) -> Option<String> {
        self.notes.get(&t.to_lowercase()).cloned()
    }

    fn exists(&self, id: &str) -> Result<bool> {
        Ok(graph::node(self.conn, id)?.is_some())
    }

    /// A note id, a node id, `path#qname`, a commit sha, a file or directory
    /// path, or a qualified name. `hash` picks between same-named symbols.
    fn resolve(&self, t: &str, hash: Option<&str>) -> Result<Option<String>> {
        let t = t.trim();
        if t.is_empty() {
            return Ok(None);
        }
        if let Some(id) = self.note(t) {
            return Ok(Some(id));
        }
        if self.exists(t)? {
            return Ok(Some(t.to_string()));
        }
        if let Some((path, qname)) = t.split_once('#') {
            for id in [
                myrn_model::id::symbol_id(path, qname),
                myrn_model::id::test_id(path, qname),
            ] {
                if self.exists(&id)? {
                    return Ok(Some(id));
                }
            }
            return Ok(None);
        }
        if looks_like_sha(t) {
            let like = format!("commit:{}%", t.to_lowercase());
            let hits: Vec<String> = self
                .conn
                .prepare("SELECT id FROM nodes WHERE type = 'commit' AND id LIKE ?1 LIMIT 2")?
                .query_map([like], |r| r.get(0))?
                .collect::<rusqlite::Result<_>>()?;
            if hits.len() == 1 {
                return Ok(hits.into_iter().next());
            }
        }
        let path = t.trim_start_matches("./").trim_end_matches('/');
        for id in [
            myrn_model::id::file_id(path),
            myrn_model::id::module_id(path),
        ] {
            if self.exists(&id)? {
                return Ok(Some(id));
            }
        }
        let segs = segments(t);
        let Some(last) = segs.last() else {
            return Ok(None);
        };
        let mut hits: Vec<Node> = graph::nodes_where(
            self.conn,
            "title = ?1 AND type IN ('symbol', 'test')",
            &[last],
        )?;
        hits.retain(|n| {
            let have = segments(n.qname.as_deref().unwrap_or(""));
            have.len() >= segs.len() && have[have.len() - segs.len()..] == segs[..]
        });
        hits.sort_by_key(|n| {
            (
                !hash.is_some_and(|h| n.body_hash.as_deref().is_some_and(|b| b.starts_with(h))),
                segments(n.qname.as_deref().unwrap_or("")) != segs,
                n.ntype == "test",
                n.path.clone(),
            )
        });
        Ok(hits.into_iter().next().map(|n| n.id))
    }
}

fn add_edge(tx: &Transaction<'_>, src: &str, rel: &str, dst: &str, note_id: &str) -> Result<()> {
    tx.execute(
        "INSERT OR IGNORE INTO edges (src, rel, dst, status, evidence)
         VALUES (?1, ?2, ?3, 'resolved', ?4)",
        params![src, rel, dst, format!("note:{note_id}")],
    )?;
    Ok(())
}

/// Rebuild every note-derived row from the stored parses.
fn link(tx: &Transaction<'_>) -> Result<()> {
    tx.execute_batch(
        "DELETE FROM search WHERE rowid IN
             (SELECT rowid FROM nodes WHERE origin = 'earned' AND type != 'episode');
         DELETE FROM nodes WHERE origin = 'earned' AND type != 'episode';
         DELETE FROM edges WHERE evidence LIKE 'note:%';
         DELETE FROM anchors;
         DELETE FROM note_issues;
         UPDATE nodes SET summary = NULL WHERE origin = 'derived' AND summary IS NOT NULL;",
    )?;
    let rows: Vec<Row> = {
        let mut stmt = tx.prepare(
            "SELECT path, visibility, data, problems FROM notes
             ORDER BY visibility = 'shared' DESC, path",
        )?;
        let it = stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
            ))
        })?;
        let mut out = Vec::new();
        let mut stale = Vec::new();
        for row in it {
            let (path, visibility, data, problems) = row?;
            let Ok(note) = serde_json::from_str::<Note>(&data) else {
                stale.push(path); // written by another version
                continue;
            };
            let problems = problems
                .and_then(|p| serde_json::from_str(&p).ok())
                .unwrap_or_default();
            out.push(Row {
                path,
                visibility,
                note,
                problems,
            });
        }
        // Forget unreadable rows so the next scan re-parses their files.
        for path in stale {
            tx.execute("DELETE FROM notes WHERE path = ?1", [path])?;
        }
        out
    };
    let issue = |path: &str, id: &str, kind: &str, detail: &str| -> Result<()> {
        tx.execute(
            "INSERT INTO note_issues (path, note_id, kind, detail) VALUES (?1, ?2, ?3, ?4)",
            params![path, id, kind, detail],
        )?;
        Ok(())
    };

    // One node per id: shared copies win over private ones, then path order.
    let mut winners: BTreeMap<String, &Row> = BTreeMap::new();
    for r in &rows {
        for p in &r.problems {
            issue(&r.path, &r.note.id, "header", p)?;
        }
        match winners.get(&r.note.id) {
            Some(w) => issue(
                &r.path,
                &r.note.id,
                "duplicate",
                &format!("same id as {}; this copy is not indexed", w.path),
            )?,
            None => {
                winners.insert(r.note.id.clone(), r);
            }
        }
    }
    let resolver = Resolver {
        conn: tx,
        notes: winners
            .keys()
            .map(|id| (id.to_lowercase(), id.clone()))
            .collect(),
    };

    for (id, r) in &winners {
        let n = &r.note;
        let summary = n.ntype == "summary";
        if !summary {
            let extra = serde_json::json!({
                "visibility": r.visibility,
                "file": r.path,
                "evidence": n.evidence,
                "source": n.source,
                "signature": n.signature,
            });
            let rowid = graph::upsert_node(
                tx,
                &Node {
                    id: id.clone(),
                    ntype: n.ntype.clone(),
                    origin: "earned".into(),
                    kind: Some("note".into()),
                    title: Some(n.display_title()),
                    doc: Some(n.body.trim().to_string()),
                    status: Some(n.status.clone()),
                    extra: Some(extra.to_string()),
                    ..Default::default()
                },
            )?;
            if let Some(c) = n.confidence {
                tx.execute(
                    "UPDATE nodes SET confidence = ?2 WHERE id = ?1",
                    params![id, c],
                )?;
            }
            fts::index_node(tx, rowid, &n.display_title(), "", "", n.body.trim())?;
            if let Some(ep) = n.source.as_deref().filter(|s| s.starts_with("episode:")) {
                add_edge(tx, ep, Relation::Produced.as_str(), id, id)?;
            }
        }

        let mut summary_targets = Vec::new();
        for l in &n.links {
            match resolver.resolve(&l.target, None)? {
                Some(dst) => {
                    if summary && l.rel == Relation::About {
                        summary_targets.push(dst);
                    } else if l.inverse {
                        add_edge(tx, &dst, l.rel.as_str(), id, id)?;
                    } else {
                        add_edge(tx, id, l.rel.as_str(), &dst, id)?;
                    }
                }
                None => issue(
                    &r.path,
                    id,
                    "dangling",
                    &format!("{} -> `{}` matches nothing", l.rel.as_str(), l.target),
                )?,
            }
        }
        for a in &n.anchors {
            let sym = resolver.resolve(&a.target, a.hash.as_deref())?;
            tx.execute(
                "INSERT INTO anchors (note_id, symbol_id, body_hash) VALUES (?1, ?2, ?3)",
                params![id, sym, a.hash],
            )?;
            match sym {
                Some(s) if summary && summary_targets.is_empty() => summary_targets.push(s),
                Some(_) => {}
                None => issue(
                    &r.path,
                    id,
                    "dangling",
                    &format!("anchor `{}` matches no symbol", a.target),
                )?,
            }
        }
        if summary {
            if summary_targets.is_empty() {
                issue(&r.path, id, "dangling", "summary is not `about` any code")?;
            }
            for t in summary_targets {
                tx.execute(
                    "UPDATE nodes SET summary = ?2 WHERE id = ?1",
                    params![t, n.body.trim()],
                )?;
            }
            continue;
        }
        for target in wikilink::extract(&n.body) {
            match resolver.note(&target) {
                Some(dst) if dst != *id => add_edge(tx, id, Relation::Related.as_str(), &dst, id)?,
                Some(_) => {}
                None => match resolver.resolve(&target, None)? {
                    Some(dst) => add_edge(tx, id, Relation::Related.as_str(), &dst, id)?,
                    None => issue(
                        &r.path,
                        id,
                        "dangling",
                        &format!("[[{target}]] matches nothing"),
                    )?,
                },
            }
        }
    }
    Ok(())
}

pub fn summary(conn: &Connection) -> Result<NotesSummary> {
    let count = |sql: &str| graph::count(conn, sql);
    Ok(NotesSummary {
        private: count("SELECT COUNT(*) FROM notes WHERE visibility = 'private'")?,
        shared: count("SELECT COUNT(*) FROM notes WHERE visibility = 'shared'")?,
        malformed: count("SELECT COUNT(DISTINCT path) FROM note_issues WHERE kind = 'header'")?,
        dangling: count("SELECT COUNT(*) FROM note_issues WHERE kind = 'dangling'")?,
        duplicates: count("SELECT COUNT(*) FROM note_issues WHERE kind = 'duplicate'")?,
    })
}

pub fn issues(conn: &Connection, limit: usize) -> Result<Vec<Issue>> {
    let mut stmt = conn.prepare(
        "SELECT path, note_id, kind, detail FROM note_issues
         ORDER BY kind = 'header' DESC, path LIMIT ?1",
    )?;
    let rows = stmt.query_map([limit as i64], |r| {
        Ok(Issue {
            path: r.get(0)?,
            note_id: r.get(1)?,
            kind: r.get(2)?,
            detail: r.get(3)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}
