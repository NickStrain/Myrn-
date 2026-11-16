//! explain, impact, history, context, brief: the same answers the MCP tools give.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn explain(ctx: &Ctx, symbol: &str) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let r = engine.explain(symbol)?;
    ctx.emit(&r, || r.render(|e| e.render()))
}

pub fn impact(ctx: &Ctx, symbol: &str) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let r = engine.impact(symbol)?;
    ctx.emit(&r, || r.render(|i| i.render(25)))
}

pub fn history(ctx: &Ctx, path: &str, limit: usize) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let r = engine.history(path, limit)?;
    ctx.emit(&r, || r.render(|e| myrn_mcp::tools::history::render(e)))
}

pub fn context(ctx: &Ctx, task: &str, budget: Option<usize>) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let p = engine.context(task, budget)?;
    ctx.emit(&p, || {
        if p.text.is_empty() {
            "Nothing in the code graph is clearly relevant; an agent would get no injected context.\n".into()
        } else {
            format!("{}\n[{} tokens, {} items]\n", p.text, p.tokens, p.items.len())
        }
    })
}

pub fn brief(ctx: &Ctx, budget: Option<usize>) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let p = engine.brief(budget)?;
    ctx.emit(&p, || format!("{}\n[{} tokens]\n", p.text, p.tokens))
}
