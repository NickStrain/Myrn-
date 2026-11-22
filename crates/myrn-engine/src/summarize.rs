//! `myrn summarize`: the project map. A model (Haiku through the Claude Code
//! CLI by default) writes what the project, every directory, every code file
//! and every function or type is for. Each answer becomes a summary note in
//! `.myrn/knowledge/summaries/`, an Obsidian vault whose `[[links]]` follow the
//! code graph (contains, calls, imports), anchored to the hash of the code it
//! describes. A later run only asks again about code whose hash changed, so the
//! one-time cost is paid once and then only for edits.
//!
//! Agents never read these files directly: the summaries land on the code
//! nodes, where the brief, cards and context packs show them and search finds
//! code by what it does.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use anyhow::{bail, Context, Result};
use myrn_model::{id, Relation};
use myrn_store::graph::{self, Dir, Node};
use myrn_store::notes::{frontmatter, Anchor, Link, Note};
use myrn_store::sync;
use serde::Serialize;

use crate::api::Engine;

/// `source:` of every note this module writes; other summary notes are left alone.
pub const SOURCE: &str = "myrn summarize";
/// Symbol kinds that get a note. Constants, statics and fields are shown on their file's note.
const KINDS: [&str; 8] = [
    "function", "method", "class", "struct", "enum", "trait", "type", "macro",
];
/// Longest source sent for one file; longer files are cut.
const MAX_SOURCE: usize = 160_000;
/// Most links of one kind written on a note.
const MAX_LINKS: usize = 25;

#[derive(Debug, Clone)]
pub struct Options {
    /// Only report what is out of date.
    pub dry_run: bool,
    /// Most files to send to the model in this run (directories follow).
    pub limit: Option<usize>,
    pub jobs: usize,
    pub command: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub files: usize,
    pub symbols: usize,
    pub dirs: usize,
    /// Out of date or missing before this run.
    pub stale_files: usize,
    pub stale_symbols: usize,
    pub stale_dirs: usize,
    /// Files left for a later run because of `--limit`.
    pub deferred_files: usize,
    /// Summaries the model wrote in this run.
    pub updated_files: usize,
    pub updated_symbols: usize,
    pub updated_dirs: usize,
    pub model_calls: usize,
    pub cost_usd: f64,
    pub notes_written: usize,
    pub notes_removed: usize,
    pub failures: Vec<String>,
    pub vault: PathBuf,
}

/// One thing on the map.
#[derive(Debug, Clone)]
struct Entry {
    node: Node,
    /// Hash of what the summary describes: the code body, or the children of a directory.
    hash: String,
    note_id: String,
    summary: Option<String>,
    stale: bool,
}

#[derive(Debug, Clone)]
struct Existing {
    note_id: String,
    summary: String,
    hash: Option<String>,
    path: String,
}

impl Engine {
    /// Bring the project map up to date. `progress` gets one line per step.
    pub fn summarize(
        &mut self,
        opts: &Options,
        progress: &(dyn Fn(&str) + Sync),
    ) -> Result<Report> {
        self.refresh()?;
        let root = self.layout.knowledge_dir().join("summaries");
        let mut report = Report {
            vault: self.layout.knowledge_dir(),
            ..Default::default()
        };
        let c = &self.db.conn;

        // Notes written by earlier runs, by the node they are about.
        let mut existing: HashMap<String, Existing> = HashMap::new();
        let mut taken: HashSet<String> = HashSet::new();
        {
            let mut stmt = c.prepare("SELECT path, id, type, data FROM notes")?;
            let rows = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                ))
            })?;
            for row in rows {
                let (path, note_id, ntype, data) = row?;
                taken.insert(note_id.clone());
                if ntype != "summary" {
                    continue;
                }
                let Ok(note) = serde_json::from_str::<Note>(&data) else {
                    continue;
                };
                if note.source.as_deref() != Some(SOURCE) {
                    continue;
                }
                let Some(about) = note.links.iter().find(|l| l.rel == Relation::About) else {
                    continue;
                };
                existing.insert(
                    about.target.clone(),
                    Existing {
                        note_id: note.id.clone(),
                        summary: note.summary_text(),
                        hash: note.anchors.first().and_then(|a| a.hash.clone()),
                        path,
                    },
                );
            }
        }

        // Everything the map covers.
        let files = graph::nodes_where(c, "type = 'file' ORDER BY path", &[])?;
        let kinds = KINDS.map(|k| format!("'{k}'")).join(", ");
        let symbols = graph::nodes_where(
            c,
            &format!("type = 'symbol' AND kind IN ({kinds}) ORDER BY path, start_line"),
            &[],
        )?;
        let modules = graph::nodes_where(c, "type = 'module' ORDER BY path", &[])?;
        report.files = files.len();
        report.symbols = symbols.len();
        report.dirs = modules.len();

        // Note ids: kept from earlier runs, readable and unique for new entries.
        let mut entries: BTreeMap<String, Entry> = BTreeMap::new();
        let new_id = |n: &Node, taken: &mut HashSet<String>| -> String {
            let path = n.path.as_deref().unwrap_or("");
            let base = match n.ntype.as_str() {
                "module" if path.is_empty() => "project".to_string(),
                "module" => format!("dir-{}", slug(path)),
                "file" => format!("file-{}", slug(path)),
                _ => {
                    let segs = crate::retrieve::seed::segments(n.qname.as_deref().unwrap_or(""));
                    let tail = segs[segs.len().saturating_sub(2)..].join("-");
                    slug(&tail)
                }
            };
            let stem = slug(
                Path::new(path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(""),
            );
            let short = &blake3::hash(n.id.as_bytes()).to_hex()[..4];
            let id = [
                base.clone(),
                format!("{base}-{stem}"),
                format!("{base}-{short}"),
            ]
            .into_iter()
            .find(|c| !taken.contains(c))
            .unwrap_or_else(|| format!("{base}-{}", &blake3::hash(n.id.as_bytes()).to_hex()[..12]));
            taken.insert(id.clone());
            id
        };
        for n in modules.iter().chain(&files).chain(&symbols) {
            if let Some(e) = existing.get(&n.id) {
                taken.insert(e.note_id.clone());
            }
        }
        for n in modules.iter().chain(&files).chain(&symbols) {
            let prev = existing.get(&n.id);
            let note_id = match prev {
                Some(e) => e.note_id.clone(),
                None => new_id(n, &mut taken),
            };
            let hash = n.body_hash.clone().unwrap_or_default();
            let stale = n.ntype != "module"
                && !prev.is_some_and(|e| {
                    e.hash
                        .as_deref()
                        .is_some_and(|h| !h.is_empty() && hash.starts_with(h))
                });
            entries.insert(
                n.id.clone(),
                Entry {
                    node: n.clone(),
                    hash,
                    note_id,
                    summary: prev.map(|e| e.summary.clone()).filter(|s| !s.is_empty()),
                    stale,
                },
            );
        }

        // Files to send: the file or one of its symbols is out of date.
        let mut by_file: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for s in &symbols {
            by_file
                .entry(s.path.clone().unwrap_or_default())
                .or_default()
                .push(s.id.clone());
        }
        let mut jobs: Vec<FileJob> = Vec::new();
        for f in &files {
            let path = f.path.clone().unwrap_or_default();
            let syms = by_file.get(&path).cloned().unwrap_or_default();
            let stale_syms: Vec<String> = syms
                .iter()
                .filter(|id| entries[*id].stale)
                .cloned()
                .collect();
            let file_stale = entries[&f.id].stale;
            report.stale_symbols += stale_syms.len();
            report.stale_files += file_stale as usize;
            if file_stale || !stale_syms.is_empty() {
                jobs.push(FileJob {
                    file: f.id.clone(),
                    path,
                    lang: f.lang.clone().unwrap_or_default(),
                    symbols: stale_syms
                        .iter()
                        .map(|id| entries[id].node.clone())
                        .collect(),
                });
            }
        }
        if let Some(limit) = opts.limit {
            if jobs.len() > limit {
                report.deferred_files = jobs.len() - limit;
                jobs.truncate(limit);
            }
        }

        if opts.dry_run {
            report.stale_dirs = modules
                .iter()
                .filter(|m| !existing.contains_key(&m.id))
                .count();
            return Ok(report);
        }

        // 1. Files and their symbols.
        if !jobs.is_empty() {
            progress(&format!(
                "summarizing {} files ({} symbols) with {} at a time",
                jobs.len(),
                jobs.iter().map(|j| j.symbols.len()).sum::<usize>(),
                opts.jobs.max(1)
            ));
        }
        let root_dir = self.layout.repo_root.clone();
        let done = AtomicUsize::new(0);
        let total = jobs.len();
        let results = parallel(&jobs, opts.jobs, |job| {
            let r = summarize_file(&opts.command, &root_dir, job);
            let n = done.fetch_add(1, Ordering::Relaxed) + 1;
            match &r {
                Ok(_) => progress(&format!("  [{n}/{total}] {}", job.path)),
                Err(e) => progress(&format!("  [{n}/{total}] {} failed: {e:#}", job.path)),
            }
            r
        });
        for (job, r) in jobs.iter().zip(results) {
            report.model_calls += 1;
            match r {
                Ok(out) => {
                    report.cost_usd += out.cost;
                    if let Some(s) = out.file.filter(|s| !s.is_empty()) {
                        let e = entries.get_mut(&job.file).expect("file entry");
                        e.summary = Some(s);
                        e.stale = false;
                        report.updated_files += 1;
                    }
                    for n in &job.symbols {
                        let qn = n.qname.clone().unwrap_or_default();
                        // Exact qualified name, else the bare name when it is unique in the file.
                        let bare = n.title.clone().unwrap_or_default();
                        let unique = job.symbols.iter().filter(|m| m.title == n.title).count() == 1;
                        let answer = out
                            .symbols
                            .get(&qn)
                            .or_else(|| unique.then(|| out.symbols.get(&bare)).flatten());
                        if let Some(s) = answer.filter(|s| !s.is_empty()) {
                            let e = entries.get_mut(&n.id).expect("symbol entry");
                            e.summary = Some(s.clone());
                            e.stale = false;
                            report.updated_symbols += 1;
                        }
                    }
                }
                Err(e) => report.failures.push(format!("{}: {e:#}", job.path)),
            }
        }

        // 2. Directories, deepest first: each from its children's summaries.
        let mut children: HashMap<String, Vec<String>> = HashMap::new();
        for m in &modules {
            let mut kids: Vec<String> = graph::neighbors(c, &m.id, "part_of", Dir::In)?
                .into_iter()
                .map(|(n, _)| n.id)
                .filter(|id| entries.contains_key(id))
                .collect();
            kids.sort();
            children.insert(m.id.clone(), kids);
        }
        let mut levels: BTreeMap<std::cmp::Reverse<i64>, Vec<String>> = BTreeMap::new();
        for m in &modules {
            let p = m.path.as_deref().unwrap_or("");
            let depth = if p.is_empty() {
                -1
            } else {
                p.matches('/').count() as i64
            };
            levels
                .entry(std::cmp::Reverse(depth))
                .or_default()
                .push(m.id.clone());
        }
        let readme = readme_excerpt(&root_dir);
        for ids in levels.values() {
            let mut dir_jobs = Vec::new();
            for mid in ids {
                let kids = &children[mid];
                let mut h = blake3::Hasher::new();
                for k in kids {
                    h.update(k.as_bytes());
                    h.update(entries[k].summary.as_deref().unwrap_or("").as_bytes());
                }
                let hash = h.finalize().to_hex()[..16].to_string();
                let prev_hash = existing.get(mid).and_then(|e| e.hash.clone());
                let e = entries.get_mut(mid).expect("module entry");
                e.hash = hash.clone();
                let fresh = prev_hash.is_some_and(|p| p == hash) && e.summary.is_some();
                if fresh || kids.iter().all(|k| entries[k].summary.is_none()) {
                    continue;
                }
                report.stale_dirs += 1;
                let listing: Vec<String> = kids
                    .iter()
                    .map(|k| {
                        let e = &entries[k];
                        let kind = if e.node.ntype == "module" {
                            "dir "
                        } else {
                            "file"
                        };
                        let name = e.node.path.as_deref().unwrap_or("");
                        let name = name.rsplit('/').next().unwrap_or(name);
                        let slash = if e.node.ntype == "module" { "/" } else { "" };
                        format!(
                            "- {kind} {name}{slash}: {}",
                            e.summary.as_deref().unwrap_or("(no summary)")
                        )
                    })
                    .collect();
                dir_jobs.push(DirJob {
                    module: mid.clone(),
                    path: entries[mid].node.path.clone().unwrap_or_default(),
                    listing,
                });
            }
            if dir_jobs.is_empty() {
                continue;
            }
            progress(&format!("summarizing {} directories", dir_jobs.len()));
            let results = parallel(&dir_jobs, opts.jobs, |j| {
                summarize_dir(&opts.command, &root_dir, j, readme.as_deref())
            });
            for (j, r) in dir_jobs.iter().zip(results) {
                report.model_calls += 1;
                match r {
                    Ok((s, cost)) => {
                        report.cost_usd += cost;
                        entries.get_mut(&j.module).expect("module entry").summary = Some(s);
                        report.updated_dirs += 1;
                    }
                    Err(e) => report
                        .failures
                        .push(format!("{}/: {e:#}", display_dir(&j.path))),
                }
            }
        }

        // 3. Write every note whose content changed; remove notes for code that is gone.
        progress("writing the map");
        let has_note: HashMap<String, String> = entries
            .values()
            .filter(|e| e.summary.is_some())
            .map(|e| (e.node.id.clone(), e.note_id.clone()))
            .collect();
        for sub in ["project", "dirs", "files", "symbols"] {
            std::fs::create_dir_all(root.join(sub))?;
        }
        for e in entries.values() {
            let Some(summary) = &e.summary else { continue };
            let body = self.map_body(e, summary, &has_note)?;
            let note = Note {
                id: e.note_id.clone(),
                ntype: "summary".into(),
                status: "unverified".into(),
                title: Some(title_of(&e.node)),
                confidence: None,
                source: Some(SOURCE.into()),
                signature: None,
                links: vec![Link {
                    rel: Relation::About,
                    target: e.node.id.clone(),
                    inverse: false,
                }],
                anchors: vec![Anchor {
                    target: e.node.id.clone(),
                    hash: Some(e.hash.clone()).filter(|h| !h.is_empty()),
                }],
                evidence: Vec::new(),
                extra: Vec::new(),
                body,
            };
            let sub = match e.node.ntype.as_str() {
                "module" if e.node.path.as_deref() == Some("") => "project",
                "module" => "dirs",
                "file" => "files",
                _ => "symbols",
            };
            let path = root
                .join(sub)
                .join(myrn_store::notes::layout::file_name(&note.id));
            let text = frontmatter::render(&note, &|_| false);
            if std::fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
                std::fs::write(&path, text)
                    .with_context(|| format!("writing {}", path.display()))?;
                report.notes_written += 1;
            }
            // A note that moved folder (or an older layout) leaves its old file behind.
            if let Some(old) = existing.get(&e.node.id) {
                let old_path = self.layout.repo_root.join(&old.path);
                if old_path != path && old_path.exists() {
                    std::fs::remove_file(&old_path)?;
                }
            }
        }
        for (target, e) in &existing {
            if !entries.contains_key(target) {
                let p = self.layout.repo_root.join(&e.path);
                if p.exists() {
                    std::fs::remove_file(&p)?;
                    report.notes_removed += 1;
                }
            }
        }
        let roots = self.note_roots();
        sync::sync(&mut self.db.conn, &roots, true)?;
        Ok(report)
    }

    /// The note body: heading, summary, then the links Obsidian draws.
    fn map_body(
        &self,
        e: &Entry,
        summary: &str,
        notes: &HashMap<String, String>,
    ) -> Result<String> {
        let c = &self.db.conn;
        let n = &e.node;
        let link_list = |ids: Vec<String>| -> String {
            let mut seen = HashSet::new();
            ids.iter()
                .filter_map(|id| notes.get(id))
                .filter(|l| seen.insert((*l).clone()))
                .take(MAX_LINKS)
                .map(|l| format!("[[{l}]]"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let ids = |rel: &str, dir: Dir| -> Result<Vec<String>> {
            Ok(graph::neighbors(c, &n.id, rel, dir)?
                .into_iter()
                .map(|(m, _)| m.id)
                .collect())
        };
        let mut rows: Vec<(String, String)> = Vec::new();
        let mut push = |k: &str, v: String| {
            if !v.is_empty() {
                rows.push((k.to_string(), v));
            }
        };
        match n.ntype.as_str() {
            "module" => {
                let parent = n
                    .path
                    .as_deref()
                    .filter(|p| !p.is_empty())
                    .map(|p| id::module_id(id::parent_dir(p)));
                push("In", link_list(parent.into_iter().collect()));
                let kids = ids("part_of", Dir::In)?;
                push(
                    "Directories",
                    link_list(
                        kids.iter()
                            .filter(|k| k.starts_with("module:"))
                            .cloned()
                            .collect(),
                    ),
                );
                push(
                    "Files",
                    link_list(
                        kids.iter()
                            .filter(|k| k.starts_with("file:"))
                            .cloned()
                            .collect(),
                    ),
                );
            }
            "file" => {
                let path = n.path.clone().unwrap_or_default();
                push("Location", format!("`{path}`"));
                push("In", link_list(vec![id::module_id(id::parent_dir(&path))]));
                let defined: Vec<String> = graph::nodes_where(
                    c,
                    "type = 'symbol' AND path = ?1 ORDER BY start_line",
                    &[&path],
                )?
                .into_iter()
                .map(|s| s.id)
                .collect();
                push("Defines", link_list(defined));
                push("Imports", link_list(ids("imports", Dir::Out)?));
                push("Imported by", link_list(ids("imports", Dir::In)?));
            }
            _ => {
                push("Location", format!("`{}`", n.location()));
                if let Some(sig) = n.signature.as_deref().filter(|s| !s.is_empty()) {
                    let sig = sig.split_whitespace().collect::<Vec<_>>().join(" ");
                    push(
                        "Signature",
                        format!("`{}`", crate::capture::episode::clip(&sig, 200)),
                    );
                }
                push(
                    "In",
                    link_list(vec![id::file_id(n.path.as_deref().unwrap_or(""))]),
                );
                push("Calls", link_list(ids("calls", Dir::Out)?));
                push("Called by", link_list(ids("calls", Dir::In)?));
                let tests: Vec<String> = crate::retrieve::walks::direct_tests(c, &n.id)?
                    .into_iter()
                    .take(6)
                    .map(|t| format!("`{}`", t.title.unwrap_or_default()))
                    .collect();
                push("Tests", tests.join(", "));
            }
        }
        let mut body = format!("# {}\n\n{}\n", title_of(n), summary.trim());
        if !rows.is_empty() {
            body.push_str("\n## Connections\n\n");
            for (k, v) in rows {
                body.push_str(&format!("- **{k}:** {v}\n"));
            }
        }
        Ok(body)
    }
}

struct FileJob {
    file: String,
    path: String,
    lang: String,
    symbols: Vec<Node>,
}

struct FileOut {
    file: Option<String>,
    symbols: HashMap<String, String>,
    cost: f64,
}

struct DirJob {
    module: String,
    path: String,
    listing: Vec<String>,
}

const STYLE: &str = "You write entries for a code map that AI coding agents read instead of \
opening files, so they can decide what to edit. Be concrete: name what the code actually \
does, what it takes and returns, its side effects, and any non-obvious rule or gotcha. No \
filler (never start with \"This function\" or \"This file\"), no markdown, no guessing beyond \
the code shown.";

fn summarize_file(command: &[String], cwd: &Path, job: &FileJob) -> Result<FileOut> {
    let src = std::fs::read_to_string(cwd.join(&job.path))
        .with_context(|| format!("reading {}", job.path))?;
    let src = if src.len() > MAX_SOURCE {
        let mut cut = MAX_SOURCE;
        while !src.is_char_boundary(cut) {
            cut -= 1;
        }
        format!("{}\n... (cut: file continues)", &src[..cut])
    } else {
        src
    };
    let mut prompt = format!(
        "{STYLE}\n\nFile: {path}\n\nReturn ONLY a JSON object, no code fence:\n\
         {{\"file\": \"<what this file is for and its main parts, 1-2 sentences>\", \
         \"symbols\": {{\"<name>\": \"<what it does, 1 sentence, 2 at most>\"}}}}\n",
        path = job.path
    );
    if job.symbols.is_empty() {
        prompt.push_str("Use an empty object for \"symbols\".\n");
    } else {
        prompt.push_str("\"symbols\" must have exactly these keys:\n");
        for s in &job.symbols {
            prompt.push_str(&format!(
                "- {} ({}, lines {}-{})\n",
                s.qname.as_deref().unwrap_or(""),
                s.kind.as_deref().unwrap_or("symbol"),
                s.start_line.unwrap_or(0),
                s.end_line.unwrap_or(0)
            ));
        }
    }
    prompt.push_str(&format!("\nSource:\n```{}\n{src}\n```\n", job.lang));
    let (v, cost) = ask_json(command, &prompt)?;
    let mut symbols = HashMap::new();
    if let Some(map) = v["symbols"].as_object() {
        for (k, s) in map {
            if let Some(s) = s.as_str() {
                symbols.insert(k.clone(), clean(s));
            }
        }
    }
    Ok(FileOut {
        file: v["file"].as_str().map(clean),
        symbols,
        cost,
    })
}

fn summarize_dir(
    command: &[String],
    _cwd: &Path,
    job: &DirJob,
    readme: Option<&str>,
) -> Result<(String, f64)> {
    let project = job.path.is_empty();
    let mut prompt = if project {
        format!(
            "{STYLE}\n\nDescribe the whole project: what it is for, how it works, and how its \
             top-level parts fit together, in 2-4 sentences.\n"
        )
    } else {
        format!(
            "{STYLE}\n\nDirectory: {}/\nDescribe what this directory is for and how its parts \
             fit together, in 1-3 sentences.\n",
            job.path
        )
    };
    prompt.push_str(
        "Return ONLY a JSON object, no code fence: {\"summary\": \"...\"}\n\nContents:\n",
    );
    for l in &job.listing {
        prompt.push_str(l);
        prompt.push('\n');
    }
    if let (true, Some(r)) = (project, readme) {
        prompt.push_str(&format!("\nREADME (excerpt):\n{r}\n"));
    }
    let (v, cost) = ask_json(command, &prompt)?;
    match v["summary"].as_str() {
        Some(s) if !s.trim().is_empty() => Ok((clean(s), cost)),
        _ => bail!("no summary in the answer"),
    }
}

/// Run the model command with the prompt on stdin. Returns the answer text and
/// its cost when the command reports one (Claude Code's JSON output).
fn run_model(command: &[String], prompt: &str) -> Result<(String, f64)> {
    let Some((prog, args)) = command.split_first() else {
        bail!("[summarize] command is empty");
    };
    // Run outside the repository, so the agent's project hooks (and Myrn's own) stay out of it.
    let mut child = Command::new(prog)
        .args(args)
        .current_dir(std::env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!("starting `{prog}` (set [summarize] command in .myrn/project.toml)")
        })?;
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(prompt.as_bytes())?;
    let out = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let msg = if err.trim().is_empty() {
            &stdout
        } else {
            &*err
        };
        bail!(
            "`{prog}` failed: {}",
            crate::capture::episode::clip(msg.trim(), 300)
        );
    }
    // Claude Code `--output-format json`: {"result": "...", "total_cost_usd": ...}
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
        if let Some(r) = v["result"].as_str() {
            if v["is_error"].as_bool() == Some(true) {
                bail!("model error: {}", crate::capture::episode::clip(r, 300));
            }
            return Ok((r.to_string(), v["total_cost_usd"].as_f64().unwrap_or(0.0)));
        }
    }
    Ok((stdout, 0.0))
}

/// Ask, and ask once more if the answer is not valid JSON.
fn ask_json(command: &[String], prompt: &str) -> Result<(serde_json::Value, f64)> {
    let (text, mut cost) = run_model(command, prompt)?;
    match parse_json(&text) {
        Ok(v) => Ok((v, cost)),
        Err(_) => {
            let (text, c) = run_model(command, prompt)?;
            cost += c;
            Ok((parse_json(&text)?, cost))
        }
    }
}

/// The first JSON object in a model answer (tolerates a code fence or prose around it).
fn parse_json(text: &str) -> Result<serde_json::Value> {
    let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) else {
        bail!(
            "answer has no JSON: {}",
            crate::capture::episode::clip(text.trim(), 200)
        );
    };
    serde_json::from_str(&text[start..=end]).with_context(|| {
        format!(
            "answer is not valid JSON: {}",
            crate::capture::episode::clip(text.trim(), 200)
        )
    })
}

/// Run `f` over `items` with at most `jobs` at a time, results in input order.
fn parallel<T: Sync, R: Send>(items: &[T], jobs: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..jobs.clamp(1, 16).min(items.len().max(1)) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(item) = items.get(i) else { break };
                let r = f(item);
                out.lock().expect("results lock")[i] = Some(r);
            });
        }
    });
    out.into_inner()
        .expect("results lock")
        .into_iter()
        .map(|r| r.expect("every item ran"))
        .collect()
}

fn readme_excerpt(root: &Path) -> Option<String> {
    for name in [
        "README.md",
        "README",
        "readme.md",
        "README.rst",
        "README.txt",
    ] {
        if let Ok(t) = std::fs::read_to_string(root.join(name)) {
            let mut cut = t.len().min(4000);
            while !t.is_char_boundary(cut) {
                cut -= 1;
            }
            return Some(t[..cut].to_string());
        }
    }
    None
}

fn title_of(n: &Node) -> String {
    match n.ntype.as_str() {
        "module" => match n.path.as_deref() {
            Some("") | None => "Project".to_string(),
            Some(p) => format!("{p}/"),
        },
        "file" => n.path.clone().unwrap_or_default(),
        _ => {
            let segs = crate::retrieve::seed::segments(n.qname.as_deref().unwrap_or(""));
            segs[segs.len().saturating_sub(2)..].join("::")
        }
    }
}

fn display_dir(p: &str) -> &str {
    if p.is_empty() {
        "."
    } else {
        p
    }
}

/// One line, no stray whitespace, never a `## ` that would cut the summary.
fn clean(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Lowercase ASCII words joined by `-`: `crates/myrn/src/main.rs` -> `crates-myrn-src-main-rs`.
pub fn slug(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        out.push('x');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_are_obsidian_safe() {
        assert_eq!(slug("crates/myrn/src/main.rs"), "crates-myrn-src-main-rs");
        assert_eq!(slug("Engine-context"), "engine-context");
        assert_eq!(slug("__init__"), "init");
        assert_eq!(slug("..."), "x");
    }

    #[test]
    fn json_is_found_inside_noise() {
        let v = parse_json("Here:\n```json\n{\"summary\": \"a b\"}\n```").unwrap();
        assert_eq!(v["summary"], "a b");
        assert!(parse_json("no json").is_err());
    }

    #[test]
    fn parallel_keeps_order() {
        let items: Vec<usize> = (0..20).collect();
        assert_eq!(
            parallel(&items, 4, |i| i * 2),
            (0..20).map(|i| i * 2).collect::<Vec<_>>()
        );
    }
}
