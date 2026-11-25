//! "How code gets added here", from git history. For a request to add code,
//! the files that past commits adding code to the same place changed together
//! (the function's file, its stub, its tests, the docs index, the package
//! export). On the benchmark repositories this named 90% of the files the
//! reference solutions touch; it replaces the exploration agents otherwise do
//! to find where things go.

use std::collections::HashMap;

use anyhow::Result;
use myrn_store::db::Db;
use rusqlite::params;

use super::seed;

/// Most files a commit may change and still count (also enforced when indexing).
const MAX_FILES: usize = 12;
/// A file is listed when at least this share of the commits changed it...
const MIN_SHARE: f64 = 0.3;
/// ...and at least this many of them.
const MIN_COMMITS: usize = 2;
/// Most files listed.
const MAX_LISTED: usize = 8;
/// Fewer qualifying commits than this: say nothing.
const MIN_EXAMPLES: usize = 2;

/// The checklist text, or `None` when history says nothing useful.
pub fn addition_recipe(db: &Db, task: &str) -> Result<Option<String>> {
    let c = &db.conn;
    // Names in backticks that are not in the code yet: what the task adds.
    let mut new_names = Vec::new();
    let mut existing_paths = Vec::new();
    let mut ambiguous = false;
    for t in seed::backticked(task) {
        let hits = seed::find(db, &t)?;
        if hits.is_empty() {
            new_names.push(t.to_lowercase());
        } else {
            // Only a name that picks out one file (`Glob` is both a globset
            // type and a command-line flag in ripgrep): otherwise the
            // checklist would describe the wrong place.
            let mut paths: Vec<String> = hits
                .iter()
                .filter_map(|n| n.path.clone())
                .filter(|p| !seed::is_stub(p))
                .collect();
            paths.sort();
            paths.dedup();
            if paths.len() == 1 {
                existing_paths.push(paths.remove(0));
            } else if paths.len() > 1 {
                ambiguous = true;
            }
        }
    }
    let Some(target) = target_path(db, task, &existing_paths, ambiguous)? else {
        return Ok(None);
    };
    let file_exists = c
        .query_row(
            "SELECT 1 FROM nodes WHERE type = 'file' AND path = ?1",
            [&target],
            |_| Ok(()),
        )
        .is_ok();
    let dir = target.rsplit_once('/').map_or("", |(d, _)| d).to_string();

    // Commits that added code to the target file, or a new file beside it.
    let candidates: Vec<(String, String)> = if file_exists {
        let mut s = c.prepare("SELECT sha, path FROM commit_adds WHERE path = ?1")?;
        let rows = s.query_map([&target], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect::<rusqlite::Result<_>>()?
    } else {
        let mut s = c.prepare(
            "SELECT a.sha, a.path FROM commit_adds a JOIN commit_files f
               ON f.sha = a.sha AND f.path = a.path AND f.change = 'added'",
        )?;
        let rows = s.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
            .into_iter()
            .filter(|(_, p)| {
                p.rsplit_once('/').map_or("", |(d, _)| d) == dir && !is_test_like(p) && !p.ends_with("/index.ts")
            })
            .collect()
    };
    let mut seen = std::collections::HashSet::new();
    let mut groups: Vec<std::collections::HashSet<String>> = Vec::new();
    for (sha, added) in candidates {
        if !seen.insert(sha.clone()) {
            continue;
        }
        let title: String = c
            .query_row(
                "SELECT COALESCE(title, '') FROM nodes WHERE id = ?1",
                [myrn_model::id::commit_id(&sha)],
                |r| r.get(0),
            )
            .unwrap_or_default();
        let title = title.to_lowercase();
        // Never learn from the commit that added this very code.
        if new_names.iter().any(|n| title.contains(n.as_str())) {
            continue;
        }
        let mut s = c.prepare_cached("SELECT path FROM commit_files WHERE sha = ?1")?;
        let files: Vec<String> = s
            .query_map(params![sha], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        if files.len() > MAX_FILES {
            continue;
        }
        let stem = stem_of(&added);
        groups.push(
            files
                .into_iter()
                .map(|p| if file_exists { p } else { generalise(&p, &stem) })
                .collect(),
        );
    }
    if groups.len() < MIN_EXAMPLES {
        return Ok(None);
    }
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for g in &groups {
        for p in g {
            *counts.entry(p.as_str()).or_default() += 1;
        }
    }
    let n = groups.len();
    let mut listed: Vec<(&str, usize)> = counts
        .into_iter()
        .filter(|&(_, k)| k >= MIN_COMMITS && k as f64 / n as f64 >= MIN_SHARE)
        .collect();
    listed.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    listed.truncate(MAX_LISTED);
    if listed.len() < 2 {
        return Ok(None);
    }
    let name = new_names.first().map(|n| {
        // Keep the task's spelling (`takeEvery`, not `takeevery`).
        seed::backticked(task)
            .into_iter()
            .find(|t| t.to_lowercase() == *n)
            .unwrap_or_else(|| n.clone())
    });
    let what = if file_exists {
        format!("added a function to {target}")
    } else {
        format!("added a new file next to {target}")
    };
    let mut out = format!("How code is added here: {n} past commits that {what} changed together:\n");
    for (p, k) in listed {
        let shown = match &name {
            Some(nm) => p.replace("<name>", nm),
            None => p.to_string(),
        };
        out.push_str(&format!("- {shown}  ({k} of {n})\n"));
    }
    Ok(Some(out))
}

/// The file the new code goes in: a path the task names, else the file of a
/// thing it names, else where past additions most often went.
fn target_path(db: &Db, task: &str, existing: &[String], ambiguous: bool) -> Result<Option<String>> {
    for word in task.split(|c: char| c.is_whitespace() || matches!(c, '`' | '"' | '\'' | ',' | '(' | ')')) {
        let w = word.trim_end_matches(['.', ':', ';']);
        if w.contains('/') && w.rsplit('/').next().is_some_and(|f| f.contains('.')) {
            return Ok(Some(w.to_string()));
        }
    }
    if let Some(p) = existing.first() {
        return Ok(Some(p.clone()));
    }
    if ambiguous {
        return Ok(None);
    }
    // Where past additions of code (not tests) most often went, if one file
    // clearly dominates (more-itertools: more.py).
    let mut stmt = db.conn.prepare("SELECT sha, path FROM commit_adds")?;
    let rows: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;
    let mut by_path: HashMap<&str, usize> = HashMap::new();
    let mut shas = std::collections::HashSet::new();
    for (sha, p) in &rows {
        if !is_test_like(p) && seed_lang(p) && !seed::is_stub(p) {
            *by_path.entry(p.as_str()).or_default() += 1;
            shas.insert(sha.as_str());
        }
    }
    let best = by_path.into_iter().max_by_key(|&(p, k)| (k, std::cmp::Reverse(p)));
    Ok(best
        .filter(|&(_, k)| k >= 5 && k as f64 >= 0.4 * shas.len() as f64)
        .map(|(p, _)| p.to_string()))
}

/// A source file myrn indexes (not docs, config or data).
fn seed_lang(p: &str) -> bool {
    crate::indexer::lang::Lang::from_path(p).is_some()
}

fn is_test_like(p: &str) -> bool {
    let f = p.rsplit('/').next().unwrap_or(p);
    p.split('/').any(|d| matches!(d, "test" | "tests" | "__tests__"))
        || f.starts_with("test_")
        || f.contains(".test.")
        || f.contains(".test-d.")
        || f.contains(".test-prop.")
        || f.contains(".bench.")
        || f.contains("_test.")
}

fn stem_of(p: &str) -> String {
    let f = p.rsplit('/').next().unwrap_or(p);
    f.split('.').next().unwrap_or(f).to_string()
}

/// `src/chunk.test.ts` with stem `chunk` -> `src/<name>.test.ts`.
fn generalise(p: &str, stem: &str) -> String {
    let (dir, file) = p.rsplit_once('/').map_or(("", p), |(d, f)| (d, f));
    if file == stem || file.starts_with(&format!("{stem}.")) {
        let rest = &file[stem.len()..];
        if dir.is_empty() {
            format!("<name>{rest}")
        } else {
            format!("{dir}/<name>{rest}")
        }
    } else {
        p.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sibling_paths_are_generalised() {
        assert_eq!(generalise("packages/remeda/src/chunk.test.ts", "chunk"), "packages/remeda/src/<name>.test.ts");
        assert_eq!(generalise("packages/remeda/src/index.ts", "chunk"), "packages/remeda/src/index.ts");
        assert_eq!(stem_of("packages/remeda/src/chunk.test-d.ts"), "chunk");
        assert!(is_test_like("tests/test_more.py"));
        assert!(!is_test_like("more_itertools/more.py"));
    }
}
