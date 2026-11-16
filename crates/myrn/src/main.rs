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
    /// Diagnose store, index and setup problems
    Doctor,
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
        Command::Doctor => cmd::doctor::run(&ctx),
        Command::Explain { symbol } => cmd::query::explain(&ctx, &symbol),
        Command::Impact { symbol } => cmd::query::impact(&ctx, &symbol),
        Command::History { path, limit } => cmd::query::history(&ctx, &path, limit),
        Command::Context { task, budget } => cmd::query::context(&ctx, &task.join(" "), budget),
        Command::Brief { budget } => cmd::query::brief(&ctx, budget),
        Command::Mcp => cmd::mcp::run(&ctx),
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("myrn: {e:#}");
            ExitCode::FAILURE
        }
    }
}
