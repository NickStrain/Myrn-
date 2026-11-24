//! The single internal engine API used by adapters, MCP tools and the CLI.
//! Nothing else touches the stores.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{anyhow, bail, Context, Result};
use git2::Repository;
use myrn_model::Visibility;
use myrn_store::graph::{self, Dir, Node};
use myrn_store::notes::{self, Note};
use myrn_store::{paths, sync, Db, Layout};
use serde::Serialize;

use crate::capture::{self, episode::Episode};
use crate::config::{Config, INITIAL_IGNORE};
use crate::indexer::{self, git, lang::Lang, IndexReport};
use crate::pack::cards::{FileCard, ImpactReport, Known, Link, SymbolCard};
use crate::pack::{budget, log, render};
use crate::retrieve::{rank, seed, walks};

pub struct Engine {
    pub layout: Layout,
    pub config: Config,
    pub project_id: String,
    /// True when the project id is a stand-in because the repository has no commits yet.
    pub provisional_id: bool,
    pub(crate) db: Db,
    last_refresh: Option<Instant>,
}

/// Outcome of looking a name up: one match, several, or none.
#[derive(Debug, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Lookup<T> {
    Found(T),
    Ambiguous { candidates: Vec<Link> },
    NotFound { query: String },
}

impl<T> Lookup<T> {
    pub fn render(&self, f: impl Fn(&T) -> String) -> String {
        match self {
            Lookup::Found(t) => f(t),
            Lookup::Ambiguous { candidates } => {
                let mut s = String::from(
                    "Several matches; ask again with a qualified name or path:line.\n",
                );
                for c in candidates {
                    s.push_str(&format!("  {}  {}\n", c.name, c.location));
                }
                s
            }
            Lookup::NotFound { query } => format!("Nothing in the code graph matches `{query}`.\n"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Explained {
    Symbol(SymbolCard),
    File(FileCard),
}

impl Explained {
    pub fn render(&self) -> String {
        match self {
            Explained::Symbol(c) => c.render(12),
            Explained::File(c) => c.render(25),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Pack {
    pub text: String,
    pub items: Vec<String>,
    pub tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct HistoryEntry {
    pub sha: String,
    pub summary: String,
    pub author: String,
    pub time: i64,
    pub change: Option<String>,
    #[serde(skip)]
    parents: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub root: String,
    pub project_id: String,
    pub provisional_id: bool,
    pub db_path: String,
    pub files_by_language: BTreeMap<String, i64>,
    pub files_with_errors: i64,
    pub symbols: i64,
    pub tests: i64,
    pub modules: i64,
    pub commits: i64,
    pub edges: BTreeMap<String, i64>,
    pub calls_inferred: i64,
    pub records_by_state: Vec<(String, i64)>,
    pub notes: sync::NotesSummary,
    /// Records that disagree, at least one shared: for a person to resolve.
    pub conflicts: Vec<sync::Conflict>,
    pub connected_agents: Vec<String>,
    pub last_refresh: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Ok,
    Warn,
    Fail,
}

#[derive(Debug, Serialize)]
pub struct Check {
    pub name: String,
    pub level: Level,
    pub detail: String,
}

fn check(name: &str, level: Level, detail: impl Into<String>) -> Check {
    Check {
        name: name.into(),
        level,
        detail: detail.into(),
    }
}

/// Where the repository is and who it is.
struct Located {
    root: PathBuf,
    project_id: String,
    provisional: bool,
    worktree: Option<String>,
}

fn locate(cwd: &Path, config: Option<&Config>) -> Result<Located> {
    let repo = Repository::discover(cwd).map_err(|_| {
        anyhow!(
            "{} is not inside a Git repository; myrn works per repository",
            cwd.display()
        )
    })?;
    let root = repo
        .workdir()
        .ok_or_else(|| anyhow!("bare repositories are not supported"))?
        .to_path_buf();
    let root = root.canonicalize().unwrap_or(root);
    let worktree = repo
        .is_worktree()
        .then(|| indexer::content_hash(root.to_string_lossy().as_bytes())[..12].to_string());
    let cached = config.and_then(|c| c.project.id.clone());
    let (project_id, provisional) = match cached.or_else(|| git::root_commit(&repo)) {
        Some(id) => (id, false),
        None => (
            format!(
                "uncommitted-{}",
                &indexer::content_hash(root.to_string_lossy().as_bytes())[..12]
            ),
            true,
        ),
    };
    Ok(Located {
        root,
        project_id,
        provisional,
        worktree,
    })
}

impl Engine {
    /// Index the repository and create the local store. Safe to run again.
    pub fn init(cwd: &Path, data_home: Option<&Path>) -> Result<(Engine, IndexReport)> {
        let first = locate(cwd, None)?;
        let repo_dir = first.root.join(paths::REPO_DIR);
        let existing = Config::load(&repo_dir.join("project.toml"))?;
        let loc = locate(cwd, Some(&existing))?;
        std::fs::create_dir_all(&repo_dir)?;
        let config_path = repo_dir.join("project.toml");
        if !config_path.exists() && !loc.provisional {
            std::fs::write(&config_path, Config::initial_toml(&loc.project_id))?;
        }
        let ignore = repo_dir.join("ignore");
        if !ignore.exists() {
            std::fs::write(&ignore, INITIAL_IGNORE)?;
        }
        let mut engine = Self::open_located(loc, data_home)?;
        engine.layout.create_dirs()?;
        for kind in paths::KNOWLEDGE_KINDS {
            let keep = engine.layout.knowledge_dir().join(kind).join(".gitkeep");
            if !keep.exists() {
                std::fs::write(keep, "")?;
            }
        }
        let report = engine.refresh()?;
        Ok((engine, report))
    }

    /// Open an initialised project. Does not refresh the index.
    pub fn open(cwd: &Path, data_home: Option<&Path>) -> Result<Engine> {
        let first = locate(cwd, None)?;
        let config_path = first.root.join(paths::REPO_DIR).join("project.toml");
        let ignore_path = first.root.join(paths::REPO_DIR).join("ignore");
        if !config_path.exists() && !ignore_path.exists() {
            bail!(
                "myrn is not set up in {}; run `myrn init`",
                first.root.display()
            );
        }
        let config = Config::load(&config_path)?;
        let loc = locate(cwd, Some(&config))?;
        Self::open_located(loc, data_home)
    }

    fn open_located(loc: Located, data_home: Option<&Path>) -> Result<Engine> {
        let home = match data_home {
            Some(h) => h.to_path_buf(),
            None => paths::data_home()?,
        };
        let layout = Layout::new(&loc.root, &home, &loc.project_id, loc.worktree.as_deref());
        let config = Config::load(&layout.config_path())?;
        if let Some(parent) = layout.db_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let db = Db::open(&layout.db_path)?;
        Ok(Engine {
            layout,
            config,
            project_id: loc.project_id,
            provisional_id: loc.provisional,
            db,
            last_refresh: None,
        })
    }

    pub fn root(&self) -> &Path {
        &self.layout.repo_root
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    /// Bring the index up to date with the working tree and the notes.
    pub fn refresh(&mut self) -> Result<IndexReport> {
        let started = Instant::now();
        let mut r = indexer::refresh(
            &mut self.db.conn,
            &self.layout.repo_root,
            &self.config,
            &self.layout.ignore_path(),
        )?;
        // Links from notes follow the code, so re-link when the code changed.
        let code_changed = r.parsed > 0 || r.removed > 0;
        let roots = self.note_roots();
        r.notes = sync::sync(&mut self.db.conn, &roots, code_changed)?;
        // Keep vectors in step with the code and summaries (only when on).
        if (code_changed || r.notes.relinked) && self.embeddings_on() {
            if let Err(e) = self.update_vectors() {
                eprintln!("myrn: embeddings not updated: {e:#}");
            }
        }
        r.millis = started.elapsed().as_millis();
        self.last_refresh = Some(Instant::now());
        Ok(r)
    }

    /// Shared knowledge first: a shared note wins over a private copy of the same id.
    pub(crate) fn note_roots(&self) -> Vec<sync::NoteRoot> {
        vec![
            sync::NoteRoot {
                dir: self.layout.knowledge_dir(),
                visibility: Visibility::Shared,
                label: format!("{}/knowledge", paths::REPO_DIR),
            },
            sync::NoteRoot {
                dir: self.layout.notes_dir(),
                visibility: Visibility::Private,
                label: "notes".into(),
            },
        ]
    }

    /// Write a note, then index it (the file is always written first). A note
    /// without an id gets one from its content. Returns the file written.
    pub fn write_note(&mut self, note: &mut Note, visibility: Visibility) -> Result<PathBuf> {
        let root = match visibility {
            Visibility::Shared => self.layout.knowledge_dir(),
            Visibility::Private => self.layout.notes_dir(),
        };
        let c = &self.db.conn;
        let is_note = |id: &str| {
            c.query_row("SELECT 1 FROM notes WHERE id = ?1", [id], |_| Ok(()))
                .is_ok()
        };
        let path = notes::write(&root, note, &is_note)?;
        let roots = self.note_roots();
        sync::sync(&mut self.db.conn, &roots, false)?;
        Ok(path)
    }

    /// Problems found in the notes: malformed headers, dangling links, duplicate ids.
    pub fn note_issues(&self, limit: usize) -> Result<Vec<sync::Issue>> {
        sync::issues(&self.db.conn, limit)
    }

    /// Refresh unless one ran within `secs` seconds (for long-lived servers).
    pub fn refresh_if_older(&mut self, secs: u64) -> Result<Option<IndexReport>> {
        if self
            .last_refresh
            .is_some_and(|t| t.elapsed().as_secs() < secs)
        {
            return Ok(None);
        }
        self.refresh().map(Some)
    }

    /// Recreate the database from the code and the notes. The note files are only read.
    pub fn rebuild(&mut self) -> Result<IndexReport> {
        let path = self.layout.db_path.clone();
        // Not derivable from the files: this clone's own verifications of shared records.
        let kept: Vec<(String, String, Option<String>, Option<i64>)> = self
            .db
            .conn
            .prepare("SELECT note_id, body_hash, reason, t FROM local_verified")
            .and_then(|mut s| {
                s.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
                    .collect()
            })
            .unwrap_or_default();
        self.db = Db::open_in_memory()?; // release the file
        for suffix in ["", "-wal", "-shm"] {
            let p = PathBuf::from(format!("{}{suffix}", path.display()));
            if p.exists() {
                std::fs::remove_file(&p)?;
            }
        }
        self.db = Db::open(&path)?;
        for (id, hash, reason, t) in kept {
            self.db.conn.execute(
                "INSERT OR REPLACE INTO local_verified (note_id, body_hash, reason, t)
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, hash, reason, t],
            )?;
        }
        let report = self.refresh()?;
        self.reindex_episodes()?;
        Ok(report)
    }

    // ------------------------------------------------------------ queries

    /// Resolve a name to nodes, best first.
    pub fn find(&self, query: &str) -> Result<Vec<Node>> {
        seed::find(&self.db, query)
    }

    pub(crate) fn pick(&self, query: &str) -> Result<Lookup<Node>> {
        let hits = self.find(query)?;
        Ok(match hits.len() {
            0 => Lookup::NotFound {
                query: query.to_string(),
            },
            1 => Lookup::Found(hits.into_iter().next().unwrap()),
            _ => {
                let first = &hits[0];
                // A bare one-word name never picks silently between several symbols.
                let qualified = seed::segments(query).len() > 1;
                let exact = qualified
                    && seed::segments(first.qname.as_deref().unwrap_or(""))
                        == seed::segments(query);
                let second_exact =
                    seed::segments(hits[1].qname.as_deref().unwrap_or("")) == seed::segments(query);
                if (exact && !second_exact) || first.ntype == "file" {
                    Lookup::Found(hits.into_iter().next().unwrap())
                } else {
                    Lookup::Ambiguous {
                        candidates: hits.iter().take(10).map(|n| Link::new(n, None)).collect(),
                    }
                }
            }
        })
    }

    fn links(v: Vec<(Node, Option<String>)>) -> Vec<Link> {
        v.into_iter().map(|(n, s)| Link::new(&n, s)).collect()
    }

    fn commits_touching(&self, n: &Node, limit: usize) -> Result<Vec<Link>> {
        let c = &self.db.conn;
        let mut v: Vec<(Node, i64)> = Vec::new();
        if n.ntype == "file" {
            let mut stmt = c.prepare(
                "SELECT n.id FROM commit_files cf JOIN nodes n ON n.id = 'commit:' || cf.sha
                 WHERE cf.path = ?1 ORDER BY json_extract(n.extra, '$.time') DESC LIMIT ?2",
            )?;
            let ids: Vec<String> = stmt
                .query_map(rusqlite::params![n.path, limit as i64], |r| r.get(0))?
                .collect::<rusqlite::Result<_>>()?;
            for id in ids {
                if let Some(c) = graph::node(c, &id)? {
                    v.push((c, 0));
                }
            }
        } else {
            for (c, _) in graph::neighbors(c, &n.id, "touches", Dir::In)? {
                let t = c
                    .extra
                    .as_deref()
                    .and_then(|e| serde_json::from_str::<serde_json::Value>(e).ok())
                    .and_then(|v| v["time"].as_i64())
                    .unwrap_or(0);
                v.push((c, t));
            }
            v.sort_by_key(|c| std::cmp::Reverse(c.1));
            v.truncate(limit);
        }
        Ok(v.into_iter()
            .map(|(c, _)| Link {
                id: c.id.clone(),
                name: c.qname.clone().unwrap_or_default(),
                location: c.title.clone().unwrap_or_default(),
                status: None,
            })
            .collect())
    }

    fn symbol_card(&self, n: Node) -> Result<SymbolCard> {
        let c = &self.db.conn;
        let members = walks::members(c, &n)?;
        let mut tests: Vec<Link> = walks::direct_tests(c, &n.id)?
            .iter()
            .map(|t| Link::new(t, None))
            .collect();
        // A type's tests include those of its methods.
        let mut seen: HashSet<String> = tests.iter().map(|t| t.id.clone()).collect();
        for m in &members {
            for t in walks::direct_tests(c, &m.id)? {
                if seen.insert(t.id.clone()) {
                    tests.push(Link::new(&t, None));
                }
            }
        }
        Ok(SymbolCard {
            known: self.known_about(&n.id)?,
            summary_outdated: self.summary_outdated(&n)?,
            calls: Self::links(graph::neighbors(c, &n.id, "calls", Dir::Out)?),
            called_by: Self::links(
                graph::neighbors(c, &n.id, "calls", Dir::In)?
                    .into_iter()
                    .filter(|(n, _)| n.ntype != "test")
                    .collect(),
            ),
            members: members.iter().map(|m| Link::new(m, None)).collect(),
            tests,
            commits: self.commits_touching(&n, 3)?,
            node: n,
        })
    }

    /// Learned records about a symbol: facts and decisions `about` it, root
    /// causes `located_in` it and the bugs they caused. Superseded ones are left out.
    fn known_about(&self, id: &str) -> Result<Vec<Known>> {
        let c = &self.db.conn;
        let mut out: Vec<Known> = Vec::new();
        let mut push = |n: &Node| {
            if n.origin == "earned"
                && n.status.as_deref() != Some("superseded")
                && !out.iter().any(|k| k.id == n.id)
            {
                out.push(Known {
                    id: n.id.clone(),
                    ntype: n.ntype.clone(),
                    status: n.status.clone().unwrap_or_default(),
                    title: crate::pack::render::safe_text(
                        n,
                        n.title.as_deref().unwrap_or_default(),
                    ),
                    from: crate::pack::render::node_provenance(n),
                });
            }
        };
        for (n, _) in graph::neighbors(c, id, "about", Dir::In)? {
            push(&n);
        }
        for (rc, _) in graph::neighbors(c, id, "located_in", Dir::In)? {
            for (bug, _) in graph::neighbors(c, &rc.id, "caused_by", Dir::In)? {
                push(&bug);
            }
            push(&rc);
        }
        out.sort_by_key(|k| k.status != "verified");
        Ok(out)
    }

    fn file_card(&self, n: Node) -> Result<FileCard> {
        let c = &self.db.conn;
        let symbols = graph::neighbors(c, &n.id, "defined_in", Dir::In)?;
        let mut symbols: Vec<Node> = symbols.into_iter().map(|(n, _)| n).collect();
        symbols.sort_by_key(|s| s.start_line);
        Ok(FileCard {
            symbols: symbols.iter().map(|s| Link::new(s, None)).collect(),
            imports: Self::links(graph::neighbors(c, &n.id, "imports", Dir::Out)?),
            imported_by: Self::links(graph::neighbors(c, &n.id, "imports", Dir::In)?),
            commits: self.commits_touching(&n, 5)?,
            node: n,
        })
    }

    /// What a symbol (or file) is, who calls it, and what is known about it.
    pub fn explain(&self, query: &str) -> Result<Lookup<Explained>> {
        Ok(match self.pick(query)? {
            Lookup::Found(n) if n.ntype == "file" => {
                Lookup::Found(Explained::File(self.file_card(n)?))
            }
            Lookup::Found(n) => Lookup::Found(Explained::Symbol(self.symbol_card(n)?)),
            Lookup::Ambiguous { candidates } => Lookup::Ambiguous { candidates },
            Lookup::NotFound { query } => Lookup::NotFound { query },
        })
    }

    /// What depends on a symbol (or file) and which tests cover it.
    pub fn impact(&self, query: &str) -> Result<Lookup<ImpactReport>> {
        let n = match self.pick(query)? {
            Lookup::Found(n) => n,
            Lookup::Ambiguous { candidates } => return Ok(Lookup::Ambiguous { candidates }),
            Lookup::NotFound { query } => return Ok(Lookup::NotFound { query }),
        };
        let c = &self.db.conn;
        let (members, importers) = if n.ntype == "file" {
            let syms: Vec<Node> = graph::neighbors(c, &n.id, "defined_in", Dir::In)?
                .into_iter()
                .map(|(n, _)| n)
                .collect();
            (syms, graph::neighbors(c, &n.id, "imports", Dir::In)?)
        } else {
            let file = myrn_model::id::file_id(n.path.as_deref().unwrap_or(""));
            (
                walks::members(c, &n)?,
                graph::neighbors(c, &file, "imports", Dir::In)?,
            )
        };
        let mut targets = vec![n.id.clone()];
        targets.extend(members.iter().map(|m| m.id.clone()));
        let reach = walks::callers(c, &targets, 3, 300)?;
        let target_ids: HashSet<&String> = targets.iter().collect();
        let mut direct_tests = Vec::new();
        let mut seen = HashSet::new();
        for t in &targets {
            for test in walks::direct_tests(c, t)? {
                if seen.insert(test.id.clone()) {
                    direct_tests.push(Link::new(&test, None));
                }
            }
        }
        let indirect_tests: Vec<(Link, usize)> = reach
            .tests
            .iter()
            .filter(|(t, _)| !seen.contains(&t.id))
            .map(|(t, d)| (Link::new(t, None), *d))
            .collect();
        let mut files: HashSet<String> = HashSet::new();
        let mut direct = Vec::new();
        let mut indirect = Vec::new();
        for (caller, d, status) in &reach.callers {
            if target_ids.contains(&caller.id) {
                continue;
            }
            if let Some(p) = &caller.path {
                files.insert(p.clone());
            }
            if *d == 1 {
                direct.push(Link::new(caller, status.clone()));
            } else {
                indirect.push((Link::new(caller, status.clone()), *d));
            }
        }
        Ok(Lookup::Found(ImpactReport {
            target: Link::new(&n, None),
            members: members.iter().map(|m| Link::new(m, None)).collect(),
            direct_callers: direct,
            indirect_callers: indirect,
            importers: Self::links(importers),
            direct_tests,
            indirect_tests,
            files_affected: files.len(),
            truncated: reach.truncated,
        }))
    }

    /// Changes concerning a file, module or symbol, newest first.
    pub fn history(&self, query: &str, limit: usize) -> Result<Lookup<Vec<HistoryEntry>>> {
        let c = &self.db.conn;
        let shas: Vec<(String, Option<String>)> = match self.pick(query)? {
            Lookup::Found(n) if n.ntype == "file" => {
                let mut stmt = c.prepare(
                    "SELECT sha, change FROM commit_files WHERE path = ?1 OR old_path = ?1",
                )?;
                let rows = stmt.query_map([&n.path], |r| Ok((r.get(0)?, r.get(1)?)))?;
                rows.collect::<rusqlite::Result<_>>()?
            }
            Lookup::Found(n) => graph::neighbors(c, &n.id, "touches", Dir::In)?
                .into_iter()
                .map(|(c, _)| {
                    (
                        c.id.trim_start_matches("commit:").to_string(),
                        Some("modified".to_string()),
                    )
                })
                .collect(),
            Lookup::Ambiguous { candidates } => return Ok(Lookup::Ambiguous { candidates }),
            Lookup::NotFound { query: q } => {
                // A directory prefix.
                let like = format!("{}/%", q.trim_end_matches('/'));
                let mut stmt =
                    c.prepare("SELECT DISTINCT sha, NULL FROM commit_files WHERE path LIKE ?1")?;
                let rows = stmt.query_map([&like], |r| Ok((r.get(0)?, r.get(1)?)))?;
                let v: Vec<(String, Option<String>)> = rows.collect::<rusqlite::Result<_>>()?;
                if v.is_empty() {
                    return Ok(Lookup::NotFound { query: q });
                }
                v
            }
        };
        let mut out = Vec::new();
        for (sha, change) in shas {
            let Some(n) = graph::node(c, &myrn_model::id::commit_id(&sha))? else {
                continue;
            };
            let extra: serde_json::Value = n
                .extra
                .as_deref()
                .and_then(|e| serde_json::from_str(e).ok())
                .unwrap_or_default();
            out.push(HistoryEntry {
                sha: sha.clone(),
                summary: n.title.clone().unwrap_or_default(),
                author: extra["author"].as_str().unwrap_or("").to_string(),
                time: extra["time"].as_i64().unwrap_or(0),
                change,
                parents: extra["parents"]
                    .as_array()
                    .map(|v| {
                        v.iter()
                            .filter_map(|p| p.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
            });
        }
        // Newest first; commits made in the same second are ordered by parentage.
        out.sort_by(|a, b| {
            b.time.cmp(&a.time).then_with(|| {
                if a.parents.contains(&b.sha) {
                    std::cmp::Ordering::Less
                } else if b.parents.contains(&a.sha) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        });
        out.truncate(limit);
        Ok(Lookup::Found(out))
    }

    /// A ranked context pack for a described task. Empty when nothing is relevant enough.
    pub fn context(&self, task: &str, budget: Option<usize>) -> Result<Pack> {
        self.context_excluding(task, budget, &HashSet::new())
    }

    /// Ids injected into this session so far, so later prompts are not sent them again.
    pub fn injected_in_session(&self, session: &str) -> Result<HashSet<String>> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT DISTINCT note_id FROM usage WHERE session_id = ?1 AND injected = 1")?;
        let ids = stmt
            .query_map([session], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(ids)
    }

    /// A context pack that leaves out items the agent already has (`seen`).
    /// The relevance cutoff is set before they are removed, so weaker items
    /// never move up to fill their place.
    pub fn context_excluding(
        &self,
        task: &str,
        budget: Option<usize>,
        seen: &HashSet<String>,
    ) -> Result<Pack> {
        let cfg = &self.config.context;
        let budget = budget.unwrap_or(cfg.prompt_budget);
        let mut seeds = seed::seeds(&self.db, task)?;
        seeds.extend(self.vector_seeds(task)?);
        let ranked = rank::rank(&self.db, seeds, &self.config.ranking)?;
        let Some(top) = ranked.first() else {
            return Ok(Pack::empty());
        };
        // A request to add a new function (named in backticks, not in the code
        // yet) with no strong, standout match: pointers would be guesses at a
        // "neighbour", and agents follow them (on the benchmark both such
        // feature tasks opened an unrelated file first and copied its style, at
        // +30-65% tokens). Say nothing. "Add X to existing Y" keeps its pointers:
        // they found the right file 8 times in 10 (git-history test).
        let named = matches!(top.why, "named in task" | "named file");
        // Flat: the best match is barely ahead of the third, so none stands out.
        let flat = ranked
            .get(2)
            .is_none_or(|third| top.score - third.score < FLAT_LEAD);
        if !named
            && top.score < WEAK_TOP
            && flat
            && seed::asks_for_new_code(task)
            && seed::names_new_symbol(&self.db, task)?
        {
            return Ok(Pack::empty());
        }
        let question = !seed::asks_about_bug(task) && seed::is_question(task);
        // Only items close to the best match; never pad to fill the budget.
        let cutoff = (top.score * RELATIVE_CUTOFF).max(rank::MIN_SCORE);
        let history = asks_about_history(task);
        let tests = asks_about_tests(task);
        let mut items = Vec::new();
        for r in ranked.iter().filter(|r| r.score >= cutoff) {
            // Tests of a match are summarised on its card, not listed.
            if r.why == "tests a match" || seen.contains(&r.id) {
                continue;
            }
            let Some(n) = graph::node(&self.db.conn, &r.id)? else {
                continue;
            };
            let in_tests = n.ntype == "test" || n.path.as_deref().is_some_and(is_test_path);
            if (n.ntype == "commit" && !history) || (in_tests && !tests) || n.ntype == "module" {
                continue;
            }
            items.push(n);
            if items.len() >= cfg.max_items * 2 {
                break;
            }
        }
        seed::drop_shadowed_stubs(&mut items);
        items.truncate(cfg.max_items);
        // Code is shown only when the best match is certain: the task names it,
        // or it is far ahead of the best match in any other file. Otherwise one
        // line each, so a wrong guess costs little and does not look like the
        // answer. (On the benchmark tasks a small lead said nothing about being
        // right; being named did.)
        let ranked_of = |id: &str| ranked.iter().find(|r| r.id == id);
        let score_of = |id: &str| ranked_of(id).map_or(0.0, |r| r.score);
        let confident = items.first().is_some_and(|first| {
            // Named, and the name picks out one thing (not `TokenStore` in three languages).
            let named = ranked_of(&first.id)
                .is_some_and(|r| matches!(r.why, "named in task" | "named file"))
                && !items.iter().skip(1).any(|n| n.title == first.title);
            let runner_up = items
                .iter()
                .skip(1)
                .find(|n| n.path != first.path)
                .map_or(0.0, |n| score_of(&n.id));
            named || score_of(&first.id) >= runner_up * CONFIDENT_MARGIN
        });
        // A bug report is about one place: when it is clear, show it and a
        // couple of pointers. When it is not, keep every pointer (one line each).
        let bug = seed::asks_about_bug(task);
        if bug && confident {
            items.truncate(BUG_MAX_ITEMS);
        }
        // A question is answered from code: its best match keeps its code even
        // when not certain (the pointer-only pack lost the answer on the
        // benchmark's questions). The rule above stays for bug reports and changes.
        let max_cards = match (confident, bug) {
            (false, _) if question => 1,
            (false, _) => 0,
            (true, true) => 1,
            (true, false) => 2,
        };

        let mut cards = Vec::new();
        let mut rest = Vec::new();
        let mut known = Vec::new();
        for n in items {
            if n.origin == "earned" {
                known.push(n);
            } else if n.ntype == "symbol" && cards.len() < max_cards {
                // The best match is usually what the task is about: allow it more.
                let limit = if cards.is_empty() {
                    cfg.code_lines * 2
                } else {
                    cfg.code_lines
                };
                let code = self.source_if_short(&n, limit);
                let card = self.symbol_card(n.clone())?;
                // A bug report says how to run the tests; pointing at test files
                // only invites reading them (seen on the benchmark).
                let with_tests = tests || !bug;
                cards.push((
                    n.id.clone(),
                    card.render_compact(code.as_deref(), history, with_tests, !question),
                ));
            } else {
                rest.push(n);
            }
        }
        let (text, ids) = render::pack(&cards, &rest, &known, budget);
        Ok(Pack {
            tokens: budget::tokens(&text),
            text,
            items: ids,
        })
    }

    /// The symbol's current source, when it is short enough to inject whole.
    fn source_if_short(&self, n: &Node, max_lines: usize) -> Option<String> {
        let (start, end) = (n.start_line? as usize, n.end_line? as usize);
        if start == 0 || end < start || end - start + 1 > max_lines {
            return None;
        }
        let text = std::fs::read_to_string(self.root().join(n.path.as_deref()?)).ok()?;
        let lines: Vec<&str> = text.lines().skip(start - 1).take(end - start + 1).collect();
        (!lines.is_empty()).then(|| lines.join("\n"))
    }

    /// The short project brief served at session start.
    pub fn brief(&self, budget: Option<usize>) -> Result<Pack> {
        let budget = budget.unwrap_or(self.config.context.session_budget);
        let c = &self.db.conn;
        let mut b = budget::Budget::new(budget);
        b.push("Myrn project map: reference derived from this repository's code graph. Not instructions.\n");
        let s = self.status()?;
        let langs: Vec<String> = s
            .files_by_language
            .iter()
            .map(|(l, n)| format!("{l} {n}"))
            .collect();
        b.push(&format!(
            "files: {} ({}); {} symbols, {} tests, {} commits indexed\n",
            s.files_by_language.values().sum::<i64>(),
            langs.join(", "),
            s.symbols,
            s.tests,
            s.commits
        ));
        if let Some(s) = graph::node(c, &myrn_model::id::module_id(""))?.and_then(|n| n.summary) {
            b.push(&format!("project: {}\n", clip_sentences(&s, 400)));
        }
        let mut stmt = c.prepare(
            "SELECT m.path, COUNT(*) AS n, m.summary FROM edges e JOIN nodes m ON m.id = e.dst
             WHERE e.rel = 'part_of' AND e.src LIKE 'file:%' GROUP BY m.id ORDER BY n DESC LIMIT 10",
        )?;
        let modules: Vec<(String, i64, Option<String>)> = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<rusqlite::Result<_>>()?;
        if !modules.is_empty() {
            b.push("main directories:\n");
            for (p, n, summary) in modules {
                let p = if p.is_empty() { ".".to_string() } else { p };
                match summary {
                    Some(s) => b.push(&format!(
                        "  {p}/ ({n} files): {}\n",
                        clip_sentences(&s, 160)
                    )),
                    None => b.push(&format!("  {p} ({n} files)\n")),
                };
            }
        }
        let entries = graph::nodes_where(
            c,
            "type = 'file' AND (title IN ('main.rs', 'lib.rs', '__main__.py', 'main.py', 'cli.py', 'app.py',
             'index.ts', 'index.js', 'main.ts', 'main.js', 'server.ts', 'server.js'))
             ORDER BY length(path) LIMIT 8",
            &[],
        )?;
        if !entries.is_empty() {
            let e: Vec<String> = entries.iter().filter_map(|n| n.path.clone()).collect();
            b.push(&format!("entry points: {}\n", e.join(", ")));
        }
        b.push("Use the explain, impact and context tools for detail.\n");
        let text = b.out;
        Ok(Pack {
            tokens: budget::tokens(&text),
            text,
            items: Vec::new(),
        })
    }

    /// Record which items were injected into a session.
    /// Consent and connected agents (private, survives rebuild).
    pub fn state(&self) -> Result<myrn_store::State> {
        myrn_store::State::load(&self.layout.state_path())
    }

    pub fn save_state(&self, state: &myrn_store::State) -> Result<()> {
        state.save(&self.layout.state_path())
    }

    pub fn log_injection(&self, session: &str, items: &[String]) -> Result<()> {
        log::record(&self.db.conn, session, items)
    }

    // ------------------------------------------------------------ capture

    /// Record one hook event in the session's episode log, if the developer
    /// consented and this agent is connected with hooks. Returns whether
    /// anything was written.
    pub fn capture(&mut self, agent: &str, event: &myrn_model::event::Event) -> Result<bool> {
        use myrn_model::event::{EventKind, ToolOutcome};
        use myrn_store::episodes;

        if !capture::consent::may_record(&self.state()?, agent) {
            return Ok(false);
        }
        let ignore = indexer::walk::load_ignore(&self.layout.repo_root, &self.layout.ignore_path());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let root = self.layout.repo_root.clone();
        let Some(mut entry) = capture::to_entry(agent, event, &root, &ignore, now) else {
            return Ok(false);
        };
        let session = event.session_id.as_str();
        let dir = self.layout.episodes_dir();
        let log_path = episodes::path_for(&dir, session);

        // Files a shell command changed (`sed -i`, formatters, code generators):
        // recorded as an edit of its own, next to the command.
        let mut shell_edit = None;
        match &event.kind {
            EventKind::ToolResult(r) if r.outcome == ToolOutcome::FileEdit => {
                entry.symbols = self.touched_symbols(&entry.files)?;
            }
            EventKind::ToolResult(r) if r.command.is_some() => {
                let (files, symbols) = self.shell_changes()?;
                if !files.is_empty() {
                    shell_edit = Some(myrn_store::episodes::Entry {
                        t: now,
                        agent: agent.to_string(),
                        event: "tool".into(),
                        tool: entry.tool.clone(),
                        outcome: Some("file_edit".into()),
                        files,
                        symbols,
                        via: Some("shell".into()),
                        ..Default::default()
                    });
                }
            }
            EventKind::SessionEnd { .. } => {
                let started = episodes::read(&log_path)
                    .ok()
                    .and_then(|e| e.first().map(|e| e.t))
                    .unwrap_or(now);
                entry.commits = capture::outcome::commits_since(&root, started, 50);
            }
            _ => {}
        }
        episodes::append(&dir, session, &entry)?;
        if let Some(e) = &shell_edit {
            episodes::append(&dir, session, e)?;
        }
        let ep = capture::episode::fold(&episode_key(&log_path), &episodes::read(&log_path)?);
        capture::episode::persist(&self.db.conn, &ep)?;
        Ok(true)
    }

    /// After a shell command: refresh, and return the files it changed and the
    /// symbols whose body changed, appeared or disappeared. Files are compared
    /// with the index, so anything changed since the last refresh counts.
    fn shell_changes(&mut self) -> Result<(Vec<String>, Vec<String>)> {
        let before: BTreeMap<String, (String, Option<String>)> = {
            let mut stmt = self.db.conn.prepare(
                "SELECT id, path, body_hash FROM nodes WHERE type IN ('symbol', 'test')",
            )?;
            let rows =
                stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, (r.get(1)?, r.get(2)?))))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        let report = self.refresh()?;
        if report.changed_paths.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }
        let changed: HashSet<&str> = report.changed_paths.iter().map(String::as_str).collect();
        let mut symbols = Vec::new();
        let mut stmt = self.db.conn.prepare_cached(
            "SELECT id, body_hash FROM nodes WHERE path = ?1 AND type IN ('symbol', 'test')",
        )?;
        for path in &report.changed_paths {
            let rows = stmt.query_map([path], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
            })?;
            for row in rows {
                let (id, h) = row?;
                if before.get(&id).map(|(_, bh)| bh) != Some(&h) {
                    symbols.push(id);
                }
            }
        }
        // Symbols that disappeared from a changed file.
        for (id, (p, _)) in &before {
            if changed.contains(p.as_str()) && graph::node(&self.db.conn, id)?.is_none() {
                symbols.push(id.clone());
            }
        }
        symbols.sort();
        symbols.dedup();
        Ok((report.changed_paths, symbols))
    }

    /// Re-parse the edited files and return the symbols whose body changed,
    /// appeared or disappeared.
    fn touched_symbols(&mut self, files: &[String]) -> Result<Vec<String>> {
        let snapshot = |conn: &rusqlite::Connection| -> Result<BTreeMap<String, Option<String>>> {
            let mut out = BTreeMap::new();
            let mut stmt = conn.prepare_cached(
                "SELECT id, body_hash FROM nodes WHERE path = ?1 AND type IN ('symbol', 'test')",
            )?;
            for f in files {
                let rows = stmt.query_map([f], |r| Ok((r.get::<_, String>(0)?, r.get(1)?)))?;
                for row in rows {
                    let (id, h) = row?;
                    out.insert(id, h);
                }
            }
            Ok(out)
        };
        let before = snapshot(&self.db.conn)?;
        self.refresh()?;
        let after = snapshot(&self.db.conn)?;
        let mut out: Vec<String> = after
            .iter()
            .filter(|(id, h)| before.get(*id) != Some(h))
            .map(|(id, _)| id.clone())
            .collect();
        out.extend(before.keys().filter(|id| !after.contains_key(*id)).cloned());
        out.sort();
        Ok(out)
    }

    /// Recreate Episode nodes from the logs (after `rebuild`).
    pub fn reindex_episodes(&self) -> Result<usize> {
        let mut n = 0;
        for (key, path) in myrn_store::episodes::list(&self.layout.episodes_dir())? {
            let entries = myrn_store::episodes::read(&path)?;
            if entries.is_empty() {
                continue;
            }
            capture::episode::persist(&self.db.conn, &capture::episode::fold(&key, &entries))?;
            n += 1;
        }
        Ok(n)
    }

    /// Recorded sessions, newest first.
    pub fn episodes(&self, limit: usize) -> Result<Vec<Episode>> {
        let mut out = Vec::new();
        for (key, path) in myrn_store::episodes::list(&self.layout.episodes_dir())?
            .into_iter()
            .take(limit)
        {
            let entries = myrn_store::episodes::read(&path)?;
            if !entries.is_empty() {
                out.push(capture::episode::fold(&key, &entries));
            }
        }
        out.sort_by_key(|e| std::cmp::Reverse(e.started));
        Ok(out)
    }

    /// One session's log, by id or unique prefix.
    pub fn episode_log(
        &self,
        session: &str,
    ) -> Result<Option<(Episode, Vec<myrn_store::episodes::Entry>)>> {
        let all = myrn_store::episodes::list(&self.layout.episodes_dir())?;
        let wanted = myrn_store::episodes::file_name(session);
        let wanted = wanted.trim_end_matches(".jsonl");
        let matches: Vec<_> = all
            .iter()
            .filter(|(k, _)| k == wanted || k.starts_with(wanted))
            .collect();
        let pick = matches
            .iter()
            .find(|(k, _)| k == wanted)
            .or(if matches.len() == 1 {
                matches.first()
            } else {
                None
            });
        let Some((key, path)) = pick else {
            return Ok(None);
        };
        let entries = myrn_store::episodes::read(path)?;
        Ok(Some((capture::episode::fold(key, &entries), entries)))
    }

    /// One node with all its edges.
    pub fn show(&self, query: &str) -> Result<Lookup<(Node, Vec<graph::Edge>)>> {
        Ok(match self.pick(query)? {
            Lookup::Found(n) => {
                let edges = graph::edges_of(&self.db.conn, &n.id)?;
                Lookup::Found((n, edges))
            }
            Lookup::Ambiguous { candidates } => Lookup::Ambiguous { candidates },
            Lookup::NotFound { query } => Lookup::NotFound { query },
        })
    }

    pub fn status(&self) -> Result<Status> {
        let c = &self.db.conn;
        let pairs = |sql: &str| -> Result<BTreeMap<String, i64>> {
            let mut stmt = c.prepare(sql)?;
            let rows = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                    r.get(1)?,
                ))
            })?;
            Ok(rows.collect::<rusqlite::Result<_>>()?)
        };
        let earned = pairs(
            "SELECT status, COUNT(*) FROM nodes WHERE origin = 'earned' AND type != 'episode'
             GROUP BY status",
        )?;
        let records: Vec<(String, i64)> = myrn_model::Status::ALL
            .iter()
            .map(|s| {
                (
                    s.as_str().to_string(),
                    earned.get(s.as_str()).copied().unwrap_or(0),
                )
            })
            .collect();
        Ok(Status {
            root: self.layout.repo_root.display().to_string(),
            project_id: self.project_id.clone(),
            provisional_id: self.provisional_id,
            db_path: self.layout.db_path.display().to_string(),
            files_by_language: pairs("SELECT lang, COUNT(*) FROM files GROUP BY lang")?,
            files_with_errors: graph::count(
                c,
                "SELECT COUNT(*) FROM files WHERE error IS NOT NULL",
            )?,
            symbols: graph::count(c, "SELECT COUNT(*) FROM nodes WHERE type = 'symbol'")?,
            tests: graph::count(c, "SELECT COUNT(*) FROM nodes WHERE type = 'test'")?,
            modules: graph::count(c, "SELECT COUNT(*) FROM nodes WHERE type = 'module'")?,
            commits: graph::count(c, "SELECT COUNT(*) FROM nodes WHERE type = 'commit'")?,
            edges: pairs("SELECT rel, COUNT(*) FROM edges GROUP BY rel")?,
            calls_inferred: graph::count(
                c,
                "SELECT COUNT(*) FROM edges WHERE rel = 'calls' AND status = 'inferred'",
            )?,
            records_by_state: records,
            notes: sync::summary(c)?,
            conflicts: sync::conflicts(c)?,
            connected_agents: self.state()?.agents.keys().cloned().collect(),
            last_refresh: self
                .db
                .get_meta("last_refresh")?
                .and_then(|s| s.parse().ok()),
        })
    }

    /// Diagnose the installation, the store and the index.
    pub fn doctor(&mut self) -> Vec<Check> {
        let mut out = Vec::new();
        out.push(check(
            "repository",
            Level::Ok,
            self.layout.repo_root.display().to_string(),
        ));
        let config_ok = Config::load(&self.layout.config_path());
        match (&config_ok, self.provisional_id) {
            (Err(e), _) => out.push(check("project.toml", Level::Fail, format!("{e:#}"))),
            (Ok(_), true) => out.push(check(
                "project id",
                Level::Warn,
                "no commits yet; knowledge is stored under a temporary id until the first commit",
            )),
            (Ok(cfg), false) => {
                let actual = Repository::discover(self.root())
                    .ok()
                    .and_then(|r| git::root_commit(&r));
                match (&cfg.project.id, actual) {
                    (Some(id), Some(a)) if *id != a => out.push(check(
                        "project id",
                        Level::Warn,
                        format!("project.toml says {id} but the root commit is {a}"),
                    )),
                    _ => out.push(check("project id", Level::Ok, self.project_id.clone())),
                }
            }
        }
        let probe = self.layout.data_dir.join(".write-test");
        match std::fs::write(&probe, b"ok").and_then(|_| std::fs::remove_file(&probe)) {
            Ok(_) => out.push(check(
                "data directory",
                Level::Ok,
                self.layout.data_dir.display().to_string(),
            )),
            Err(e) => out.push(check(
                "data directory",
                Level::Fail,
                format!("{}: {e}", self.layout.data_dir.display()),
            )),
        }
        match (self.db.schema_version(), self.db.integrity_ok()) {
            (Ok(v), Ok(true)) => out.push(check(
                "database",
                Level::Ok,
                format!("schema v{v}, integrity ok"),
            )),
            (Ok(v), Ok(false)) => out.push(check(
                "database",
                Level::Fail,
                format!("schema v{v}, integrity check failed; run `myrn rebuild`"),
            )),
            (Err(e), _) | (_, Err(e)) => out.push(check("database", Level::Fail, format!("{e:#}"))),
        }
        for l in Lang::ALL {
            let mut p = tree_sitter::Parser::new();
            if let Err(e) = p.set_language(&l.grammar()) {
                out.push(check(
                    &format!("grammar {}", l.name()),
                    Level::Fail,
                    e.to_string(),
                ));
            }
        }
        out.push(check(
            "grammars",
            Level::Ok,
            "rust, python, javascript, typescript, tsx",
        ));
        match self.refresh() {
            Ok(r) => out.push(check(
                "index",
                if r.errors.is_empty() {
                    Level::Ok
                } else {
                    Level::Warn
                },
                format!(
                    "{} files, {} re-parsed, {} removed in {} ms{}",
                    r.files,
                    r.parsed,
                    r.removed,
                    r.millis,
                    if r.errors.is_empty() {
                        String::new()
                    } else {
                        format!("; {}", r.errors.join("; "))
                    }
                ),
            )),
            Err(e) => out.push(check("index", Level::Fail, format!("{e:#}"))),
        }
        let errors: Vec<String> = self
            .db
            .conn
            .prepare("SELECT path FROM files WHERE error IS NOT NULL ORDER BY path LIMIT 5")
            .and_then(|mut s| s.query_map([], |r| r.get(0))?.collect())
            .unwrap_or_default();
        if errors.is_empty() {
            out.push(check("parse errors", Level::Ok, "none"));
        } else {
            out.push(check(
                "parse errors",
                Level::Warn,
                format!("indexed as far as possible: {}", errors.join(", ")),
            ));
        }
        match (sync::summary(&self.db.conn), self.note_issues(5)) {
            (Ok(sum), Ok(issues)) => {
                let total = sum.private + sum.shared;
                let detail = format!(
                    "{total} notes ({} private, {} shared)",
                    sum.private, sum.shared
                );
                if issues.is_empty() {
                    out.push(check("notes", Level::Ok, detail));
                } else {
                    let list: Vec<String> = issues
                        .iter()
                        .map(|i| format!("{}: {}", i.path, i.detail))
                        .collect();
                    out.push(check(
                        "notes",
                        Level::Warn,
                        format!(
                            "{detail}; indexed as far as possible: {} malformed, {} dangling links, \
                             {} duplicate ids; first: {}",
                            sum.malformed,
                            sum.dangling,
                            sum.duplicates,
                            list.join("; ")
                        ),
                    ));
                }
            }
            (Err(e), _) | (_, Err(e)) => out.push(check("notes", Level::Fail, format!("{e:#}"))),
        }
        out.push(check(
            "agents",
            Level::Warn,
            "none connected (`myrn connect` is not available yet)",
        ));
        out
    }
}

impl Pack {
    pub fn empty() -> Pack {
        Pack {
            text: String::new(),
            items: Vec::new(),
            tokens: 0,
        }
    }
}

/// Items scoring below this share of the best match are left out.
const RELATIVE_CUTOFF: f64 = 0.4;
/// Below this, for a request to add new code, a best match that the task does
/// not name is a guess (the benchmark's unrelated packs topped out at 6.2-6.9;
/// a full match on the task's words scores up to about 10).
const WEAK_TOP: f64 = 7.0;
/// ...and it leads the third-best by less than this (both benchmark cases: 0.2, 0.5).
const FLAT_LEAD: f64 = 1.0;
/// A best match the task does not name shows its code only when it scores at
/// least this many times the best match in another file.
const CONFIDENT_MARGIN: f64 = 1.5;
/// Most items for a bug report: the place to fix and two pointers.
const BUG_MAX_ITEMS: usize = 3;

/// Does the task ask about history (so commits are worth showing)?
/// Whole sentences of `s` up to about `max` characters (at least the first, clipped).
fn clip_sentences(s: &str, max: usize) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = String::new();
    for sentence in s.split_inclusive(". ") {
        if !out.is_empty() && out.len() + sentence.len() > max {
            break;
        }
        out.push_str(sentence);
    }
    crate::capture::episode::clip(out.trim(), max)
}

fn asks_about_history(task: &str) -> bool {
    let t = task.to_lowercase();
    if t.contains("used to") {
        return true;
    }
    t.split(|c: char| !c.is_alphanumeric()).any(|w| {
        matches!(
            w,
            "commit"
                | "commits"
                | "history"
                | "historical"
                | "regression"
                | "regressed"
                | "regressions"
                | "broke"
                | "introduced"
                | "changed"
                | "revert"
                | "reverted"
                | "blame"
                | "previously"
                | "recently"
        )
    })
}

/// A file that holds tests: `tests/…`, `test_x.py`, `x_test.go`, `x.test.ts`, `x.spec.js`.
fn is_test_path(path: &str) -> bool {
    let file = path.rsplit('/').next().unwrap_or(path);
    path.split('/')
        .any(|d| matches!(d, "test" | "tests" | "__tests__" | "spec"))
        || file.starts_with("test_")
        || file.contains("_test.")
        || file.contains(".test.")
        || file.contains(".spec.")
}

/// Is the task about tests themselves?
/// Does the task ask about tests themselves? "Make sure the tests still pass"
/// does not: it only says how to check the work.
fn asks_about_tests(task: &str) -> bool {
    task.to_lowercase()
        .split(['.', '?', '!'])
        .filter(|s| !s.contains("pass"))
        .flat_map(|s| s.split(|c: char| !c.is_alphanumeric()))
        .any(|w| w.starts_with("test") || w == "spec" || w == "specs")
}

/// The session key an episode log is stored under (its file stem).
fn episode_key(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

#[cfg(test)]
mod pack_rule_tests {
    use super::*;

    #[test]
    fn keeping_tests_green_is_not_asking_about_tests() {
        assert!(!asks_about_tests(
            "`unique_everseen` returns duplicates. Find the cause and fix it. Make sure the test suite still passes\n(`python3 -m unittest discover -s tests -t .`)."
        ));
        assert!(!asks_about_tests(
            "Fix it. Make sure the ignore crate's tests still pass."
        ));
        assert!(asks_about_tests("Which tests cover `clamp`?"));
        assert!(asks_about_tests("Add a test for chunk with a zero size."));
    }
}
