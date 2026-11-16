//! myrn doctor: diagnose store, index and setup problems.

use std::process::ExitCode;

use anyhow::Result;
use myrn_engine::api::{Engine, Level};

use super::Ctx;

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let mut engine = Engine::open(&cwd, ctx.data_dir.as_deref())?;
    let checks = engine.doctor();
    let failed = checks.iter().any(|c| c.level == Level::Fail);
    let _ = ctx.emit(&checks, || {
        let mut out = String::new();
        for c in &checks {
            let mark = match c.level {
                Level::Ok => "ok  ",
                Level::Warn => "warn",
                Level::Fail => "FAIL",
            };
            out += &format!("{mark}  {:<15} {}\n", c.name, c.detail);
        }
        out
    })?;
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
