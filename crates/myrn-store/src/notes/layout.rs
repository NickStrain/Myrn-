//! Folder layout (decisions/, bugs/, workflows/, facts/, summaries/) and Windows-safe file names.
//!
//! A note's file name is its id. Ids are lowercase ASCII (`bug-7f3a91`), so two
//! notes never differ only by case and every name is valid on Windows.

/// Note types. All but `summary` are earned node types; a summary note sets the
/// summary of the code node it is `about` instead of becoming a node itself.
pub const NOTE_TYPES: [&str; 7] = [
    "decision",
    "bug",
    "root_cause",
    "workflow",
    "fact",
    "feature",
    "summary",
];

/// Folder a note of this type is written to.
pub fn folder_for(ntype: &str) -> Option<&'static str> {
    Some(match ntype {
        "decision" => "decisions",
        "bug" | "root_cause" => "bugs",
        "workflow" => "workflows",
        "fact" | "feature" => "facts",
        "summary" => "summaries",
        _ => return None,
    })
}

/// Type assumed for a note in this folder whose header does not say.
pub fn type_for_folder(folder: &str) -> Option<&'static str> {
    Some(match folder {
        "decisions" => "decision",
        "bugs" => "bug",
        "workflows" => "workflow",
        "facts" => "fact",
        "summaries" => "summary",
        _ => return None,
    })
}

/// `root_cause` -> `rootcause`; other types are their own prefix.
pub fn id_prefix(ntype: &str) -> String {
    ntype.replace('_', "")
}

/// `bug-7f3a91`: the type prefix and the first `len` hex digits of the content hash.
/// Identical knowledge gets the same id everywhere.
pub fn make_id(ntype: &str, content: &str, len: usize) -> String {
    let normalized: String = content.split_whitespace().collect::<Vec<_>>().join(" ");
    let h = blake3::hash(format!("{ntype}\n{normalized}").as_bytes()).to_hex();
    format!("{}-{}", id_prefix(ntype), &h[..len.clamp(4, 64)])
}

const RESERVED: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// A file stem that is valid on every OS: lowercase, no reserved characters,
/// no Windows device names, no trailing dot or space.
pub fn safe_stem(id: &str) -> String {
    let mut s: String = id
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_control() || matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') {
                '_'
            } else {
                c
            }
        })
        .collect();
    while s.ends_with(['.', ' ']) {
        s.pop();
    }
    if s.is_empty() {
        s.push('_');
    }
    let base = s.split('.').next().unwrap_or("");
    if RESERVED.contains(&base) {
        s.insert(base.len(), '_');
    }
    s
}

pub fn file_name(id: &str) -> String {
    format!("{}.md", safe_stem(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_stable_and_prefixed() {
        let a = make_id("bug", "Refresh  returns\n401", 6);
        let b = make_id("bug", "Refresh returns 401", 6);
        assert_eq!(a, b);
        assert!(a.starts_with("bug-") && a.len() == 10);
        assert!(make_id("root_cause", "x", 6).starts_with("rootcause-"));
        assert_ne!(make_id("fact", "x", 6), make_id("decision", "x", 6));
    }

    #[test]
    fn windows_safe_names() {
        assert_eq!(safe_stem("Bug-7F3A91"), "bug-7f3a91");
        assert_eq!(safe_stem("con"), "con_");
        assert_eq!(safe_stem("aux.notes"), "aux_.notes");
        assert_eq!(safe_stem("a:b/c?"), "a_b_c_");
        assert_eq!(safe_stem("trailing. "), "trailing");
        assert_eq!(safe_stem("console"), "console");
    }

    #[test]
    fn every_type_has_a_folder() {
        for t in NOTE_TYPES {
            let f = folder_for(t).unwrap();
            assert!(crate::paths::KNOWLEDGE_KINDS.contains(&f));
        }
    }
}
