//! myrn doctor: diagnose store, index, hooks and adapter setup.

use std::process::ExitCode;

use anyhow::Result;
use myrn_adapters::Mode;
use myrn_engine::api::{Check, Engine, Level};

use super::{connect::install_ctx, Ctx};

pub fn run(ctx: &Ctx) -> Result<ExitCode> {
    let cwd = std::env::current_dir()?;
    let mut engine = Engine::open(&cwd, ctx.data_dir.as_deref())?;
    let mut checks = engine.doctor();
    checks.retain(|c| c.name != "agents");
    checks.extend(agent_checks(&engine)?);
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

fn agent_checks(engine: &Engine) -> Result<Vec<Check>> {
    let state = engine.state()?;
    let mut out = Vec::new();
    if state.agents.is_empty() {
        out.push(Check {
            name: "agents".into(),
            level: Level::Warn,
            detail: "none connected; run `myrn connect`".into(),
        });
        return Ok(out);
    }
    let ictx = install_ctx(engine.root())?;
    for (name, s) in &state.agents {
        let Some(a) = myrn_adapters::by_name(name) else {
            continue;
        };
        let mode = Mode::parse(&s.mode);
        out.push(Check {
            name: a.name().into(),
            level: Level::Ok,
            detail: format!(
                "connected ({}), wrote: {}",
                s.mode.replace('_', "-"),
                s.written.join(", ")
            ),
        });
        for f in a.inspect(&ictx, mode) {
            out.push(Check {
                name: a.name().into(),
                level: if f.ok { Level::Ok } else { Level::Fail },
                detail: f.what,
            });
        }
    }
    out.push(Check {
        name: "recording".into(),
        level: Level::Ok,
        detail: if state.capture_consent {
            "consent given".into()
        } else {
            "no consent; hooks not installed".into()
        },
    });
    out.push(Check {
        name: "injection".into(),
        level: Level::Ok,
        detail: if engine.config.context.inject {
            "on".into()
        } else {
            "off ([context] inject = false)".into()
        },
    });
    let mut ms: Vec<u64> = state.hook_timings.iter().map(|t| t.millis).collect();
    if ms.is_empty() {
        out.push(Check {
            name: "hook latency".into(),
            level: Level::Ok,
            detail: "no hook calls yet".into(),
        });
    } else {
        ms.sort_unstable();
        let p50 = ms[ms.len() / 2];
        let max = *ms.last().unwrap();
        out.push(Check {
            name: "hook latency".into(),
            level: if p50 > 500 { Level::Warn } else { Level::Ok },
            detail: format!(
                "median {p50} ms, max {max} ms over the last {} calls",
                ms.len()
            ),
        });
    }
    Ok(out)
}
