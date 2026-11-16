//! `.myrn/project.toml` and `.myrn/ignore`.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub index: IndexConfig,
    #[serde(default)]
    pub context: ContextConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Root commit of the repository. Cached here so it is not recomputed on every call.
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IndexConfig {
    /// Files larger than this are not parsed.
    pub max_file_bytes: u64,
    /// How many commits of history to index.
    pub history_depth: usize,
}

impl Default for IndexConfig {
    fn default() -> Self {
        IndexConfig {
            max_file_bytes: 1_000_000,
            history_depth: 500,
        }
    }
}

/// Token budgets. Starting targets, to be tuned against the benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextConfig {
    pub session_budget: usize,
    pub prompt_budget: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        ContextConfig {
            session_budget: 500,
            prompt_budget: 1500,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        if !path.exists() {
            return Ok(Config::default());
        }
        let text = std::fs::read_to_string(path)?;
        toml::from_str(&text).with_context(|| format!("reading {}", path.display()))
    }

    /// The file `myrn init` writes.
    pub fn initial_toml(project_id: &str) -> String {
        let d = Config::default();
        format!(
            "# Myrn project configuration. Commit this file.\n\
             \n\
             [project]\n\
             # Root commit of the repository; identifies this project's knowledge.\n\
             id = \"{project_id}\"\n\
             \n\
             [index]\n\
             max_file_bytes = {}\n\
             history_depth = {}\n\
             \n\
             [context]\n\
             # Token budgets for injected context.\n\
             session_budget = {}\n\
             prompt_budget = {}\n",
            d.index.max_file_bytes,
            d.index.history_depth,
            d.context.session_budget,
            d.context.prompt_budget
        )
    }
}

pub const INITIAL_IGNORE: &str = "\
# Paths Myrn never indexes or records, in .gitignore syntax.
# Files ignored by Git are already skipped.
#
# secrets/
# *.generated.ts
";
