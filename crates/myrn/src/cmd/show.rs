//! myrn show <id>: one node with its fields and relations.

use std::process::ExitCode;

use anyhow::Result;

use super::Ctx;

pub fn run(ctx: &Ctx, id: &str) -> Result<ExitCode> {
    let engine = ctx.engine()?;
    let found = engine.show(id)?;
    ctx.emit(&found, || {
        found.render(|(n, edges)| {
            let mut out = String::new();
            let mut field = |k: &str, v: Option<String>| {
                if let Some(v) = v.filter(|v| !v.is_empty()) {
                    out += &format!("{k:<11}{v}\n");
                }
            };
            field("id", Some(n.id.clone()));
            field("type", Some(format!("{} ({})", n.ntype, n.origin)));
            field("kind", n.kind.clone());
            field("name", n.qname.clone().or(n.title.clone()));
            field("location", Some(n.location()));
            field("signature", n.signature.clone());
            field("summary", n.summary.clone());
            field("doc", n.doc.clone());
            field("body hash", n.body_hash.clone());
            field("status", n.status.clone());
            let extra: serde_json::Value = n
                .extra
                .as_deref()
                .and_then(|e| serde_json::from_str(e).ok())
                .unwrap_or_default();
            if n.origin == "earned" && extra.is_object() {
                field("file", extra["file"].as_str().map(str::to_string));
                field("source", extra["source"].as_str().map(str::to_string));
                for ev in extra["evidence"].as_array().into_iter().flatten() {
                    let kind = ev["kind"].as_str().unwrap_or("");
                    let value = ev["value"].as_str().unwrap_or("");
                    field(
                        "evidence",
                        Some(if kind.is_empty() {
                            value.to_string()
                        } else {
                            format!("{kind}: {value}")
                        }),
                    );
                }
            } else {
                field("extra", n.extra.clone());
            }
            if !edges.is_empty() {
                out += "relations\n";
                for e in edges {
                    let status = e
                        .status
                        .as_deref()
                        .map(|s| format!("  ({s})"))
                        .unwrap_or_default();
                    if e.src == n.id {
                        out += &format!("  -> {:<12} {}{status}\n", e.rel, e.dst);
                    } else {
                        out += &format!("  <- {:<12} {}{status}\n", e.rel, e.src);
                    }
                }
            }
            out
        })
    })
}
