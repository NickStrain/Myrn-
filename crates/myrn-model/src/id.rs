//! Identifiers for derived nodes.
//!
//! Derived ids are readable and stable for the same code: a symbol's identity
//! is its file plus its qualified name. Earned records (later phases) use a
//! content hash instead.

/// `file:<path>`
pub fn file_id(path: &str) -> String {
    format!("file:{path}")
}

/// `module:<dir>`; the repository root is `module:.`
pub fn module_id(dir: &str) -> String {
    if dir.is_empty() {
        "module:.".to_string()
    } else {
        format!("module:{dir}")
    }
}

/// `sym:<path>#<qualified name>`
pub fn symbol_id(path: &str, qname: &str) -> String {
    format!("sym:{path}#{qname}")
}

/// `test:<path>#<qualified name>`
pub fn test_id(path: &str, qname: &str) -> String {
    format!("test:{path}#{qname}")
}

/// `commit:<sha>`
pub fn commit_id(sha: &str) -> String {
    format!("commit:{sha}")
}

/// `episode:<session>`
pub fn episode_id(session: &str) -> String {
    format!("episode:{session}")
}

/// Directory part of a repo-relative, forward-slash path ("" for the root).
pub fn parent_dir(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[..i],
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert_eq!(symbol_id("src/a.rs", "a::f"), "sym:src/a.rs#a::f");
        assert_eq!(module_id(""), "module:.");
        assert_eq!(parent_dir("src/a/b.rs"), "src/a");
        assert_eq!(parent_dir("b.rs"), "");
    }
}
