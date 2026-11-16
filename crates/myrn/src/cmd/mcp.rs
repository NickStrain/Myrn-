//! myrn mcp: run the MCP server over stdio.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    myrn_mcp::serve(engine)?;
    Ok(ExitCode::SUCCESS)
}
