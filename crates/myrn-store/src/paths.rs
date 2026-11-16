//! Locations: the repo's `.myrn/` directory and the per-user data directory.
//!
//! Private data lives in the platform's per-user data directory:
//! `~/.local/share/myrn` (Linux), `~/Library/Application Support/myrn` (macOS),
//! `%LOCALAPPDATA%\myrn` (Windows). `MYRN_HOME` overrides it.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

pub const REPO_DIR: &str = ".myrn";
pub const KNOWLEDGE_KINDS: [&str; 5] = ["decisions", "bugs", "workflows", "facts", "summaries"];

/// Root of Myrn's private per-user data.
pub fn data_home() -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("MYRN_HOME") {
        return Ok(PathBuf::from(p));
    }
    let base =
        directories::BaseDirs::new().ok_or_else(|| anyhow!("cannot find a home directory"))?;
    Ok(base.data_local_dir().join("myrn"))
}

/// Where every piece of one project's knowledge lives.
#[derive(Debug, Clone)]
pub struct Layout {
    pub repo_root: PathBuf,
    /// `<repo>/.myrn`
    pub repo_dir: PathBuf,
    /// `<data home>/projects/<project id>`
    pub data_dir: PathBuf,
    /// The code graph and index. Linked worktrees get their own, because their files differ.
    pub db_path: PathBuf,
}

impl Layout {
    pub fn new(
        repo_root: &Path,
        data_home: &Path,
        project_id: &str,
        worktree: Option<&str>,
    ) -> Layout {
        let data_dir = data_home.join("projects").join(project_id);
        let db_path = match worktree {
            Some(key) => data_dir.join("worktrees").join(key).join("memory.db"),
            None => data_dir.join("memory.db"),
        };
        Layout {
            repo_root: repo_root.to_path_buf(),
            repo_dir: repo_root.join(REPO_DIR),
            data_dir,
            db_path,
        }
    }

    pub fn config_path(&self) -> PathBuf {
        self.repo_dir.join("project.toml")
    }
    pub fn ignore_path(&self) -> PathBuf {
        self.repo_dir.join("ignore")
    }
    pub fn knowledge_dir(&self) -> PathBuf {
        self.repo_dir.join("knowledge")
    }
    /// Consent and connected agents; survives `rebuild`.
    pub fn state_path(&self) -> PathBuf {
        self.data_dir.join("state.json")
    }
    pub fn notes_dir(&self) -> PathBuf {
        self.data_dir.join("notes")
    }
    pub fn episodes_dir(&self) -> PathBuf {
        self.data_dir.join("episodes")
    }
    pub fn evidence_dir(&self) -> PathBuf {
        self.data_dir.join("evidence")
    }

    /// Create the shared and private directory trees.
    pub fn create_dirs(&self) -> Result<()> {
        for kind in KNOWLEDGE_KINDS {
            std::fs::create_dir_all(self.knowledge_dir().join(kind))?;
            std::fs::create_dir_all(self.notes_dir().join(kind))?;
        }
        std::fs::create_dir_all(self.episodes_dir())?;
        std::fs::create_dir_all(self.evidence_dir())?;
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

/// Repo-relative path with forward slashes, so notes and ids match on every OS.
pub fn repo_rel(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    Some(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_paths_use_forward_slashes() {
        let root = Path::new("/r");
        assert_eq!(
            repo_rel(root, &root.join("a").join("b.rs")).unwrap(),
            "a/b.rs"
        );
    }

    #[test]
    fn worktrees_get_their_own_db() {
        let l = Layout::new(Path::new("/r"), Path::new("/h"), "abc", Some("wt1"));
        assert!(l.db_path.ends_with("projects/abc/worktrees/wt1/memory.db"));
        let l = Layout::new(Path::new("/r"), Path::new("/h"), "abc", None);
        assert!(l.db_path.ends_with("projects/abc/memory.db"));
    }
}
