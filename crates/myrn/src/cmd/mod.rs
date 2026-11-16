//! One module per subcommand.

pub mod connect;
pub mod disconnect;
pub mod doctor;
pub mod hook;
pub mod init;
pub mod mcp;
pub mod query;
pub mod rebuild;
pub mod show;
pub mod status;

// Not built yet; kept to fix the layout.
pub mod diff;
pub mod forget;
pub mod gc;
pub mod log;
pub mod publish;
pub mod trace;
pub mod verify;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::api::Engine;
use serde::Serialize;

pub struct Ctx {
    pub data_dir: Option<PathBuf>,
    pub json: bool,
}

impl Ctx {
    /// Open the project and bring the index up to date, as every command does.
    pub fn engine(&self) -> Result<Engine> {
        let cwd = std::env::current_dir()?;
        let mut e = Engine::open(&cwd, self.data_dir.as_deref())?;
        e.refresh()?;
        Ok(e)
    }

    pub fn emit<T: Serialize>(&self, value: &T, text: impl FnOnce() -> String) -> Result<ExitCode> {
        if self.json {
            println!("{}", serde_json::to_string_pretty(value)?);
        } else {
            print!("{}", text());
        }
        Ok(ExitCode::SUCCESS)
    }
}

/// `1234567` -> `1,234,567`
pub fn thousands(n: i64) -> String {
    let s = n.abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}
