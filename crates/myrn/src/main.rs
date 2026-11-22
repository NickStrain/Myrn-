//! `myrn` command-line entry point. Also runs the MCP server (`myrn mcp`).
//!
//! Design doc: "CLI".

mod cmd;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "myrn",
    version,
    about = "Persistent project memory for AI coding agents"
)]
struct Cli {
    /// Where private data is kept (default: the platform's per-user data directory, or MYRN_HOME).
    #[arg(long, global = true, hide = true, env = "MYRN_HOME")]
    data_dir: Option<PathBuf>,

    /// Print JSON instead of text.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Index the repository and create the local store
    Init,
    /// Index health, record counts by state, connected agents
    Status,
    /// One node with its fields and relations
    Show {
        /// Node id, symbol name, qualified name, path or path:line
        id: String,
    },
    /// Recreate the index from the code
    Rebuild,
    /// Write the project map: what the project, each directory, file and function is for
    Summarize {
        /// Only show what is missing or out of date
        #[arg(long)]
        dry_run: bool,
        /// Send at most this many files to the model in this run
        #[arg(long)]
        limit: Option<usize>,
        /// Model calls at the same time (default from .myrn/project.toml)
        #[arg(long, short = 'j')]
        jobs: Option<usize>,
    },
    /// History of sessions and learned records, newest first; one session in full with its id
    Log {
        /// Session id or unique prefix
        session: Option<String>,
        /// Maximum entries
        #[arg(short = 'n', long, default_value_t = 20)]
        limit: usize,
        /// Only this type: episode, bug, root_cause, decision, workflow, fact, summary
        #[arg(long = "type", value_name = "TYPE")]
        ntype: Option<String>,
        /// Only records in this state: unverified, verified, suspect, superseded
        #[arg(long)]
        status: Option<String>,
        /// Include the project map's summaries
        #[arg(long)]
        map: bool,
    },
    /// Where a record came from: session, commits, tests, code it is anchored to
    Trace {
        /// Record id
        id: String,
    },
    /// How shared knowledge changed between two points (revisions or YYYY-MM-DD)
    Diff {
        /// Start point (default: HEAD)
        from: Option<String>,
        /// End point (default: the working tree)
        to: Option<String>,
    },
    /// Permanently remove a record and anything derived only from it
    Forget {
        /// Record id
        id: String,
        /// Don't ask for confirmation
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Merge duplicate note copies, drop index entries for deleted code, compact old sessions
    Gc {
        /// Compact session logs older than this many days
        #[arg(long, default_value_t = 30)]
        older_than: u64,
        /// Only report what would change
        #[arg(long)]
        dry_run: bool,
    },
    /// Diagnose store, index and setup problems
    Doctor,
    /// Re-check records against the current code: promote, list suspect ones, confirm one
    Verify {
        /// Confirm that this suspect record still holds for the current code
        #[arg(long, value_name = "ID")]
        confirm: Option<String>,
    },
    /// What a symbol or file is, who calls it, which tests cover it
    Explain {
        /// Symbol name, qualified name (a::b or A.b), path or path:line
        symbol: String,
    },
    /// What depends on a symbol or file, and which tests reach it
    Impact {
        /// Symbol name, qualified name, path or path:line
        symbol: String,
    },
    /// Commits that changed a file, directory or symbol
    History {
        /// File path, directory or symbol
        path: String,
        /// Maximum commits
        #[arg(short = 'n', long, default_value_t = 15)]
        limit: usize,
    },
    /// The context pack an agent would get for a task
    Context {
        /// The task, in words
        #[arg(required = true, num_args = 1..)]
        task: Vec<String>,
        /// Token budget (default from project.toml)
        #[arg(long)]
        budget: Option<usize>,
    },
    /// The project brief an agent gets at session start
    Brief {
        #[arg(long)]
        budget: Option<usize>,
    },
    /// Run the MCP server over stdio (started by the agent)
    Mcp,
    /// Explain session recording, ask consent, and install agent hooks and the MCP server
    Connect {
        /// Every agent found on this machine
        #[arg(long)]
        all: bool,
        /// A specific agent: claude or codex (repeatable)
        #[arg(long = "agent", value_name = "NAME")]
        agents: Vec<String>,
        /// Only the MCP server and an instruction-file line; no hooks, nothing recorded
        #[arg(long)]
        pull_only: bool,
        /// Consent without the interactive prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Remove everything `connect` installed
    Disconnect {
        /// A specific agent (default: all connected)
        #[arg(long = "agent", value_name = "NAME")]
        agents: Vec<String>,
    },
    /// Entry point for agent hooks (reads the hook payload on stdin)
    #[command(hide = true)]
    Hook { agent: String, event: String },
    /// Developer tools
    Debug {
        #[command(subcommand)]
        command: DebugCommand,
    },
}

#[derive(Subcommand)]
enum DebugCommand {
    /// Gate 2: a checklist of learned records to mark correct / vague / wrong,
    /// or (with --score) the precision of a marked checklist
    Review {
        /// Records to sample (0 = all)
        #[arg(long, default_value_t = 30)]
        sample: usize,
        /// Sampling seed, so the same sample can be drawn again
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Write the checklist here instead of stdout
        #[arg(long)]
        out: Option<std::path::PathBuf>,
        /// Read a marked checklist and report precision
        #[arg(long, value_name = "FILE")]
        score: Option<std::path::PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let ctx = cmd::Ctx {
        data_dir: cli.data_dir,
        json: cli.json,
    };
    let result = match cli.command {
        Command::Init => cmd::init::run(&ctx),
        Command::Status => cmd::status::run(&ctx),
        Command::Show { id } => cmd::show::run(&ctx, &id),
        Command::Rebuild => cmd::rebuild::run(&ctx),
        Command::Summarize {
            dry_run,
            limit,
            jobs,
        } => cmd::summarize::run(&ctx, dry_run, limit, jobs),
        Command::Doctor => cmd::doctor::run(&ctx),
        Command::Verify { confirm } => cmd::verify::run(&ctx, confirm.as_deref()),
        Command::Explain { symbol } => cmd::query::explain(&ctx, &symbol),
        Command::Impact { symbol } => cmd::query::impact(&ctx, &symbol),
        Command::History { path, limit } => cmd::query::history(&ctx, &path, limit),
        Command::Context { task, budget } => cmd::query::context(&ctx, &task.join(" "), budget),
        Command::Brief { budget } => cmd::query::brief(&ctx, budget),
        Command::Mcp => cmd::mcp::run(&ctx),
        Command::Log {
            session,
            limit,
            ntype,
            status,
            map,
        } => cmd::log::run(
            &ctx,
            session.as_deref(),
            limit,
            ntype.as_deref(),
            status.as_deref(),
            map,
        ),
        Command::Trace { id } => cmd::trace::run(&ctx, &id),
        Command::Diff { from, to } => cmd::diff::run(&ctx, from.as_deref(), to.as_deref()),
        Command::Forget { id, yes } => cmd::forget::run(&ctx, &id, yes),
        Command::Gc {
            older_than,
            dry_run,
        } => cmd::gc::run(&ctx, older_than, dry_run),
        Command::Connect {
            all,
            agents,
            pull_only,
            yes,
        } => cmd::connect::run(
            &ctx,
            cmd::connect::Args {
                all,
                agents,
                pull_only,
                yes,
            },
        ),
        Command::Disconnect { agents } => cmd::disconnect::run(&ctx, agents),
        Command::Hook { agent, event } => cmd::hook::run(&ctx, &agent, &event),
        Command::Debug {
            command:
                DebugCommand::Review {
                    sample,
                    seed,
                    out,
                    score,
                },
        } => cmd::debug::review(&ctx, sample, seed, out.as_deref(), score.as_deref()),
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("myrn: {e:#}");
            ExitCode::FAILURE
        }
    }
}
