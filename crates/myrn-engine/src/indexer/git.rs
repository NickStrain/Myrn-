//! Git history: commits, the files and symbols each one changed, renames and
//! reverts. Uses libgit2 directly rather than shelling out.

use std::path::Path;

use anyhow::Result;
use git2::{Delta, DiffFindOptions, DiffOptions, Oid, Repository, Sort};
use myrn_model::id::{commit_id, symbol_id, test_id};
use myrn_store::graph::{self, Node};
use rusqlite::{params, Connection};

use super::lang::Lang;
use super::parse::FileParser;

/// The first commit of the repository's history, which identifies the project.
pub fn root_commit(repo: &Repository) -> Option<String> {
    let mut walk = repo.revwalk().ok()?;
    walk.push_head().ok()?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE).ok()?;
    walk.next()?.ok().map(|o| o.to_string())
}

pub fn head_sha(repo: &Repository) -> Option<String> {
    repo.head()
        .ok()?
        .peel_to_commit()
        .ok()
        .map(|c| c.id().to_string())
}

/// Index commits that are not in the graph yet, newest first, up to `depth`.
pub fn index_history(conn: &Connection, root: &Path, depth: usize) -> Result<usize> {
    let Ok(repo) = Repository::discover(root) else {
        return Ok(0);
    };
    let Some(head) = head_sha(&repo) else {
        return Ok(0);
    };
    if conn
        .query_row("SELECT value FROM meta WHERE key = 'git_head'", [], |r| {
            r.get::<_, String>(0)
        })
        .ok()
        .as_deref()
        == Some(head.as_str())
    {
        backfill_adds(conn, &repo, &mut FileParser::new())?;
        return Ok(0);
    }
    let mut parser = FileParser::new();
    backfill_adds(conn, &repo, &mut parser)?;
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(Sort::TIME)?;
    let mut added = 0;
    let tx = conn.unchecked_transaction()?;
    for oid in walk.take(depth) {
        let oid = oid?;
        if graph::node(&tx, &commit_id(&oid.to_string()))?.is_some() {
            break;
        }
        index_commit(&tx, &repo, oid, &mut parser)?;
        added += 1;
    }
    tx.execute(
        "INSERT INTO meta(key, value) VALUES ('git_head', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [&head],
    )?;
    tx.commit()?;
    Ok(added)
}

fn index_commit(
    conn: &Connection,
    repo: &Repository,
    oid: Oid,
    parser: &mut FileParser,
) -> Result<()> {
    let commit = repo.find_commit(oid)?;
    let sha = oid.to_string();
    let id = commit_id(&sha);
    let message = commit.message().unwrap_or_default().trim().to_string();
    let reverts = message.find("This reverts commit ").map(|i| {
        message[i + 20..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect::<String>()
    });
    let extra = serde_json::json!({
        "author": commit.author().name().unwrap_or_default(),
        "time": commit.time().seconds(),
        "parents": commit.parent_ids().map(|p| p.to_string()).collect::<Vec<_>>(),
        "reverts": reverts,
    });
    let node = Node {
        id: id.clone(),
        ntype: "commit".into(),
        origin: "derived".into(),
        kind: Some("commit".into()),
        title: Some(commit.summary().ok().flatten().unwrap_or("").to_string()),
        qname: Some(sha[..7.min(sha.len())].to_string()),
        doc: Some(message.chars().take(2000).collect()),
        extra: Some(extra.to_string()),
        ..Default::default()
    };
    let rowid = graph::upsert_node(conn, &node)?;
    myrn_store::fts::index_node(
        conn,
        rowid,
        node.title.as_deref().unwrap_or(""),
        "",
        "",
        node.doc.as_deref().unwrap_or(""),
    )?;
    if commit.parent_count() > 1 {
        return Ok(()); // merges: the changes are attributed to the merged commits
    }
    let tree = commit.tree()?;
    let parent_tree = match commit.parent(0) {
        Ok(p) => Some(p.tree()?),
        Err(_) => None,
    };
    let mut opts = DiffOptions::new();
    opts.context_lines(0);
    let mut diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut opts))?;
    diff.find_similar(Some(DiffFindOptions::new().renames(true)))?;

    for (idx, delta) in diff.deltas().enumerate() {
        let new_path = delta
            .new_file()
            .path()
            .map(|p| p.to_string_lossy().replace('\\', "/"));
        let old_path = delta
            .old_file()
            .path()
            .map(|p| p.to_string_lossy().replace('\\', "/"));
        let (change, path) = match delta.status() {
            Delta::Added => ("added", new_path.clone()),
            Delta::Deleted => ("deleted", old_path.clone()),
            Delta::Modified => ("modified", new_path.clone()),
            Delta::Renamed => ("renamed", new_path.clone()),
            Delta::Copied => ("added", new_path.clone()),
            _ => continue,
        };
        let Some(path) = path else { continue };
        conn.execute(
            "INSERT OR IGNORE INTO commit_files (sha, path, change, old_path) VALUES (?1, ?2, ?3, ?4)",
            params![sha, path, change, if change == "renamed" { old_path } else { None }],
        )?;
        if change == "deleted" {
            continue;
        }
        let Some(lang) = Lang::from_path(&path) else {
            continue;
        };
        let Ok(Some(patch)) = git2::Patch::from_diff(&diff, idx) else {
            continue;
        };
        let mut ranges = Vec::new();
        for h in 0..patch.num_hunks() {
            let (hunk, _) = patch.hunk(h)?;
            let start = hunk.new_start() as i64;
            let end = start + (hunk.new_lines() as i64 - 1).max(0);
            ranges.push((start, end));
        }
        if ranges.is_empty() {
            continue;
        }
        let Ok(blob) = repo.find_blob(delta.new_file().id()) else {
            continue;
        };
        let Ok(parsed) = parser.parse(lang, &path, blob.content()) else {
            continue;
        };
        let hit: Vec<usize> = (0..parsed.defs.len())
            .filter(|&i| {
                let d = &parsed.defs[i];
                ranges
                    .iter()
                    .any(|&(s, e)| s <= d.end_line && e >= d.start_line)
            })
            .collect();
        // Keep the innermost definitions: drop any that contain another touched one.
        for &i in &hit {
            let d = &parsed.defs[i];
            let contains_other = hit.iter().any(|&j| {
                let o = &parsed.defs[j];
                j != i && d.start_line <= o.start_line && d.end_line >= o.end_line
            });
            if contains_other {
                continue;
            }
            let dst = if d.is_test {
                test_id(&path, &d.qname)
            } else {
                symbol_id(&path, &d.qname)
            };
            graph::insert_edge(conn, &id, "touches", &dst, None)?;
        }
    }
    record_adds(conn, repo, oid, parser)?;
    Ok(())
}

/// Commits indexed before `commit_adds` existed: fill it once.
fn backfill_adds(conn: &Connection, repo: &Repository, parser: &mut FileParser) -> Result<()> {
    const KEY: &str = "commit_adds_filled";
    if conn
        .query_row("SELECT 1 FROM meta WHERE key = ?1", [KEY], |_| Ok(()))
        .is_ok()
    {
        return Ok(());
    }
    let shas: Vec<String> = {
        let mut stmt = conn.prepare("SELECT DISTINCT sha FROM commit_files")?;
        let rows = stmt.query_map([], |r| r.get(0))?;
        rows.collect::<rusqlite::Result<_>>()?
    };
    let tx = conn.unchecked_transaction()?;
    for sha in shas {
        if let Ok(oid) = Oid::from_str(&sha) {
            record_adds(&tx, repo, oid, parser)?;
        }
    }
    tx.execute(
        "INSERT OR REPLACE INTO meta(key, value) VALUES (?1, '1')",
        [KEY],
    )?;
    tx.commit()?;
    Ok(())
}

/// Most files a commit may change and still count as adding code: larger ones
/// are moves, renames and sweeping edits.
const ADDS_MAX_FILES: usize = 12;

/// Record the files this commit added, or added a whole new definition to.
fn record_adds(conn: &Connection, repo: &Repository, oid: Oid, parser: &mut FileParser) -> Result<()> {
    let Ok(commit) = repo.find_commit(oid) else {
        return Ok(());
    };
    if commit.parent_count() > 1 {
        return Ok(());
    }
    let tree = commit.tree()?;
    let parent_tree = match commit.parent(0) {
        Ok(p) => Some(p.tree()?),
        Err(_) => return Ok(()), // the first commit adds everything
    };
    let mut opts = DiffOptions::new();
    opts.context_lines(0);
    let mut diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut opts))?;
    diff.find_similar(Some(DiffFindOptions::new().renames(true)))?;
    if diff.deltas().len() > ADDS_MAX_FILES {
        return Ok(());
    }
    let sha = oid.to_string();
    for (idx, delta) in diff.deltas().enumerate() {
        let Some(path) = delta
            .new_file()
            .path()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
        else {
            continue;
        };
        let adds = match delta.status() {
            Delta::Added | Delta::Copied => true,
            Delta::Modified => {
                let Some(lang) = Lang::from_path(&path) else {
                    continue;
                };
                let Ok(Some(patch)) = git2::Patch::from_diff(&diff, idx) else {
                    continue;
                };
                // Hunks that only insert lines: a definition wholly inside one is new.
                let mut inserted = Vec::new();
                for h in 0..patch.num_hunks() {
                    let (hunk, _) = patch.hunk(h)?;
                    if hunk.old_lines() == 0 && hunk.new_lines() > 0 {
                        let start = hunk.new_start() as i64;
                        inserted.push((start, start + hunk.new_lines() as i64 - 1));
                    }
                }
                if inserted.is_empty() {
                    continue;
                }
                let Ok(blob) = repo.find_blob(delta.new_file().id()) else {
                    continue;
                };
                let Ok(parsed) = parser.parse(lang, &path, blob.content()) else {
                    continue;
                };
                parsed.defs.iter().any(|d| {
                    !d.is_test
                        && inserted
                            .iter()
                            .any(|&(s, e)| s <= d.start_line && e >= d.end_line)
                })
            }
            _ => false,
        };
        if adds {
            conn.execute(
                "INSERT OR IGNORE INTO commit_adds (sha, path) VALUES (?1, ?2)",
                params![sha, path],
            )?;
        }
    }
    Ok(())
}
