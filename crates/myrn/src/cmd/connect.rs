//! myrn connect: explain session recording, ask consent, install adapters.

use std::io::{BufRead, IsTerminal, Write};
use std::process::ExitCode;

use anyhow::{bail, Result};
use myrn_adapters::{Adapter, InstallCtx, Mode};
use myrn_store::state::AgentState;

use super::Ctx;

pub struct Args {
    pub all: bool,
    pub agents: Vec<String>,
    pub pull_only: bool,
    pub yes: bool,
    /// Also register the MCP server in hooks mode.
    pub mcp: bool,
}

/// The install context for this repository and this binary.
pub fn install_ctx(root: &std::path::Path, mcp: bool) -> Result<InstallCtx> {
    let exe = std::env::current_exe()?;
    Ok(InstallCtx {
        repo_root: root.to_path_buf(),
        binary: exe.canonicalize().unwrap_or(exe),
        run_agent_cli: std::env::var_os("MYRN_NO_AGENT_CLI").is_none(),
        mcp,
    })
}

fn choose(args: &Args) -> Result<Vec<Box<dyn Adapter>>> {
    if !args.agents.is_empty() {
        return args
            .agents
            .iter()
            .map(|n| {
                myrn_adapters::by_name(n)
                    .ok_or_else(|| anyhow::anyhow!("unknown agent `{n}`; known: claude, codex"))
            })
            .collect();
    }
    let found: Vec<Box<dyn Adapter>> = myrn_adapters::all()
        .into_iter()
        .filter(|a| a.detect().is_some())
        .collect();
    match found.len() {
        0 => bail!("no supported agent found (Claude Code, Codex); use `--agent claude` or `--agent codex` to install anyway"),
        1 => Ok(found),
        _ if args.all => Ok(found),
        _ => {
            let names: Vec<&str> = found.iter().map(|a| a.name()).collect();
            bail!("found {}; pick with `--agent <name>` or use `--all`", names.join(" and "))
        }
    }
}

const EXPLAIN: &str = "\
Session hooks let myrn see what your coding agent does in this repository:
  - when a session starts and ends, and the prompts you type
  - which files the agent reads and edits, the commands it runs, and the errors it hits
What it does NOT record: the model's replies or its full transcript.
Where it goes: only this machine, in myrn's private data directory. Nothing is sent
anywhere, and paths in .myrn/ignore are never recorded. Secrets are redacted before
anything is written.
Automatic context injection stays OFF ([context] inject = false in .myrn/project.toml);
the hooks refresh the index and record the above. See it with `myrn log`.
Undo at any time with `myrn disconnect`.";

fn ask(question: &str) -> Result<bool> {
    if !std::io::stdin().is_terminal() {
        return Ok(false);
    }
    print!("{question} [y/N] ");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(matches!(line.trim().to_lowercase().as_str(), "y" | "yes"))
}

pub fn run(ctx: &Ctx, args: Args) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let engine = myrn_engine::api::Engine::open(&cwd, ctx.data_dir.as_deref())?;
    let adapters = choose(&args)?;
    let mode = if args.pull_only {
        Mode::PullOnly
    } else {
        Mode::Hooks
    };
    let mut state = engine.state()?;

    if mode == Mode::Hooks && !state.capture_consent {
        println!("{EXPLAIN}\n");
        let names: Vec<&str> = adapters.iter().map(|a| a.display_name()).collect();
        let yes = args.yes
            || ask(&format!(
                "Install session hooks for {}?",
                names.join(" and ")
            ))?;
        if !yes {
            println!("Nothing installed. `myrn connect --pull-only` gives the agent the MCP tools without hooks.");
            return Ok(ExitCode::FAILURE);
        }
        state.capture_consent = true;
        state.consent_time = Some(now());
    }

    let ictx = install_ctx(engine.root(), args.mcp)?;
    for a in &adapters {
        let r = a.install(&ictx, mode)?;
        println!(
            "{} ({}):",
            a.display_name(),
            mode.as_str().replace('_', "-")
        );
        for c in &r.changes {
            println!("  {:<34} {}", c.path, c.what);
        }
        for m in &r.manual {
            println!("  to do: {m}");
        }
        let prev_created = state
            .agents
            .get(a.name())
            .map(|s| s.created.clone())
            .unwrap_or_default();
        let mut created = prev_created;
        created.extend(r.created);
        created.dedup();
        state.agents.insert(
            a.name().to_string(),
            AgentState {
                mode: mode.as_str().into(),
                connected_time: now(),
                written: r.written,
                created,
                mcp: ictx.wants_mcp(mode),
            },
        );
    }
    engine.save_state(&state)?;
    println!("\nCheck the setup any time with `myrn doctor`.");
    Ok(ExitCode::SUCCESS)
}

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
