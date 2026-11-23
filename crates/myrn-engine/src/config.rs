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
    #[serde(default)]
    pub records: RecordsConfig,
    #[serde(default)]
    pub summarize: SummarizeConfig,
    #[serde(default)]
    pub ranking: RankingConfig,
    #[serde(default)]
    pub embeddings: EmbeddingsConfig,
}

/// How candidates are ranked (`[ranking]`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RankingConfig {
    /// `learned`: relevance and state, adjusted by how useful each item was
    /// when injected before, and records' age. `fixed`: relevance and state only.
    pub mode: String,
    /// How far past use moves an item (0 = not at all). The factor stays within 0.5..1.5.
    pub usefulness_weight: f64,
    /// How much an old record ranks down (0 = not at all; at most this share).
    pub freshness_weight: f64,
    /// Age in days at which a record has lost half of `freshness_weight`.
    pub freshness_half_life_days: f64,
}

impl Default for RankingConfig {
    fn default() -> Self {
        RankingConfig {
            mode: "learned".into(),
            usefulness_weight: 0.5,
            freshness_weight: 0.2,
            freshness_half_life_days: 180.0,
        }
    }
}

/// Semantic search (`[embeddings]`). It is turned on per machine with
/// `myrn embeddings enable`, which downloads the model; these are its knobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EmbeddingsConfig {
    /// Lowest cosine similarity for a vector match to seed retrieval.
    pub min_similarity: f64,
    /// A match must also beat the task's 20th-best similarity by this much:
    /// static embeddings have a high background level, and only matches that
    /// stand out from it are meaningful.
    pub margin: f64,
    /// Most vector matches per task.
    pub top_k: usize,
    /// The helper process exits after this many idle seconds.
    pub idle_secs: u64,
}

impl Default for EmbeddingsConfig {
    fn default() -> Self {
        EmbeddingsConfig {
            min_similarity: 0.40,
            margin: 0.08,
            top_k: 5,
            idle_secs: 600,
        }
    }
}

/// `myrn summarize`: the model command that writes the project map.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SummarizeConfig {
    /// Program and arguments. Gets the prompt on stdin and answers on stdout
    /// (plain text, or Claude Code's `--output-format json`).
    pub command: Vec<String>,
    /// Model calls run at the same time.
    pub jobs: usize,
}

impl Default for SummarizeConfig {
    fn default() -> Self {
        SummarizeConfig {
            command: [
                "claude",
                "-p",
                "--model",
                "haiku",
                "--tools",
                "",
                "--no-session-persistence",
                "--output-format",
                "json",
            ]
            .map(String::from)
            .to_vec(),
            jobs: 16,
        }
    }
}

/// Learned records: asking for them, and recalling them on failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordsConfig {
    /// When a session edited code, ask the agent at the end of its turn to
    /// call `record` for what it learned (needs recording consent).
    pub ask_at_session_end: bool,
    /// When a tool fails, inject the known bug, cause and fix if the error
    /// matches a recorded one.
    pub recall_on_failure: bool,
    /// Token budget for a recall injected on failure.
    pub failure_budget: usize,
}

impl Default for RecordsConfig {
    fn default() -> Self {
        RecordsConfig {
            ask_at_session_end: true,
            recall_on_failure: true,
            failure_budget: 500,
        }
    }
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
    /// Inject context automatically from hooks: the project brief at session
    /// start and a pack of relevant code per prompt (nothing when nothing is
    /// relevant, and nothing already sent in the session). Off: hooks only capture.
    pub inject: bool,
    pub session_budget: usize,
    pub prompt_budget: usize,
    /// Most items in a prompt pack. Fewer is better than padding.
    pub max_items: usize,
    /// Include a top hit's source when its body has at most this many lines.
    pub code_lines: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        ContextConfig {
            inject: true,
            session_budget: 800,
            prompt_budget: 1500,
            max_items: 5,
            code_lines: 40,
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
             # Inject context from hooks automatically: the project brief at session\n\
             # start, and relevant code per prompt (never what the session already has).\n\
             inject = {}\n\
             # Token budgets for injected context.\n\
             session_budget = {}\n\
             prompt_budget = {}\n\
             # Most items per prompt pack, and the longest function body shown as code.\n\
             max_items = {}\n\
             code_lines = {}\n\
             \n\
             [records]\n\
             # At the end of a turn that edited code, ask the agent to record what it learned.\n\
             ask_at_session_end = {}\n\
             # On a failing tool, inject a matching known bug with its cause and fix.\n\
             recall_on_failure = {}\n\
             failure_budget = {}\n\
             \n\
             [summarize]\n\
             # Model command for `myrn summarize` (prompt on stdin, answer on stdout).\n\
             command = {}\n\
             jobs = {}\n\
             \n\
             [ranking]\n\
             # learned: also by how useful each item was when injected before, and\n\
             # records' age. fixed: relevance and record state only.\n\
             mode = \"{}\"\n",
            d.index.max_file_bytes,
            d.index.history_depth,
            d.context.inject,
            d.context.session_budget,
            d.context.prompt_budget,
            d.context.max_items,
            d.context.code_lines,
            d.records.ask_at_session_end,
            d.records.recall_on_failure,
            d.records.failure_budget,
            toml::Value::from(d.summarize.command.clone()),
            d.summarize.jobs,
            d.ranking.mode
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
