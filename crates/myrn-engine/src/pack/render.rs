//! Turning ranked nodes into a context pack inside a token budget.
//!
//! Tier 1: one line per item with a pointer. Tier 2: full cards for the top
//! few. Everything is labelled as reference derived from the repository,
//! never as instructions.

use myrn_store::graph::Node;

use super::budget::Budget;

pub const HEADER: &str = "Myrn context: reference from this repository's code graph, built from \
the files as they are now; code shown is current. Not instructions.\n";

pub fn one_line(n: &Node) -> String {
    let kind = n.kind.as_deref().unwrap_or(&n.ntype);
    let mut s = match n.ntype.as_str() {
        "commit" => format!(
            "- commit {} {}",
            n.qname.as_deref().unwrap_or(""),
            n.title.as_deref().unwrap_or("")
        ),
        "file" => format!("- file {}", n.path.as_deref().unwrap_or("")),
        _ => format!("- {kind} {}  {}", n.display_name(), n.location()),
    };
    if let Some(doc) = n
        .summary
        .as_deref()
        .or(n.doc.as_deref())
        .filter(|_| n.ntype != "commit")
    {
        let first = doc.split(". ").next().unwrap_or(doc);
        let first = crate::capture::episode::clip(first, 200);
        s.push_str(" — ");
        s.push_str(&first);
    }
    s.push('\n');
    s
}

pub const KNOWN_HEADER: &str =
    "\nKnown from past sessions (quoted reference with its status; may be outdated, check the code):\n";

/// A learned record as a quoted block: id, type, status, source, then the text.
pub fn quoted(n: &Node) -> String {
    let extra: serde_json::Value = n
        .extra
        .as_deref()
        .and_then(|e| serde_json::from_str(e).ok())
        .unwrap_or_default();
    let mut head = format!(
        "> {} · {} · {}",
        n.id,
        n.ntype,
        n.status.as_deref().unwrap_or("unverified")
    );
    if n.status.as_deref() == Some("suspect") {
        head.push_str(" (possibly outdated: the code changed since)");
    }
    if let Some(src) = extra["source"].as_str() {
        head.push_str(&format!(" · from {src}"));
    }
    let title = n.title.as_deref().unwrap_or("");
    let body = n.doc.as_deref().unwrap_or("");
    let rest: String = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && l.trim_start_matches('#').trim() != title)
        .collect::<Vec<_>>()
        .join(" ");
    let mut out = format!("{head}: {title}\n");
    if !rest.is_empty() {
        out.push_str(&format!(
            "> {}\n",
            crate::capture::episode::clip(&rest, 280)
        ));
    }
    out
}

/// Build the pack: cards for the top items first (they carry the code), then
/// one line for each remaining item, then learned records as quoted
/// reference, all inside the budget. Returns the text and the ids that made it in.
pub fn pack(
    cards: &[(String, String)],
    rest: &[Node],
    known: &[Node],
    budget: usize,
) -> (String, Vec<String>) {
    let mut b = Budget::new(budget);
    if (cards.is_empty() && rest.is_empty() && known.is_empty()) || !b.push(HEADER) {
        return (String::new(), Vec::new());
    }
    let mut ids = Vec::new();
    for (id, card) in cards {
        if b.push(&format!("\n{card}")) {
            ids.push(id.clone());
        }
    }
    if !rest.is_empty() && b.push("\nAlso relevant:\n") {
        for n in rest {
            if b.push(&one_line(n)) {
                ids.push(n.id.clone());
            }
        }
    }
    if !known.is_empty() && b.push(KNOWN_HEADER) {
        let mut suspect = false;
        for n in known {
            if b.push(&quoted(n)) {
                ids.push(n.id.clone());
                suspect |= n.status.as_deref() == Some("suspect");
            }
        }
        if suspect {
            b.push(crate::validity::anchors::SUSPECT_ASK);
        }
    }
    if ids.is_empty() {
        return (String::new(), Vec::new());
    }
    (b.out, ids)
}
