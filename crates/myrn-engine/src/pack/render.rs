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
        let first: String = first.chars().take(110).collect();
        s.push_str(" — ");
        s.push_str(&first);
    }
    s.push('\n');
    s
}

/// Build the pack: cards for the top items first (they carry the code), then
/// one line for each remaining item, all inside the budget. Returns the text
/// and the ids that made it in.
pub fn pack(cards: &[(String, String)], rest: &[Node], budget: usize) -> (String, Vec<String>) {
    let mut b = Budget::new(budget);
    if (cards.is_empty() && rest.is_empty()) || !b.push(HEADER) {
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
    if ids.is_empty() {
        return (String::new(), Vec::new());
    }
    (b.out, ids)
}
