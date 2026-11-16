//! Walk the repository, honouring `.gitignore` and `.myrn/ignore`.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::WalkBuilder;

use super::lang::Lang;

#[derive(Debug, Clone)]
pub struct FileStat {
    /// Repo-relative, forward slashes.
    pub path: String,
    pub abs: PathBuf,
    pub lang: Lang,
    pub size: u64,
    pub mtime: i64,
}

pub fn load_ignore(root: &Path, ignore_file: &Path) -> Gitignore {
    let mut b = GitignoreBuilder::new(root);
    if ignore_file.exists() {
        let _ = b.add(ignore_file);
    }
    b.build().unwrap_or_else(|_| Gitignore::empty())
}

/// Every source file in a supported language, sorted by path.
pub fn walk(root: &Path, ignore_file: &Path, max_bytes: u64) -> Result<Vec<FileStat>> {
    let extra = Arc::new(load_ignore(root, ignore_file));
    let mut builder = WalkBuilder::new(root);
    builder
        .standard_filters(true)
        .require_git(false)
        .follow_links(false);
    {
        let extra = extra.clone();
        builder.filter_entry(move |e| {
            let is_dir = e.file_type().is_some_and(|t| t.is_dir());
            !extra
                .matched_path_or_any_parents(e.path(), is_dir)
                .is_ignore()
        });
    }
    let mut out = Vec::new();
    for entry in builder.build() {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let Some(path) = myrn_store::paths::repo_rel(root, entry.path()) else {
            continue;
        };
        let Some(lang) = Lang::from_path(&path) else {
            continue;
        };
        let Ok(meta) = entry.metadata() else { continue };
        if meta.len() > max_bytes {
            continue;
        }
        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos() as i64)
            .unwrap_or(0);
        out.push(FileStat {
            path,
            abs: entry.path().to_path_buf(),
            lang,
            size: meta.len(),
            mtime,
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}
