//! myrn verify: re-check records against the current code. Promotes records
//! whose evidence now holds, lists the suspect ones with what changed, and
//! confirms one with `--confirm <id>`.

use std::process::ExitCode;

use anyhow::Result;
use serde::Serialize;

use super::Ctx;

pub fn run(ctx: &Ctx, confirm: Option<&str>) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    if let Some(id) = confirm {
        let c = engine.confirm(id)?;
        return ctx.emit(&c, || {
            format!(
                "{} confirmed for the current code: {} anchor(s) refreshed, status {}.\n",
                c.id, c.anchors_refreshed, c.status
            )
        });
    }
    #[derive(Serialize)]
    struct Out {
        verified: Vec<myrn_engine::validity::states::Promotion>,
        suspect: Vec<myrn_engine::validity::states::SuspectRecord>,
    }
    let out = Out {
        verified: engine.promote()?,
        suspect: engine.suspects()?,
    };
    ctx.emit(&out, || {
        let mut s = String::new();
        for p in &out.verified {
            s += &format!("verified  {}  ({})\n", p.id, p.reason);
        }
        if out.suspect.is_empty() {
            s += "No suspect records: every record matches the current code.\n";
            return s;
        }
        if !out.verified.is_empty() {
            s.push('\n');
        }
        s += &format!(
            "{} record(s) written about code that has changed since:\n\n",
            out.suspect.len()
        );
        for r in &out.suspect {
            s += &format!("suspect   {}  {}  {}\n", r.id, r.ntype, r.title);
            for c in &r.changed {
                let what = if c.gone { "gone" } else { "changed" };
                s += &format!(
                    "          {what}: {}  {}\n",
                    c.name,
                    c.location.as_deref().unwrap_or("")
                );
            }
        }
        s += "\nIf a record still holds: `myrn verify --confirm <id>` (or the agent's `confirm` tool).\n\
              If it does not: the agent's `supersede` tool replaces it with a corrected record.\n\
              Suspect records stay ranked down and labelled until then.\n";
        s
    })
}
