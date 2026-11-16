//! history(path): changes concerning a file, directory or symbol.

use anyhow::Result;
use myrn_engine::api::Engine;
use serde_json::{json, Value};

use super::{schema, str_arg};

pub fn spec() -> Value {
    json!({
        "name": "history",
        "description": "Commits that changed a file, directory or symbol, newest first.",
        "inputSchema": schema(json!({
            "path": { "type": "string", "description": "File path, directory, or symbol name." },
            "limit": { "type": "integer", "description": "Maximum commits (default 15)." }
        }), &["path"]),
    })
}

pub fn call(engine: &mut Engine, args: &Value) -> Result<String> {
    let q = str_arg(args, "path")?;
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(15) as usize;
    Ok(engine.history(q, limit)?.render(|entries| render(entries)))
}

pub fn render(entries: &[myrn_engine::api::HistoryEntry]) -> String {
    if entries.is_empty() {
        return "No indexed commits touch this.\n".into();
    }
    let mut s = String::new();
    for e in entries {
        let date = days_to_date(e.time.div_euclid(86_400));
        let change = e
            .change
            .as_deref()
            .map(|c| format!(" [{c}]"))
            .unwrap_or_default();
        s.push_str(&format!(
            "{} {date} {}{change}  ({})\n",
            &e.sha[..7.min(e.sha.len())],
            e.summary,
            e.author
        ));
    }
    s
}

/// Civil date from days since 1970-01-01.
fn days_to_date(z: i64) -> String {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn dates() {
        assert_eq!(super::days_to_date(0), "1970-01-01");
        assert_eq!(super::days_to_date(20_773), "2026-11-16");
    }
}
