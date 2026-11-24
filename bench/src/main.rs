//! Paired benchmark: same tasks, same repos, with and without Myrn.
//! Gate for turning automatic injection on by default.
//!
//! Design doc: "Evaluation". Usage: `bench/README.md`.

mod metrics;
mod report;
mod retrieval;
mod runner;
mod task;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

use runner::{Condition, Paths, RunRecord, RunSpec, Tools};

#[derive(Parser)]
#[command(
    name = "myrn-bench",
    about = "Paired with/without benchmark for myrn (Gate 1)"
)]
struct Cli {
    /// Directory with repos/ and tasks/
    #[arg(long, global = true, default_value = concat!(env!("CARGO_MANIFEST_DIR")))]
    bench_dir: PathBuf,
    /// Where repository checkouts live (default: <bench-dir>/repos/checkouts)
    #[arg(long, global = true)]
    checkouts: Option<PathBuf>,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List repositories and tasks
    List,
    /// Clone missing repository checkouts (network)
    Fetch,
    /// Check every task without an agent: broken before, fixed by the reference solution
    Validate {
        /// Only tasks whose id starts with this
        #[arg(long)]
        task: Option<String>,
    },
    /// Run the benchmark (spends agent usage: tasks x runs x 2 sessions)
    Run {
        /// Runs per task and condition
        #[arg(long, default_value_t = 3)]
        runs: usize,
        /// Number of the first run, to add more runs to an existing results
        /// directory without overwriting its transcripts (e.g. --first-run 2)
        #[arg(long, default_value_t = 1)]
        first_run: usize,
        /// Only tasks whose id starts with one of these (comma-separated)
        #[arg(long)]
        task: Option<String>,
        /// Model passed to `claude --model`
        #[arg(long)]
        model: Option<String>,
        /// Comma-separated: without (no myrn), off (myrn, injection off), with (myrn, injection on)
        #[arg(long, default_value = "without,with")]
        conditions: String,
        /// Output directory (default: <bench-dir>/runs/<time>)
        #[arg(long)]
        out: Option<PathBuf>,
        /// The myrn binary under test (default: target/release/myrn)
        #[arg(long)]
        myrn: Option<PathBuf>,
        #[arg(long, default_value = "claude")]
        claude: PathBuf,
        /// Runs happen in throwaway clones, so the agent may act without asking
        #[arg(long, default_value = "bypassPermissions")]
        permission_mode: String,
        /// Print the plan and stop
        #[arg(long)]
        dry_run: bool,
        /// Don't give myrn runs the project map (`myrn summarize`)
        #[arg(long)]
        no_map: bool,
        /// One myrn store per repository and condition for the whole run, so what
        /// is learned in one session ranks the next (use with `--conditions fixed,with`)
        #[arg(long)]
        shared_store: bool,
        /// Turn on semantic search in myrn runs (the model must be downloaded:
        /// `myrn embeddings enable` once on this machine)
        #[arg(long)]
        embeddings: bool,
        /// Don't attach myrn's MCP server to myrn sessions (`myrn connect --no-mcp`)
        #[arg(long)]
        no_mcp: bool,
    },
    /// What myrn would inject for each task, and whether it points at the right
    /// code. No agent sessions, no cost.
    Retrieval {
        /// Only tasks whose id starts with one of these (comma-separated)
        #[arg(long)]
        task: Option<String>,
        /// Output directory for retrieval.md (default: <bench-dir>/runs/retrieval-<time>)
        #[arg(long)]
        out: Option<PathBuf>,
        /// The myrn binary under test (default: target/release/myrn)
        #[arg(long)]
        myrn: Option<PathBuf>,
        /// Don't give myrn the project map (`myrn summarize`)
        #[arg(long)]
        no_map: bool,
        /// Print each pack's text
        #[arg(long)]
        show: bool,
        /// Keep the prepared checkouts and myrn data (out/work, out/home) for debugging
        #[arg(long)]
        keep: bool,
        /// Turn on myrn's semantic search (needs `myrn embeddings enable` once)
        #[arg(long)]
        embeddings: bool,
    },
    /// Rebuild report.md from a run directory's results.jsonl
    Report {
        dir: PathBuf,
        /// Recompute session metrics from the saved raw transcripts (after a metrics fix)
        #[arg(long)]
        rescore: bool,
    },
    /// Gate 2: sample the records a run produced into a checklist (review.md),
    /// or score the marked checklist
    Review {
        dir: PathBuf,
        #[arg(long, default_value_t = 30)]
        sample: usize,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Score <dir>/review.md after marking it
        #[arg(long)]
        score: bool,
    },
}

fn main() -> ExitCode {
    match real_main() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("myrn-bench: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<ExitCode> {
    let cli = Cli::parse();
    let repos = task::load_repos(&cli.bench_dir.join("repos"))?;
    let tasks = task::load_tasks(&cli.bench_dir.join("tasks"))?;
    let checkouts = cli
        .checkouts
        .clone()
        .unwrap_or_else(|| cli.bench_dir.join("repos/checkouts"));
    let checkouts = absolute(&checkouts)?;
    // `--task a,b`: tasks whose id starts with any of the prefixes.
    let pick = |prefix: &Option<String>| -> Vec<&task::Task> {
        let prefixes: Vec<&str> = prefix
            .as_deref()
            .map(|p| {
                p.split(',')
                    .map(str::trim)
                    .filter(|p| !p.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        tasks
            .iter()
            .filter(|t| prefixes.is_empty() || prefixes.iter().any(|p| t.id.starts_with(p)))
            .collect()
    };

    match cli.command {
        Cmd::List => {
            for r in repos.values() {
                println!(
                    "{}  {}  {} @ {}",
                    r.name,
                    r.language,
                    r.url,
                    &r.commit[..12]
                );
            }
            println!();
            for t in &tasks {
                println!("{:<48} {}", t.id, t.prompt.lines().next().unwrap_or(""));
            }
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Fetch => {
            std::fs::create_dir_all(&checkouts)?;
            for r in repos.values() {
                let dir = checkouts.join(&r.name);
                if dir.join(".git").exists() {
                    println!("{}: present", r.name);
                    continue;
                }
                println!("{}: cloning {}", r.name, r.url);
                let ok = Command::new("git")
                    .args(["clone", "-q", &r.url])
                    .arg(&dir)
                    .status()?
                    .success();
                if !ok {
                    bail!("cloning {} failed", r.url);
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Validate { task } => {
            let scratch =
                std::env::temp_dir().join(format!("myrn-bench-validate-{}", std::process::id()));
            let paths = Paths {
                checkouts,
                work: scratch.join("work"),
                target: cli.bench_dir.join("runs/target"),
            };
            let mut bad = 0;
            for t in pick(&task) {
                let repo = repos
                    .get(&t.repo)
                    .with_context(|| format!("{}: unknown repo", t.id))?;
                print!("{:<48} ", t.id);
                std::io::stdout().flush()?;
                let v = runner::validate(t, repo, &paths);
                let ok = match (v.broken_fails, v.solution_passes) {
                    (None, None) if t.kind == task::Kind::Question => {
                        println!("ok   ({})", v.note);
                        continue;
                    }
                    (Some(true), Some(true)) => true,
                    _ => false,
                };
                if ok {
                    println!("ok   (check fails when broken, passes with the reference solution)");
                } else {
                    bad += 1;
                    println!("FAIL\n{}", indent(&v.note));
                }
            }
            let _ = std::fs::remove_dir_all(&scratch);
            Ok(if bad == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Cmd::Run {
            runs,
            first_run,
            task,
            model,
            conditions,
            out,
            myrn,
            claude,
            permission_mode,
            dry_run,
            no_map,
            shared_store,
            embeddings,
            no_mcp,
        } => {
            let conds: Vec<Condition> = conditions
                .split(',')
                .map(|c| Condition::parse(c.trim()))
                .collect::<Result<_>>()?;
            let selected = pick(&task);
            if selected.is_empty() {
                bail!("no tasks match");
            }
            let myrn = match myrn {
                Some(p) => p,
                None => {
                    let p = cli.bench_dir.join("../target/release/myrn");
                    if !p.exists() {
                        bail!("build myrn first (`cargo build --release -p myrn`) or pass --myrn");
                    }
                    p
                }
            };
            let myrn = myrn.canonicalize().context("finding the myrn binary")?;
            let out = out.unwrap_or_else(|| {
                cli.bench_dir
                    .join("runs")
                    .join(format!("run-{}", runner::now()))
            });
            let out = absolute(&out)?;
            let sessions = selected.len() * runs * conds.len();
            println!(
                "{} tasks x {} runs x {} conditions = {} agent sessions, model {}",
                selected.len(),
                runs,
                conds.len(),
                sessions,
                model.as_deref().unwrap_or("default")
            );
            println!("results: {}", out.display());
            if dry_run {
                for t in &selected {
                    println!("  {}", t.id);
                }
                return Ok(ExitCode::SUCCESS);
            }
            std::fs::create_dir_all(out.join("raw"))?;
            let paths = Paths {
                checkouts,
                work: out.join("work"),
                target: cli.bench_dir.join("runs/target"),
            };
            // One project map per repository, built at the pinned commit and cached.
            let mut maps = std::collections::HashMap::new();
            if !no_map && conds.iter().any(|c| c.uses_myrn()) {
                let cache = cli.bench_dir.join("runs/maps");
                for t in &selected {
                    if maps.contains_key(&t.repo) {
                        continue;
                    }
                    let repo = repos
                        .get(&t.repo)
                        .with_context(|| format!("{}: unknown repo", t.id))?;
                    println!("project map for {} ...", repo.name);
                    let (dir, cost) =
                        runner::build_map(&paths.checkouts.join(&repo.name), repo, &myrn, &cache)?;
                    if cost > 0.0 {
                        println!("  built for ${cost:.4} (one-time, not in session tokens)");
                    } else {
                        println!("  cached: {}", dir.display());
                    }
                    maps.insert(t.repo.clone(), dir);
                }
            }
            let tools = Tools {
                claude,
                myrn,
                model,
                permission_mode,
                maps,
                shared_store,
                embeddings: if embeddings {
                    let dir = myrn_embed::model_dir(&myrn_store::paths::data_home()?);
                    if !dir.join("model.safetensors").exists() {
                        bail!(
                            "no embedding model at {}; run `myrn embeddings enable` once first",
                            dir.display()
                        );
                    }
                    Some(dir)
                } else {
                    None
                },
                mcp: !no_mcp,
            };
            let results = out.join("results.jsonl");
            let mut n = 0;
            for run in first_run.max(1)..first_run.max(1) + runs {
                for t in &selected {
                    let repo = repos
                        .get(&t.repo)
                        .with_context(|| format!("{}: unknown repo", t.id))?;
                    // Alternate which condition goes first, so drift over time
                    // (caches, rate limits) does not favour one side.
                    let mut order = conds.clone();
                    if run % 2 == 0 {
                        order.reverse();
                    }
                    for c in order {
                        n += 1;
                        print!("[{n}/{sessions}] {} {} #{run} ... ", t.id, c.as_str());
                        std::io::stdout().flush()?;
                        let rec = runner::run_one(
                            &RunSpec {
                                task: t,
                                repo,
                                condition: c,
                                run,
                            },
                            &tools,
                            &paths,
                            &out.join("raw"),
                        );
                        match &rec.error {
                            Some(e) => println!("error: {}", e.lines().next().unwrap_or("")),
                            None => println!(
                                "{} {:.1}k tokens ({:.1}k recording, {} records) {:.0}s",
                                if rec.success { "pass" } else { "fail" },
                                rec.session.tokens.total() as f64 / 1000.0,
                                rec.session.record_tokens as f64 / 1000.0,
                                rec.records,
                                rec.wall_ms as f64 / 1000.0
                            ),
                        }
                        append(&results, &rec)?;
                    }
                }
            }
            let _ = std::fs::remove_dir_all(&paths.work);
            write_report(&out)?;
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Review {
            dir,
            sample,
            seed,
            score,
        } => {
            let file = dir.join("review.md");
            if score {
                let s = myrn_engine::review::score(&std::fs::read_to_string(&file)?);
                println!("{}", serde_json::to_string_pretty(&s)?);
                return Ok(ExitCode::SUCCESS);
            }
            let mut all: Vec<myrn_engine::review::ReviewItem> = Vec::new();
            let mut files: Vec<PathBuf> = std::fs::read_dir(dir.join("records"))
                .context("no records/ in that run")?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect();
            files.sort();
            for f in files {
                let items: Vec<myrn_engine::review::ReviewItem> =
                    serde_json::from_str(&std::fs::read_to_string(&f)?)?;
                all.extend(items);
            }
            let total = all.len();
            let picked = myrn_engine::review::sample(all, sample, seed);
            let text = myrn_engine::review::checklist(
                &picked,
                &format!(
                    "Gate 2 record review: {} of {total} records from {} (seed {seed})",
                    picked.len(),
                    dir.file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default()
                ),
            );
            std::fs::write(&file, text)?;
            println!("{} of {total} records -> {}", picked.len(), file.display());
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Retrieval {
            task,
            out,
            myrn,
            no_map,
            show,
            keep,
            embeddings,
        } => {
            let selected = pick(&task);
            if selected.is_empty() {
                bail!("no tasks match");
            }
            let myrn = myrn.unwrap_or_else(|| cli.bench_dir.join("../target/release/myrn"));
            let myrn = myrn.canonicalize().context(
                "finding the myrn binary (build it with `cargo build --release -p myrn`)",
            )?;
            let out = absolute(&out.unwrap_or_else(|| {
                cli.bench_dir
                    .join("runs")
                    .join(format!("retrieval-{}", runner::now()))
            }))?;
            std::fs::create_dir_all(&out)?;
            let cache = cli.bench_dir.join("runs/maps");
            let mut maps = std::collections::HashMap::new();
            let mut rows = Vec::new();
            let emb = if embeddings {
                let dir = myrn_embed::model_dir(&myrn_store::paths::data_home()?);
                if !dir.join("model.safetensors").exists() {
                    bail!(
                        "no embedding model at {}; run `myrn embeddings enable` once first",
                        dir.display()
                    );
                }
                Some(dir)
            } else {
                None
            };
            for t in &selected {
                let repo = repos
                    .get(&t.repo)
                    .with_context(|| format!("{}: unknown repo", t.id))?;
                let checkout = checkouts.join(&repo.name);
                if !no_map && !maps.contains_key(&t.repo) {
                    let (dir, cost) = runner::build_map(&checkout, repo, &myrn, &cache)?;
                    if cost > 0.0 {
                        println!("project map for {} built for ${cost:.4}", repo.name);
                    }
                    maps.insert(t.repo.clone(), dir);
                }
                let map = maps.get(&t.repo).map(PathBuf::as_path);
                let r = retrieval::one(t, repo, &checkout, &myrn, map, emb.as_deref(), &out)?;
                println!(
                    "{:<48} {:>4} tokens  {:>2} items  {}",
                    r.id,
                    r.tokens,
                    r.items,
                    match (r.hit_rank, r.terms) {
                        (_, Some((f, n))) => format!("{f}/{n} answer terms"),
                        (Some(k), _) => format!("right file at rank {k}"),
                        (None, _) => "right file missed".into(),
                    }
                );
                if show {
                    println!("{}\n", indent(&r.text));
                }
                rows.push(r);
            }
            if !keep {
                let _ = std::fs::remove_dir_all(out.join("work"));
                let _ = std::fs::remove_dir_all(out.join("home"));
            }
            let text = retrieval::render(&rows);
            std::fs::write(out.join("retrieval.md"), &text)?;
            println!("\n{}", text.lines().last().unwrap_or(""));
            println!("report: {}", out.join("retrieval.md").display());
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Report { dir, rescore } => {
            if rescore {
                rescore_runs(&absolute(&dir)?, &tasks)?;
            }
            write_report(&dir)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn append(path: &Path, rec: &RunRecord) -> Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(f, "{}", serde_json::to_string(rec)?)?;
    Ok(())
}

/// Re-parse each run's raw transcript with the current metrics code. Tokens,
/// success and injection data are kept; tool-use metrics are recomputed.
fn rescore_runs(dir: &Path, tasks: &[task::Task]) -> Result<()> {
    let path = dir.join("results.jsonl");
    let text = std::fs::read_to_string(&path)?;
    let mut out = String::new();
    let mut n = 0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let mut rec: RunRecord = serde_json::from_str(line)?;
        let raw = dir.join("raw").join(format!(
            "{}--{}--{}.jsonl",
            rec.task.replace('/', "_"),
            rec.condition.as_str(),
            rec.run
        ));
        if let (Ok(stream), Some(task)) = (
            std::fs::read_to_string(&raw),
            tasks.iter().find(|t| t.id == rec.task),
        ) {
            let stream = stream.split("\n--- stderr ---\n").next().unwrap_or("");
            let root = dir.join("work").join(&rec.repo);
            let s = metrics::parse(stream, &root.to_string_lossy(), &task.touches);
            rec.session.exploration_before_edit = s.exploration_before_edit;
            rec.session.exploration_total = s.exploration_total;
            rec.session.edits = s.edits;
            rec.session.shell_edits = s.shell_edits;
            rec.session.tool_calls = s.tool_calls;
            rec.session.myrn_tool_calls = s.myrn_tool_calls;
            rec.session.asked_to_record = s.asked_to_record;
            rec.session.record_tokens = s.record_tokens;
            rec.session.record_calls = s.record_calls;
            rec.session.record_rejected = s.record_rejected;
            if s.asked_to_record {
                rec.session.answer = s.answer;
            }
            n += 1;
        }
        out.push_str(&serde_json::to_string(&rec)?);
        out.push('\n');
    }
    std::fs::write(&path, out)?;
    println!("rescored {n} runs from raw transcripts");
    Ok(())
}

fn write_report(dir: &Path) -> Result<()> {
    let text = std::fs::read_to_string(dir.join("results.jsonl"))
        .with_context(|| format!("reading {}/results.jsonl", dir.display()))?;
    let runs: Vec<RunRecord> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    let md = report::render(&runs, "Gate 1: paired benchmark");
    let path = dir.join("report.md");
    std::fs::write(&path, &md)?;
    let g = report::gate(&runs);
    println!("\nGate 1: {:?}: {}", g.verdict, g.reason);
    println!("report: {}", path.display());
    Ok(())
}

/// Git and the agent run in other directories, so every path must be absolute.
fn absolute(p: &Path) -> Result<PathBuf> {
    Ok(if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    })
}

fn indent(s: &str) -> String {
    s.lines().map(|l| format!("    {l}\n")).collect()
}
