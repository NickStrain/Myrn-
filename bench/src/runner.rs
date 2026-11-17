//! Run each task N times per condition with the same agent and model.
//!
//! Every run starts from a fresh clone of the pinned commit with the task's
//! setup folded into HEAD, a fresh myrn data directory, and Claude Code in
//! headless mode with user-level settings and MCP servers switched off, so
//! the only difference between conditions is myrn:
//!
//! - `without`: no hooks, no MCP servers.
//! - `with`: `myrn init`, `myrn connect` (hooks), injection on, and myrn's MCP
//!   server.

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
    With,
}

impl Condition {
    pub fn as_str(self) -> &'static str {
        match self {
            Condition::Without => "without",
            Condition::With => "with",
        }
    }
    pub fn parse(s: &str) -> Result<Condition> {
        match s {
            "without" => Ok(Condition::Without),
            "with" => Ok(Condition::With),
            _ => bail!("unknown condition {s}; use `with` or `without`"),
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
    pub check_output: String,
    pub error: Option<String>,
}

pub struct Tools {
    pub claude: PathBuf,
    pub myrn: PathBuf,
    pub model: Option<String>,
    pub permission_mode: String,
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
        hook_ms: Vec::new(),
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
    let home = paths.work.join("myrn-home");
    let target = paths.target.join(&spec.repo.name);
    std::fs::create_dir_all(&target)?;
    prepare(&checkout, spec.repo, spec.task, &work)?;
    if home.exists() {
        std::fs::remove_dir_all(&home)?;
    }

    let mcp_config = paths.work.join("mcp.json");
    let mcp = match spec.condition {
        Condition::Without => serde_json::json!({ "mcpServers": {} }),
        Condition::With => {
            setup_myrn(&tools.myrn, &work, &home)?;
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
        .args(["--setting-sources", "project,local"])
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
    if rec.session.session_id.is_none() {
        bail!(
            "no session in the agent's output (see {}): {}",
            raw.display(),
            tail(&stream, 400)
        );
    }

    if spec.condition == Condition::With {
        collect_injections(&work, &home, rec)?;
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

fn setup_myrn(myrn: &Path, work: &Path, home: &Path) -> Result<()> {
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
    let cfg = work.join(".myrn/project.toml");
    let text = std::fs::read_to_string(&cfg)?;
    if !text.contains("inject = false") {
        bail!(
            "{} has no `inject = false` line to switch on",
            cfg.display()
        );
    }
    std::fs::write(&cfg, text.replace("inject = false", "inject = true"))?;
    Ok(())
}

/// What myrn injected in this run, and whether the agent then opened it.
fn collect_injections(work: &Path, home: &Path, rec: &mut RunRecord) -> Result<()> {
    let engine = myrn_engine::api::Engine::open(work, Some(home))?;
    let mut stmt = engine
        .db()
        .conn
        .prepare("SELECT DISTINCT note_id FROM usage WHERE injected = 1")?;
    rec.injected = stmt
        .query_map([], |r| r.get::<_, String>(0))?
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
    rec.hook_ms = engine
        .state()?
        .hook_timings
        .iter()
        .filter(|t| t.event == "UserPromptSubmit")
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
