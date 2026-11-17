//! Benchmark repositories (`bench/repos/*.toml`) and tasks
//! (`bench/tasks/<repo>/<kind>/*.yaml`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
    /// Full commit id every run starts from.
    pub commit: String,
    pub language: String,
    #[serde(default)]
    pub size: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Bugfix,
    Feature,
    Question,
    Replay,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Bugfix => "bugfix",
            Kind::Feature => "feature",
            Kind::Question => "question",
            Kind::Replay => "replay",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub repo: String,
    pub kind: Kind,
    /// What the agent is told. Says the symptom, never the location.
    pub prompt: String,
    /// Changes applied before the agent starts, folded into the HEAD commit
    /// so `git status` and `git diff` show a clean tree.
    #[serde(default)]
    pub setup: Option<Patch>,
    /// Reference solution, used only by `validate`. Bug fixes default to
    /// reverting the setup patch.
    #[serde(default)]
    pub solution: Option<Patch>,
    #[serde(default)]
    pub check: Option<Check>,
    /// Questions: every string must appear in the final answer (any case).
    #[serde(default)]
    pub answer_contains: Vec<String>,
    /// Files where the correct edit lands; the first edit to one of them ends
    /// the "exploration before first correct edit" count.
    #[serde(default)]
    pub touches: Vec<String>,
    /// Wall-clock limit for the agent session.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    900
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Patch {
    /// A unified diff, applied with `git apply`.
    pub patch: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    /// Hidden test files written after the session ends.
    #[serde(default)]
    pub files: BTreeMap<String, String>,
    /// Shell command; exit status 0 means success.
    pub run: String,
    #[serde(default = "default_check_timeout")]
    pub timeout_secs: u64,
}

fn default_check_timeout() -> u64 {
    900
}

impl Task {
    pub fn validate(&self) -> Result<()> {
        match self.kind {
            Kind::Question => {
                if self.answer_contains.is_empty() {
                    bail!("{}: a question needs answer_contains", self.id);
                }
            }
            Kind::Bugfix | Kind::Replay => {
                if self.setup.is_none() || self.check.is_none() {
                    bail!("{}: a bug fix needs setup and check", self.id);
                }
            }
            Kind::Feature => {
                if self.check.is_none() || self.solution.is_none() {
                    bail!("{}: a feature needs check and solution", self.id);
                }
            }
        }
        for f in self.check.iter().flat_map(|c| c.files.keys()) {
            if f.starts_with('/') || f.split('/').any(|p| p == "..") {
                bail!(
                    "{}: check file {f} must be a relative path inside the repo",
                    self.id
                );
            }
        }
        Ok(())
    }
}

pub fn load_repos(dir: &Path) -> Result<BTreeMap<String, Repo>> {
    let mut out = BTreeMap::new();
    for p in files_with_ext(dir, "toml")? {
        let r: Repo = toml_from(&p)?;
        out.insert(r.name.clone(), r);
    }
    Ok(out)
}

pub fn load_tasks(dir: &Path) -> Result<Vec<Task>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).with_context(|| format!("reading {}", d.display()))? {
            let p = e?.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yaml" || x == "yml") {
                let text = std::fs::read_to_string(&p)?;
                let t: Task = serde_yaml::from_str(&text)
                    .with_context(|| format!("parsing {}", p.display()))?;
                t.validate()?;
                out.push(t);
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    let mut seen = std::collections::HashSet::new();
    for t in &out {
        if !seen.insert(&t.id) {
            bail!("duplicate task id {}", t.id);
        }
    }
    Ok(out)
}

fn files_with_ext(dir: &Path, ext: &str) -> Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == ext))
        .collect();
    v.sort();
    Ok(v)
}

fn toml_from<T: serde::de::DeserializeOwned>(p: &Path) -> Result<T> {
    let text = std::fs::read_to_string(p)?;
    toml::from_str(&text).with_context(|| format!("parsing {}", p.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shipped_task_set_parses_and_is_consistent() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let repos = load_repos(&root.join("repos")).unwrap();
        let tasks = load_tasks(&root.join("tasks")).unwrap();
        assert!(!tasks.is_empty());
        for t in &tasks {
            assert!(
                repos.contains_key(&t.repo),
                "{}: unknown repo {}",
                t.id,
                t.repo
            );
            assert!(
                t.id.starts_with(&format!("{}/{}/", t.repo, t.kind.as_str())),
                "{}",
                t.id
            );
        }
    }
}
