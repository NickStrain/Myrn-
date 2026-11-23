//! Run each task N times per condition with the same agent and model.
//!
//! Every run starts from a fresh clone of the pinned commit with the task's
//! setup folded into HEAD, a fresh myrn data directory, and Claude Code in
//! headless mode with user-level settings and MCP servers switched off, so
//! the only difference between conditions is myrn:
//!
//! - `without`: no hooks, no MCP servers.
//! - `off`: `myrn init`, `myrn connect` (hooks: recording, records requested at
//!   the end), myrn's MCP server, injection off.
//! - `with`: the same with injection on.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::metrics::{self, Session};
use crate::task::{Kind, Repo, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Without,
    /// myrn connected, injection off.
    Off,
    /// myrn connected, injection on.
    With,
    /// Injection on with the fixed ranking (no learning from past use).
    Fixed,
}

impl Condition {
    pub fn as_str(self) -> &'static str {
        match self {
            Condition::Without => "without",
            Condition::Off => "off",
            Condition::With => "with",
            Condition::Fixed => "fixed",
        }
    }
    /// Column heading in the report.
    pub fn label(self) -> &'static str {
        match self {
            Condition::Without => "Without myrn",
            Condition::Off => "myrn, inject off",
            Condition::With => "myrn, inject on",
            Condition::Fixed => "myrn, inject on, fixed ranking",
        }
    }
    pub fn uses_myrn(self) -> bool {
        self != Condition::Without
    }
    pub fn parse(s: &str) -> Result<Condition> {
        match s {
            "without" => Ok(Condition::Without),
            "off" => Ok(Condition::Off),
            "with" => Ok(Condition::With),
            "fixed" => Ok(Condition::Fixed),
            _ => bail!("unknown condition {s}; use without, off, with or fixed"),
        }
    }
}

/// One line of `results.jsonl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub task: String,
    pub repo: String,
    pub kind: Kind,
    pub condition: Condition,
    pub run: usize,
    pub model: Option<String>,
    pub started: i64,
    pub success: bool,
    pub timed_out: bool,
    pub wall_ms: u64,
    pub session: Session,
    /// Node ids myrn injected (with-condition only).
    pub injected: Vec<String>,
    /// Injected items whose file the agent then read or edited.
    pub injected_used: usize,
    /// Prompt-hook latencies.
    pub hook_ms: Vec<u64>,
    /// Learned records among the injected items, and how many of those were
    /// suspect (written for code that has changed) when injected.
    #[serde(default)]
    pub injected_records: usize,
    #[serde(default)]
    pub injected_stale: usize,
    /// Learned records in myrn's store after the session (exported to `records/`).
    #[serde(default)]
    pub records: usize,
    pub check_output: String,
    pub error: Option<String>,
}

pub struct Tools {
    pub claude: PathBuf,
    pub myrn: PathBuf,
    pub model: Option<String>,
    pub permission_mode: String,
    /// Project map (summaries directory) per repository, copied into myrn runs.
    pub maps: std::collections::HashMap<String, PathBuf>,
    /// Keep one myrn store per repository and condition for the whole run, so
    /// usage learned in one session ranks the next (Step 20 gate).
    pub shared_store: bool,
    /// The embedding model's directory, copied into myrn runs to turn semantic search on.
    pub embeddings: Option<PathBuf>,
}

pub struct Paths {
    pub checkouts: PathBuf,
    /// Scratch space: the per-run clone and myrn data directory.
    pub work: PathBuf,
    /// Shared build cache per repository, so runs don't rebuild from scratch.
    pub target: PathBuf,
}

// ------------------------------------------------------------ workspace

fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let o = Command::new("git")
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("running git {}", args.join(" ")))?;
    if !o.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&o.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn git_apply(dir: &Path, patch: &str, reverse: bool) -> Result<()> {
    let mut args = vec!["apply", "--whitespace=nowarn"];
    if reverse {
        args.push("-R");
    }
    let mut child = Command::new("git")
        .args(&args)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(patch.as_bytes())?;
    let o = child.wait_with_output()?;
    if !o.status.success() {
        bail!(
            "git apply failed: {}",
            String::from_utf8_lossy(&o.stderr).trim()
        );
    }
    Ok(())
}

/// A fresh clone at the pinned commit, with the setup patch folded into HEAD
/// and nothing left that leads back to the unbroken code: no remote, no other
/// branches or tags, no reflog, no unreachable objects.
pub fn prepare(checkout: &Path, repo: &Repo, task: &Task, work: &Path) -> Result<()> {
    if work.exists() {
        std::fs::remove_dir_all(work)?;
    }
    std::fs::create_dir_all(work.parent().unwrap())?;
    let parent = work.parent().unwrap();
    git(
        parent,
        &[
            "clone",
            "-q",
            "--no-hardlinks",
            &checkout.to_string_lossy(),
            &work.to_string_lossy(),
        ],
    )?;
    git(work, &["checkout", "-q", "-B", "bench", &repo.commit])?;
    git(work, &["remote", "remove", "origin"])?;
    for b in git(
        work,
        &["for-each-ref", "--format=%(refname:short)", "refs/heads"],
    )?
    .lines()
    {
        if b != "bench" {
            git(work, &["branch", "-q", "-D", b])?;
        }
    }
    for t in git(work, &["tag", "-l"])?.lines() {
        git(work, &["tag", "-d", t])?;
    }
    if let Some(setup) = &task.setup {
        git_apply(work, &setup.patch, false)?;
        git(work, &["add", "-A"])?;
        let date = git(work, &["log", "-1", "--format=%cI"])?;
        let o = Command::new("git")
            .args(["commit", "-q", "--amend", "--no-edit", "--no-verify"])
            .current_dir(work)
            .env("GIT_COMMITTER_DATE", &date)
            .output()?;
        if !o.status.success() {
            bail!(
                "git commit --amend failed: {}",
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }
    let logs = work.join(".git/logs");
    if logs.exists() {
        std::fs::remove_dir_all(&logs)?;
    }
    git(work, &["gc", "-q", "--prune=now"])?;
    Ok(())
}

/// Write the hidden test files and run the check.
fn run_check(task: &Task, work: &Path, target: &Path) -> Result<(bool, String)> {
    let Some(check) = &task.check else {
        return Ok((true, String::new()));
    };
    for (rel, content) in &check.files {
        let p = work.join(rel);
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d)?;
        }
        std::fs::write(&p, content)?;
    }
    let mut cmd = Command::new("sh");
    cmd.args(["-c", &check.run])
        .current_dir(work)
        .env("CARGO_TARGET_DIR", target)
        .stdin(Stdio::null());
    let (status, out, timed_out) = run_with_timeout(cmd, Duration::from_secs(check.timeout_secs))?;
    let mut out = tail(&out, 3000);
    if timed_out {
        out.push_str("\n[check timed out]");
    }
    Ok((status == Some(0) && !timed_out, out))
}

/// Run a command, capturing stdout and stderr, killing it after `limit`.
fn run_with_timeout(mut cmd: Command, limit: Duration) -> Result<(Option<i32>, String, bool)> {
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("starting process")?;
    let mut so = child.stdout.take().unwrap();
    let mut se = child.stderr.take().unwrap();
    let t1 = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = so.read_to_string(&mut s);
        s
    });
    let t2 = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = se.read_to_string(&mut s);
        s
    });
    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        if let Some(st) = child.try_wait()? {
            break st;
        }
        if start.elapsed() > limit {
            timed_out = true;
            let _ = child.kill();
            break child.wait()?;
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let mut out = t1.join().unwrap_or_default();
    let err = t2.join().unwrap_or_default();
    if !err.is_empty() {
        out.push_str("\n--- stderr ---\n");
        out.push_str(&err);
    }
    Ok((status.code(), out, timed_out))
}

fn tail(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut i = s.len() - max;
    while !s.is_char_boundary(i) {
        i += 1;
    }
    format!("…{}", &s[i..])
}

// ------------------------------------------------------------ one run

pub struct RunSpec<'a> {
    pub task: &'a Task,
    pub repo: &'a Repo,
    pub condition: Condition,
    pub run: usize,
}

pub fn run_one(spec: &RunSpec, tools: &Tools, paths: &Paths, raw_dir: &Path) -> RunRecord {
    let started = now();
    let mut rec = RunRecord {
        task: spec.task.id.clone(),
        repo: spec.repo.name.clone(),
        kind: spec.task.kind,
        condition: spec.condition,
        run: spec.run,
        model: tools.model.clone(),
        started,
        success: false,
        timed_out: false,
        wall_ms: 0,
        session: Session::default(),
        injected: Vec::new(),
        injected_used: 0,
        injected_records: 0,
        injected_stale: 0,
        hook_ms: Vec::new(),
        records: 0,
        check_output: String::new(),
        error: None,
    };
    if let Err(e) = run_inner(spec, tools, paths, raw_dir, &mut rec) {
        rec.error = Some(format!("{e:#}"));
    }
    rec
}

fn run_inner(
    spec: &RunSpec,
    tools: &Tools,
    paths: &Paths,
    raw_dir: &Path,
    rec: &mut RunRecord,
) -> Result<()> {
    let checkout = paths.checkouts.join(&spec.repo.name);
    if !checkout.join(".git").exists() {
        bail!(
            "no checkout at {}; run `myrn-bench fetch`",
            checkout.display()
        );
    }
    let work = paths.work.join(&spec.repo.name);
    let home = if tools.shared_store {
        paths
            .work
            .parent()
            .unwrap_or(&paths.work)
            .join("stores")
            .join(format!("{}--{}", spec.repo.name, spec.condition.as_str()))
    } else {
        paths.work.join("myrn-home")
    };
    let target = paths.target.join(&spec.repo.name);
    std::fs::create_dir_all(&target)?;
    prepare(&checkout, spec.repo, spec.task, &work)?;
    if home.exists() && !tools.shared_store {
        std::fs::remove_dir_all(&home)?;
    }

    let mcp_config = paths.work.join("mcp.json");
    let mcp = match spec.condition {
        Condition::Without => serde_json::json!({ "mcpServers": {} }),
        Condition::Off | Condition::With | Condition::Fixed => {
            let map = tools.maps.get(&spec.repo.name).map(PathBuf::as_path);
            setup_myrn(
                &tools.myrn,
                &work,
                &home,
                spec.condition != Condition::Off,
                spec.condition == Condition::Fixed,
                map,
                tools.embeddings.as_deref(),
            )?;
            serde_json::json!({ "mcpServers": { "myrn": {
                "type": "stdio",
                "command": tools.myrn,
                "args": ["mcp"],
                "env": { "MYRN_HOME": home }
            }}})
        }
    };
    std::fs::write(&mcp_config, serde_json::to_string_pretty(&mcp)?)?;

    let mut cmd = Command::new(&tools.claude);
    cmd.arg("-p")
        .arg(&spec.task.prompt)
        .args(["--output-format", "stream-json", "--verbose"])
        .args(["--no-session-persistence"])
        .args(["--permission-mode", &tools.permission_mode])
        .args(["--setting-sources", "local"])
        .arg("--strict-mcp-config")
        .arg("--mcp-config")
        .arg(&mcp_config);
    if let Some(m) = &tools.model {
        cmd.args(["--model", m]);
    }
    cmd.current_dir(&work)
        .env("MYRN_HOME", &home)
        .env("CARGO_TARGET_DIR", &target)
        .env_remove("CLAUDECODE")
        .env_remove("CLAUDE_CODE_ENTRYPOINT")
        .stdin(Stdio::null());
    let start = Instant::now();
    let (_, stream, timed_out) =
        run_with_timeout(cmd, Duration::from_secs(spec.task.timeout_secs))?;
    rec.wall_ms = start.elapsed().as_millis() as u64;
    rec.timed_out = timed_out;
    let raw = raw_dir.join(format!(
        "{}--{}--{}.jsonl",
        spec.task.id.replace('/', "_"),
        spec.condition.as_str(),
        spec.run
    ));
    std::fs::write(&raw, &stream)?;
    let stream_json: String = stream
        .split("\n--- stderr ---\n")
        .next()
        .unwrap_or("")
        .to_string();
    rec.session = metrics::parse(&stream_json, &work.to_string_lossy(), &spec.task.touches);
    // The API refused the session (no credits, rate limit): not a task failure.
    if rec.session.is_error && rec.session.tokens.total() == 0 {
        bail!(
            "the agent did no work: {}",
            rec.session.answer.lines().next().unwrap_or("")
        );
    }
    if rec.session.session_id.is_none() {
        bail!(
            "no session in the agent's output (see {}): {}",
            raw.display(),
            tail(&stream, 400)
        );
    }

    if spec.condition.uses_myrn() {
        collect_injections(&work, &home, rec)?;
        let records_dir = raw_dir.parent().unwrap_or(raw_dir).join("records");
        rec.records = export_records(&tools.myrn, &work, &home, &records_dir, spec)?;
    }
    if spec.task.kind == Kind::Question {
        rec.success = metrics::answer_ok(&rec.session.answer, &spec.task.answer_contains);
    } else {
        let (ok, out) = run_check(spec.task, &work, &target)?;
        rec.success = ok;
        rec.check_output = out;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn setup_myrn(
    myrn: &Path,
    work: &Path,
    home: &Path,
    inject: bool,
    fixed_ranking: bool,
    map: Option<&Path>,
    embeddings: Option<&Path>,
) -> Result<()> {
    let run = |args: &[&str]| -> Result<()> {
        let o = Command::new(myrn)
            .args(args)
            .current_dir(work)
            .env("MYRN_HOME", home)
            .env("MYRN_NO_AGENT_CLI", "1")
            .stdin(Stdio::null())
            .output()?;
        if !o.status.success() {
            bail!(
                "myrn {} failed: {}{}",
                args.join(" "),
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
        Ok(())
    };
    run(&["init"])?;
    run(&["connect", "--agent", "claude", "--yes"])?;
    // Set injection explicitly (the default changed), and skip the end-of-session
    // record request: its tokens are not part of the comparison, only the bill.
    let cfg = work.join(".myrn/project.toml");
    let mut text = std::fs::read_to_string(&cfg)?;
    let mode = if fixed_ranking {
        "\"fixed\""
    } else {
        "\"learned\""
    };
    for (key, value) in [
        ("inject", inject.to_string()),
        ("ask_at_session_end", "false".to_string()),
        ("mode", mode.to_string()),
    ] {
        let line = text
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("{key} =")))
            .map(str::to_string)
            .with_context(|| format!("{} has no `{key}` line", cfg.display()))?;
        text = text.replace(&line, &format!("{key} = {value}"));
    }
    std::fs::write(&cfg, text)?;
    // The project map, built from the unbroken commit (as if summarized before
    // the bug landed): the broken function's summary shows as possibly outdated.
    if let Some(map) = map {
        copy_dir(map, &work.join(".myrn/knowledge/summaries"))?;
    }
    // Semantic search: the model from this machine, never a download per run.
    if let Some(model) = embeddings {
        let dst = home
            .join("models")
            .join(model.file_name().unwrap_or_default());
        if !dst.join("model.safetensors").exists() {
            copy_dir(model, &dst)?;
        }
        run(&["embeddings", "enable"])?;
    }
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let dst = to.join(e.file_name());
        if e.file_type()?.is_dir() {
            copy_dir(&e.path(), &dst)?;
        } else {
            std::fs::copy(e.path(), &dst)?;
        }
    }
    Ok(())
}

/// Build (once, cached by commit) the project map for a repository at its
/// pinned commit with `myrn summarize`. Returns the summaries directory and
/// what building it cost (0 when cached).
pub fn build_map(
    checkout: &Path,
    repo: &Repo,
    myrn: &Path,
    cache: &Path,
) -> Result<(PathBuf, f64)> {
    let dir = cache.join(format!("{}-{}", repo.name, &repo.commit[..12]));
    let out = dir.join("summaries");
    if out.join("project").exists() {
        return Ok((out, 0.0));
    }
    let src = dir.join("src");
    let home = dir.join("home");
    for d in [&src, &home, &out] {
        if d.exists() {
            std::fs::remove_dir_all(d)?;
        }
    }
    std::fs::create_dir_all(&dir)?;
    git(
        &dir,
        &[
            "clone",
            "-q",
            "--no-hardlinks",
            &checkout.to_string_lossy(),
            &src.to_string_lossy(),
        ],
    )?;
    git(&src, &["checkout", "-q", &repo.commit])?;
    let myrn_cmd = |args: &[&str]| -> Result<std::process::Output> {
        let o = Command::new(myrn)
            .args(args)
            .current_dir(&src)
            .env("MYRN_HOME", &home)
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()?;
        if !o.status.success() {
            bail!(
                "myrn {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&o.stdout)
            );
        }
        Ok(o)
    };
    myrn_cmd(&["init"])?;
    let o = myrn_cmd(&["--json", "summarize", "-j", "6"])?;
    let report: serde_json::Value = serde_json::from_slice(&o.stdout)?;
    copy_dir(&src.join(".myrn/knowledge/summaries"), &out)?;
    std::fs::remove_dir_all(&src)?;
    std::fs::remove_dir_all(&home)?;
    Ok((out, report["cost_usd"].as_f64().unwrap_or(0.0)))
}

/// Save the learned records (with their evidence and code) for the Gate 2
/// review, before the workspace is reused. Returns how many there were.
fn export_records(
    myrn: &Path,
    work: &Path,
    home: &Path,
    dir: &Path,
    spec: &RunSpec,
) -> Result<usize> {
    let o = Command::new(myrn)
        .args(["--json", "debug", "review", "--sample", "0"])
        .current_dir(work)
        .env("MYRN_HOME", home)
        .stdin(Stdio::null())
        .output()?;
    if !o.status.success() {
        bail!(
            "myrn debug review failed: {}",
            String::from_utf8_lossy(&o.stderr)
        );
    }
    let mut items: Vec<myrn_engine::review::ReviewItem> = serde_json::from_slice(&o.stdout)?;
    if items.is_empty() {
        return Ok(0);
    }
    let origin = format!(
        "{} ({}, run {})",
        spec.task.id,
        spec.condition.as_str(),
        spec.run
    );
    for it in &mut items {
        it.origin = origin.clone();
    }
    std::fs::create_dir_all(dir)?;
    let file = dir.join(format!(
        "{}--{}--{}.json",
        spec.task.id.replace('/', "_"),
        spec.condition.as_str(),
        spec.run
    ));
    std::fs::write(file, serde_json::to_string_pretty(&items)?)?;
    Ok(items.len())
}

/// What myrn injected in this run, and whether the agent then opened it.
fn collect_injections(work: &Path, home: &Path, rec: &mut RunRecord) -> Result<()> {
    let engine = myrn_engine::api::Engine::open(work, Some(home))?;
    let mut stmt = engine
        .db()
        .conn
        .prepare("SELECT DISTINCT note_id FROM usage WHERE injected = 1 AND session_id = ?1")?;
    let session = rec.session.session_id.clone().unwrap_or_default();
    rec.injected = stmt
        .query_map([&session], |r| r.get::<_, String>(0))?
        .collect::<rusqlite::Result<_>>()?;
    let answer = rec.session.answer.to_lowercase();
    rec.injected_used = rec
        .injected
        .iter()
        .filter(|id| {
            item_path(id).is_some_and(|p| rec.session.files_seen.contains(p))
                || item_name(id).is_some_and(|n| n.len() >= 4 && answer.contains(&n.to_lowercase()))
        })
        .count();
    let (records, stale): (i64, i64) = engine.db().conn.query_row(
        "SELECT COUNT(DISTINCT u.note_id), COUNT(DISTINCT CASE WHEN u.status = 'suspect' THEN u.note_id END)
         FROM usage u JOIN nodes n ON n.id = u.note_id
         WHERE u.injected = 1 AND n.origin = 'earned' AND u.session_id = ?1",
        [&session],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    rec.injected_records = records as usize;
    rec.injected_stale = stale as usize;
    rec.hook_ms = engine
        .state()?
        .hook_timings
        .iter()
        .filter(|t| t.event == "UserPromptSubmit" && t.time >= rec.started)
        .map(|t| t.millis)
        .collect();
    Ok(())
}

/// `sym:src/a.rs#m::T::f` -> `f`
fn item_name(id: &str) -> Option<&str> {
    let (_, q) = id.split_once('#')?;
    let q = q.split('#').next().unwrap_or(q);
    q.rsplit([':', '.']).next()
}

/// `sym:src/a.rs#f` -> `src/a.rs`
fn item_path(id: &str) -> Option<&str> {
    let (_, rest) = id.split_once(':')?;
    Some(rest.split('#').next().unwrap_or(rest))
}

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// ------------------------------------------------------------ validate

#[derive(Debug, Serialize)]
pub struct Validation {
    pub task: String,
    pub broken_fails: Option<bool>,
    pub solution_passes: Option<bool>,
    pub note: String,
}

/// Check a task without an agent: the check must fail on the prepared
/// workspace and pass once the reference solution is applied.
pub fn validate(task: &Task, repo: &Repo, paths: &Paths) -> Validation {
    let mut v = Validation {
        task: task.id.clone(),
        broken_fails: None,
        solution_passes: None,
        note: String::new(),
    };
    if task.kind == Kind::Question {
        v.note = format!("answer must mention: {}", task.answer_contains.join(", "));
        return v;
    }
    let r = (|| -> Result<()> {
        let checkout = paths.checkouts.join(&repo.name);
        let work = paths.work.join(&repo.name);
        let target = paths.target.join(&repo.name);
        std::fs::create_dir_all(&target)?;
        prepare(&checkout, repo, task, &work)?;
        let status = git(&work, &["status", "--porcelain"])?;
        if !status.is_empty() {
            bail!("workspace not clean after setup: {status}");
        }
        let (ok, out) = run_check(task, &work, &target)?;
        v.broken_fails = Some(!ok);
        if ok {
            v.note = format!("check passes before any fix:\n{}", tail(&out, 600));
        }
        match (&task.solution, &task.setup) {
            (Some(s), _) => git_apply(&work, &s.patch, false)?,
            (None, Some(s)) => git_apply(&work, &s.patch, true)?,
            (None, None) => return Err(anyhow!("no solution to apply")),
        }
        let (ok, out) = run_check(task, &work, &target)?;
        v.solution_passes = Some(ok);
        if !ok {
            v.note = format!("check fails with the solution:\n{}", tail(&out, 1200));
        }
        Ok(())
    })();
    if let Err(e) = r {
        v.note = format!("{e:#}");
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_paths() {
        assert_eq!(item_path("sym:src/a.rs#m::f"), Some("src/a.rs"));
        assert_eq!(item_path("file:src/a.rs"), Some("src/a.rs"));
        assert_eq!(item_path("nope"), None);
        assert_eq!(item_name("sym:src/a.rs#m::T::refresh"), Some("refresh"));
        assert_eq!(
            item_name("sym:a.py#pkg.Session.is_fresh#2"),
            Some("is_fresh")
        );
        assert_eq!(item_name("file:src/a.rs"), None);
    }

    #[test]
    fn tails_respect_char_boundaries() {
        assert_eq!(tail("abc", 5), "abc");
        assert_eq!(tail("aé", 1), "…");
        assert_eq!(tail("abcdef", 3), "…def");
    }
}
