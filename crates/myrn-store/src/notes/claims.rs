//! Whether two statements make opposite claims about the same thing.
//!
//! Without a model the check is deliberately narrow: nearly the same words,
//! where one negates what the other states ("call X before Y" / "never call X
//! before Y"). Used when a record is stored and when shared records are linked.

use std::collections::HashSet;

/// Share of the non-negation words two statements must have in common.
const SAME_CLAIM: f64 = 0.6;

const NEGATIONS: [&str; 16] = [
    "not", "no", "never", "dont", "doesnt", "isnt", "arent", "cannot", "cant", "shouldnt",
    "mustnt", "wont", "without", "neither", "nor", "avoid",
];

fn words(text: &str) -> (HashSet<String>, usize) {
    let mut set = HashSet::new();
    let mut negations = 0;
    for w in text
        .to_lowercase()
        .replace(['\'', '’'], "")
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|w| !w.is_empty())
    {
        if NEGATIONS.contains(&w) {
            negations += 1;
        } else if w.len() > 2 {
            set.insert(w.to_string());
        }
    }
    (set, negations)
}

/// Do these two statements say opposite things about the same subject?
pub fn opposite(a: &str, b: &str) -> bool {
    let (wa, na) = words(a);
    let (wb, nb) = words(b);
    if wa.is_empty() || wb.is_empty() || (na % 2) == (nb % 2) {
        return false;
    }
    let common = wa.intersection(&wb).count() as f64;
    common / wa.union(&wb).count() as f64 >= SAME_CLAIM
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negated_claims_conflict() {
        assert!(opposite(
            "Call TokenStore::lookup before Session::new.",
            "Never call TokenStore::lookup before Session::new."
        ));
        assert!(opposite(
            "The refresh window is configurable.",
            "The refresh window isn't configurable."
        ));
        assert!(!opposite(
            "The refresh window is configurable.",
            "The refresh window is configurable in project.toml."
        ));
        assert!(!opposite(
            "Never call lookup before new.",
            "Retries do not apply to uploads larger than the chunk size."
        ));
    }
}
