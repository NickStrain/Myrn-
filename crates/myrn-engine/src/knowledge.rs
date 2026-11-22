//! Looking at learned knowledge over time: the record list for `log`, where a
//! record came from (`trace`), how shared knowledge changed between two points
//! (`diff`), and removing a record for good (`forget`).

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use myrn_model::Relation;
use myrn_store::graph;
use myrn_store::notes::{frontmatter, Note};
use myrn_store::{paths, sync};
use serde::Serialize;

use crate::api::Engine;
use crate::capture::episode::Episode;
use crate::summarize;

/// One learned record in the list `log` shows.
#[derive(Debug, Clone, Serialize)]
pub struct RecordRow {
    pub id: String,
    pub ntype: String,
    /// As shown here (derived: suspect, superseded) and as written in the note.
    pub status: String,
    pub written_status: String,
    pub title: String,
    pub visibility: String,
    pub source: Option<String>,
    /// Unix seconds: the note file's last change.
    pub time: i64,
    pub file: String,
}

#[derive(Debug, Clone, Default)]
pub struct RecordFilter {
    pub ntype: Option<String>,
    pub status: Option<String>,
    /// Include the project map's summaries (hundreds of them).
    pub map: bool,
}

/// Where a record came from and what backs it.
#[derive(Debug, Clone, Serialize)]
pub struct Trace {
    pub record: RecordRow,
    pub body: String,
    pub session: Option<Episode>,
    /// (relation, sha, subject, unix time) for commits it cites.
    pub commits: Vec<TracedCommit>,
    pub tests: Vec<String>,
    pub test_runs: Vec<String>,
    pub anchors: Vec<TracedAnchor>,
    pub supersedes: Vec<String>,
    pub superseded_by: Vec<String>,
    pub contradicts: Vec<String>,
    /// Commits that changed the note file (shared notes only), newest first.
    pub note_history: Vec<TracedCommit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TracedCommit {
    pub rel: String,
    pub sha: String,
    pub subject: String,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TracedAnchor {
    pub target: String,
    pub location: Option<String>,
    /// `current`, `changed` or `gone`.
    pub state: String,
}

/// How shared knowledge changed between two points.
#[derive(Debug, Clone, Default, Serialize)]
pub struct KnowledgeDiff {
    pub from: String,
    pub to: String,
    pub added: Vec<DiffItem>,
    pub removed: Vec<DiffItem>,
    pub changed: Vec<DiffItem>,
    /// Project-map summaries added, removed or changed (counted, not listed).
    pub map_changes: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffItem {
    pub id: String,
    pub ntype: String,
    pub title: String,
    /// For changes: `unverified -> verified`, or `text`.
    pub detail: Option<String>,
}

/// What `forget` removed.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Forgotten {
    pub ids: Vec<String>,
    pub files: Vec<PathBuf>,
    /// Other records that pointed at it and had that link removed.
    pub unlinked_from: Vec<String>,
}

impl Engine {
    /// Learned records, newest first.
    pub fn records(&self, filter: &RecordFilter, limit: usize) -> Result<Vec<RecordRow>> {
        let c = &self.db.conn;
        let mut stmt = c.prepare(
            "SELECT path, id, type, visibility, mtime, data FROM notes ORDER BY mtime DESC, id",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, String>(5)?,
            ))
        })?;
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for row in rows {
            let (file, id, ntype, visibility, mtime, data) = row?;
            if !seen.insert(id.clone()) {
                continue;
            }
            let Ok(note) = serde_json::from_str::<Note>(&data) else {
                continue;
            };
            if !filter.map && note.source.as_deref() == Some(summarize::SOURCE) {
                continue;
            }
            if filter.ntype.as_deref().is_some_and(|t| t != ntype) {
                continue;
            }
            let shown = graph::node(c, &id)?
                .and_then(|n| n.status)
                .unwrap_or_else(|| note.status.clone());
            if filter.status.as_deref().is_some_and(|s| s != shown) {
                continue;
            }
            out.push(RecordRow {
                id,
                ntype,
                status: shown,
                written_status: note.status.clone(),
                title: note.display_title(),
                visibility,
                source: note.source.clone(),
                time: mtime / 1_000_000_000,
                file,
            });
            if out.len() >= limit {
                break;
            }
        }
        Ok(out)
    }

    fn record_row(&self, id: &str) -> Result<(RecordRow, Note)> {
        let c = &self.db.conn;
        let row: Option<(String, String, String, i64, String)> = c
            .query_row(
                "SELECT path, type, visibility, mtime, data FROM notes WHERE id = ?1
                 ORDER BY visibility = 'shared' DESC LIMIT 1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .ok();
        let Some((file, ntype, visibility, mtime, data)) = row else {
            bail!("no record `{id}`; see `myrn log`");
        };
        let note: Note = serde_json::from_str(&data)?;
        let shown = graph::node(c, id)?
            .and_then(|n| n.status)
            .unwrap_or_else(|| note.status.clone());
        Ok((
            RecordRow {
                id: id.to_string(),
                ntype,
                status: shown,
                written_status: note.status.clone(),
                title: note.display_title(),
                visibility,
                source: note.source.clone(),
                time: mtime / 1_000_000_000,
                file,
            },
            note,
        ))
    }

    /// Where a record came from: the session that wrote it, the commits and
    /// tests it cites, the code it is anchored to, and what replaced it.
    pub fn trace(&self, id: &str) -> Result<Trace> {
        let (record, note) = self.record_row(id)?;
        let c = &self.db.conn;
        let session = match note.source.as_deref() {
            Some(s) => {
                let sid = s.split_once(':').map(|(_, r)| r).unwrap_or(s);
                self.episode_log(sid)?.map(|(e, _)| e)
            }
            None => None,
        };
        let repo = git2::Repository::open(self.root()).ok();
        let commit_info = |rel: &str, target: &str| -> Option<TracedCommit> {
            let sha = target.trim().trim_start_matches("commit:");
            let repo = repo.as_ref()?;
            let obj = repo.revparse_single(sha).ok()?;
            let cm = obj.peel_to_commit().ok()?;
            Some(TracedCommit {
                rel: rel.to_string(),
                sha: cm.id().to_string(),
                subject: cm.summary().ok().flatten().unwrap_or("").to_string(),
                time: cm.time().seconds(),
            })
        };
        let mut commits = Vec::new();
        for l in &note.links {
            if l.rel == Relation::Fixes && l.inverse {
                commits.extend(commit_info("fixed in", &l.target));
            }
        }
        let mut tests = Vec::new();
        let mut test_runs = Vec::new();
        for e in &note.evidence {
            match e.kind.as_str() {
                "commit" => commits.extend(commit_info("evidence", &e.value)),
                "test" => tests.push(e.value.clone()),
                "test_run" => test_runs.push(e.value.clone()),
                _ => {}
            }
        }
        for l in note.links.iter().filter(|l| l.rel == Relation::VerifiedBy) {
            tests.push(l.target.clone());
        }

        let drift: Vec<sync::Drift> = sync::drift(c, Some(id))?;
        let mut anchors = Vec::new();
        let resolved: Vec<Option<String>> = {
            let mut stmt =
                c.prepare("SELECT symbol_id FROM anchors WHERE note_id = ?1 ORDER BY rowid")?;
            let rows = stmt.query_map([id], |r| r.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        for (i, a) in note.anchors.iter().enumerate() {
            let sym = resolved.get(i).cloned().flatten();
            let node = match &sym {
                Some(s) => graph::node(c, s)?,
                None => None,
            };
            let state = match (&node, &sym) {
                (None, _) => "gone",
                (Some(_), Some(s))
                    if drift
                        .iter()
                        .any(|d| d.symbol_id.as_deref() == Some(s.as_str())) =>
                {
                    "changed"
                }
                _ => "current",
            };
            anchors.push(TracedAnchor {
                target: a.target.clone(),
                location: node.map(|n| n.location()),
                state: state.into(),
            });
        }

        let ids_by = |sql: &str| -> Result<Vec<String>> {
            let mut stmt = c.prepare(sql)?;
            let rows = stmt.query_map([id], |r| r.get(0))?;
            Ok(rows.collect::<rusqlite::Result<_>>()?)
        };
        let supersedes = ids_by("SELECT dst FROM edges WHERE src = ?1 AND rel = 'supersedes'")?;
        let superseded_by = ids_by("SELECT src FROM edges WHERE dst = ?1 AND rel = 'supersedes'")?;
        let mut contradicts =
            ids_by("SELECT dst FROM edges WHERE src = ?1 AND rel = 'contradicts'")?;
        contradicts.extend(ids_by(
            "SELECT src FROM edges WHERE dst = ?1 AND rel = 'contradicts'",
        )?);

        let note_history = match (&repo, record.visibility.as_str()) {
            (Some(repo), "shared") => file_history(repo, &record.file, 20),
            _ => Vec::new(),
        };
        Ok(Trace {
            record,
            body: note.body.trim().to_string(),
            session,
            commits,
            tests,
            test_runs,
            anchors,
            supersedes,
            superseded_by,
            contradicts,
            note_history,
        })
    }

    /// How the shared knowledge (`.myrn/knowledge`) changed between two
    /// points: Git revisions or dates (`2026-11-01`). `to = None` is the
    /// working tree. Private notes are not versioned, so they are not compared.
    pub fn diff_knowledge(&self, from: &str, to: Option<&str>) -> Result<KnowledgeDiff> {
        let repo = git2::Repository::open(self.root()).context("opening the repository")?;
        let (from_label, before) = self.knowledge_at(&repo, Some(from))?;
        let (to_label, after) = self.knowledge_at(&repo, to)?;
        let mut d = KnowledgeDiff {
            from: from_label,
            to: to_label,
            ..Default::default()
        };
        let item = |n: &Note, detail: Option<String>| DiffItem {
            id: n.id.clone(),
            ntype: n.ntype.clone(),
            title: n.display_title(),
            detail,
        };
        let is_map = |n: &Note| n.source.as_deref() == Some(summarize::SOURCE);
        for (id, a) in &after {
            match before.get(id) {
                None if is_map(a) => d.map_changes += 1,
                None => d.added.push(item(a, None)),
                Some(b) if b == a => {}
                Some(_) if is_map(a) => d.map_changes += 1,
                Some(b) => {
                    let detail = if b.status != a.status {
                        format!("{} -> {}", b.status, a.status)
                    } else if b.body.split_whitespace().ne(a.body.split_whitespace()) {
                        "text".to_string()
                    } else {
                        "links or anchors".to_string()
                    };
                    d.changed.push(item(a, Some(detail)));
                }
            }
        }
        for (id, b) in &before {
            if !after.contains_key(id) {
                if is_map(b) {
                    d.map_changes += 1;
                } else {
                    d.removed.push(item(b, None));
                }
            }
        }
        Ok(d)
    }

    /// Shared notes at a revision or date, or in the working tree (`None`).
    fn knowledge_at(
        &self,
        repo: &git2::Repository,
        at: Option<&str>,
    ) -> Result<(String, BTreeMap<String, Note>)> {
        let mut out = BTreeMap::new();
        let Some(at) = at else {
            let dir = self.layout.knowledge_dir();
            walk_notes(&dir, &dir, &mut |rel, text| {
                let n = parse_note(rel, text);
                out.insert(n.id.clone(), n);
            });
            return Ok(("working tree".into(), out));
        };
        let commit = resolve_point(repo, at)?;
        let label = format!(
            "{} ({})",
            &commit.id().to_string()[..12],
            commit.summary().ok().flatten().unwrap_or("")
        );
        let tree = commit.tree()?;
        let prefix = format!("{}/knowledge", paths::REPO_DIR);
        let Ok(entry) = tree.get_path(Path::new(&prefix)) else {
            return Ok((label, out)); // no shared knowledge at that point
        };
        let sub = repo.find_tree(entry.id())?;
        sub.walk(git2::TreeWalkMode::PreOrder, |dir, e| {
            let name = e.name().unwrap_or("");
            if name.ends_with(".md") && !dir.starts_with('.') {
                if let Ok(blob) = repo.find_blob(e.id()) {
                    let text = String::from_utf8_lossy(blob.content());
                    let n = parse_note(&format!("{dir}{name}"), &text);
                    out.insert(n.id.clone(), n);
                }
            }
            git2::TreeWalkResult::Ok
        })?;
        Ok((label, out))
    }

    /// What `forget` would remove: the record, and root causes only it points to.
    pub fn forget_plan(&self, id: &str) -> Result<Forgotten> {
        let c = &self.db.conn;
        let (_, note) = self.record_row(id)?;
        let mut ids = vec![id.to_string()];
        // A root cause exists for its bug: forget it too unless another record uses it.
        for l in note.links.iter().filter(|l| l.rel == Relation::CausedBy) {
            let cause = l.target.trim_start_matches("[[").trim_end_matches("]]");
            let users: i64 = c.query_row(
                "SELECT COUNT(*) FROM edges WHERE dst = ?1 AND src != ?2 AND evidence LIKE 'note:%'",
                [cause, id],
                |r| r.get(0),
            )?;
            if users == 0 && graph::node(c, cause)?.is_some() {
                ids.push(cause.to_string());
            }
        }
        let mut files = Vec::new();
        for i in &ids {
            let mut stmt = c.prepare("SELECT path FROM notes WHERE id = ?1")?;
            let keys: Vec<String> = stmt
                .query_map([i], |r| r.get(0))?
                .collect::<rusqlite::Result<_>>()?;
            files.extend(keys.iter().map(|k| self.note_file(k)));
        }
        let mut unlinked_from = Vec::new();
        for i in &ids {
            let mut stmt = c.prepare(
                "SELECT DISTINCT substr(evidence, 6) FROM edges
                 WHERE dst = ?1 AND evidence LIKE 'note:%' AND substr(evidence, 6) != ?1",
            )?;
            for other in stmt.query_map([i], |r| r.get::<_, String>(0))? {
                let other = other?;
                if !ids.contains(&other) && !unlinked_from.contains(&other) {
                    unlinked_from.push(other);
                }
            }
        }
        Ok(Forgotten {
            ids,
            files,
            unlinked_from,
        })
    }

    /// Permanently remove a record: its note files (every copy), root causes
    /// only it used, links to it from other records, and its index and usage rows.
    pub fn forget(&mut self, id: &str) -> Result<Forgotten> {
        let plan = self.forget_plan(id)?;
        for other in &plan.unlinked_from {
            if let Some(mut ex) = crate::distill::dedupe::find_id(&self.db.conn, other)? {
                let before = ex.note.links.len();
                ex.note.links.retain(|l| {
                    let t = l.target.trim_start_matches("[[").trim_end_matches("]]");
                    !plan.ids.iter().any(|i| i == t)
                });
                if ex.note.links.len() != before {
                    self.put_note(&mut ex.note, Some(&ex.key), &HashSet::new())?;
                }
            }
        }
        for f in &plan.files {
            if f.exists() {
                std::fs::remove_file(f).with_context(|| format!("removing {}", f.display()))?;
            }
        }
        for i in &plan.ids {
            self.db
                .conn
                .execute("DELETE FROM usage WHERE note_id = ?1", [i])?;
        }
        let roots = self.note_roots();
        sync::sync(&mut self.db.conn, &roots, true)?;
        Ok(plan)
    }
}

fn parse_note(rel: &str, text: &str) -> Note {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let stem = file.strip_suffix(".md").unwrap_or(file);
    let folder = rel.split('/').next().filter(|f| *f != file);
    frontmatter::parse(text, stem, folder).note
}

fn walk_notes(root: &Path, dir: &Path, f: &mut dyn FnMut(&str, &str)) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let p = e.path();
        if p.is_dir() {
            walk_notes(root, &p, f);
        } else if name.ends_with(".md") {
            if let (Ok(text), Some(rel)) = (std::fs::read_to_string(&p), paths::repo_rel(root, &p))
            {
                f(&rel, &text);
            }
        }
    }
}

/// A revision (`HEAD~3`, a tag, a sha) or a date (`2026-11-01`: the last
/// commit on HEAD at or before the end of that day, UTC).
fn resolve_point<'r>(repo: &'r git2::Repository, at: &str) -> Result<git2::Commit<'r>> {
    if let Some(end) = date_end(at) {
        let mut walk = repo.revwalk()?;
        walk.push_head()?;
        walk.set_sorting(git2::Sort::TIME)?;
        for oid in walk {
            let c = repo.find_commit(oid?)?;
            if c.time().seconds() <= end {
                return Ok(c);
            }
        }
        bail!("no commit on or before {at}");
    }
    let obj = repo
        .revparse_single(at)
        .with_context(|| format!("`{at}` is not a revision or a date (YYYY-MM-DD)"))?;
    Ok(obj.peel_to_commit()?)
}

/// `YYYY-MM-DD` -> unix seconds at the end of that day (UTC).
fn date_end(s: &str) -> Option<i64> {
    let mut it = s.split('-');
    let (y, m, d) = (
        it.next()?.parse::<i64>().ok()?,
        it.next()?.parse::<i64>().ok()?,
        it.next()?.parse::<i64>().ok()?,
    );
    if it.next().is_some() || s.len() != 10 || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Days from civil (Howard Hinnant).
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + 86_399)
}

/// Commits that changed one file, newest first.
fn file_history(repo: &git2::Repository, path: &str, limit: usize) -> Vec<TracedCommit> {
    let mut out = Vec::new();
    let Ok(mut walk) = repo.revwalk() else {
        return out;
    };
    if walk.push_head().is_err() {
        return out;
    }
    let p = Path::new(path);
    for oid in walk.flatten() {
        let Ok(c) = repo.find_commit(oid) else {
            continue;
        };
        let Ok(tree) = c.tree() else { continue };
        let now = tree.get_path(p).ok().map(|e| e.id());
        let before = c
            .parent(0)
            .ok()
            .and_then(|pc| pc.tree().ok())
            .and_then(|t| t.get_path(p).ok().map(|e| e.id()));
        if now != before {
            out.push(TracedCommit {
                rel: if before.is_none() {
                    "added"
                } else if now.is_none() {
                    "removed"
                } else {
                    "changed"
                }
                .into(),
                sha: c.id().to_string(),
                subject: c.summary().ok().flatten().unwrap_or("").to_string(),
                time: c.time().seconds(),
            });
            if out.len() >= limit {
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_are_end_of_day_utc() {
        assert_eq!(date_end("1970-01-01"), Some(86_399));
        assert_eq!(date_end("2026-11-22"), Some(1_795_391_999));
        assert_eq!(date_end("HEAD~1"), None);
        assert_eq!(date_end("2026-13-01"), None);
    }
}
