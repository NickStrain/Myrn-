//! Supported languages and their naming rules.
//!
//! First-class in this release: Rust, Python, JavaScript, TypeScript (and TSX).

use tree_sitter::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Tsx,
}

impl Lang {
    pub const ALL: [Lang; 5] = [
        Lang::Rust,
        Lang::Python,
        Lang::JavaScript,
        Lang::TypeScript,
        Lang::Tsx,
    ];

    pub fn from_path(path: &str) -> Option<Lang> {
        let name = path.rsplit('/').next().unwrap_or(path);
        if name.ends_with(".min.js") || name.ends_with(".d.ts") {
            return None;
        }
        let ext = name.rsplit_once('.')?.1;
        Some(match ext {
            "rs" => Lang::Rust,
            "py" | "pyi" => Lang::Python,
            "js" | "mjs" | "cjs" | "jsx" => Lang::JavaScript,
            "ts" | "mts" | "cts" => Lang::TypeScript,
            "tsx" => Lang::Tsx,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Lang::Rust => "rust",
            Lang::Python => "python",
            Lang::JavaScript => "javascript",
            Lang::TypeScript => "typescript",
            Lang::Tsx => "tsx",
        }
    }

    pub fn parse_name(s: &str) -> Option<Lang> {
        Self::ALL.into_iter().find(|l| l.name() == s)
    }

    pub fn grammar(self) -> Language {
        match self {
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
            Lang::Python => tree_sitter_python::LANGUAGE.into(),
            Lang::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Lang::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
        }
    }

    pub fn family(self) -> Family {
        match self {
            Lang::Rust => Family::Rust,
            Lang::Python => Family::Python,
            _ => Family::Js,
        }
    }

    /// Separator between qualified-name segments.
    pub fn sep(self) -> &'static str {
        match self.family() {
            Family::Rust => "::",
            _ => ".",
        }
    }
}

/// Languages that share resolution rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    Rust,
    Python,
    Js,
}

/// Directories that start a Rust crate target.
const RUST_ROOTS: [&str; 4] = ["src", "tests", "examples", "benches"];

/// For a Rust file: (crate directory, module path inside the crate).
/// `crates/a/src/auth/session.rs` -> ("crates/a", ["auth", "session"]).
pub fn rust_module(path: &str) -> (String, Vec<String>) {
    let segs: Vec<&str> = path.split('/').collect();
    let root_at = segs[..segs.len().saturating_sub(1)]
        .iter()
        .rposition(|s| RUST_ROOTS.contains(s));
    let Some(i) = root_at else {
        // A loose file such as build.rs: its own root.
        return (segs[..segs.len() - 1].join("/"), Vec::new());
    };
    let crate_dir = segs[..i].join("/");
    let mut rest: Vec<String> = segs[i + 1..].iter().map(|s| s.to_string()).collect();
    let last = rest.pop().unwrap_or_default();
    let stem = last.trim_end_matches(".rs").to_string();
    if segs[i] == "src" && rest.first().map(String::as_str) == Some("bin") {
        // src/bin/foo.rs and src/bin/foo/main.rs are binary roots.
        return (crate_dir, rest.into_iter().skip(2).collect());
    }
    let is_root_file = rest.is_empty() && (segs[i] != "src" || stem == "lib" || stem == "main");
    if !is_root_file && stem != "mod" {
        rest.push(stem);
    }
    (crate_dir, rest)
}

/// Dotted module path of a Python file: `src/pkg/mod.py` -> ["pkg", "mod"].
pub fn python_module(path: &str) -> Vec<String> {
    let trimmed = path.trim_end_matches(".pyi").trim_end_matches(".py");
    let mut segs: Vec<String> = trimmed.split('/').map(str::to_string).collect();
    if segs.first().map(String::as_str) == Some("src") && segs.len() > 1 {
        segs.remove(0);
    }
    if segs.last().map(String::as_str) == Some("__init__") {
        segs.pop();
    }
    segs
}

/// Prefix for qualified names in this file.
pub fn module_prefix(lang: Lang, path: &str) -> Vec<String> {
    match lang.family() {
        Family::Rust => rust_module(path).1,
        Family::Python => python_module(path),
        Family::Js => Vec::new(),
    }
}

pub fn is_python_test_file(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    name.starts_with("test_")
        || name.ends_with("_test.py")
        || path.split('/').any(|s| s == "tests" || s == "test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_modules() {
        let m = |p| rust_module(p);
        assert_eq!(m("src/lib.rs"), ("".into(), vec![]));
        assert_eq!(m("src/main.rs"), ("".into(), vec![]));
        assert_eq!(
            m("src/auth/session.rs"),
            ("".into(), vec!["auth".into(), "session".into()])
        );
        assert_eq!(m("src/auth/mod.rs"), ("".into(), vec!["auth".into()]));
        assert_eq!(
            m("crates/x/src/a.rs"),
            ("crates/x".into(), vec!["a".into()])
        );
        assert_eq!(m("tests/it.rs"), ("".into(), vec![]));
        assert_eq!(m("tests/common/mod.rs"), ("".into(), vec!["common".into()]));
        assert_eq!(m("src/bin/tool.rs"), ("".into(), vec![]));
        assert_eq!(m("build.rs"), ("".into(), vec![]));
    }

    #[test]
    fn python_modules() {
        assert_eq!(python_module("pkg/a.py"), ["pkg", "a"]);
        assert_eq!(python_module("src/pkg/__init__.py"), ["pkg"]);
    }

    #[test]
    fn detects_languages() {
        assert_eq!(Lang::from_path("a/b.tsx"), Some(Lang::Tsx));
        assert_eq!(Lang::from_path("x.min.js"), None);
        assert_eq!(Lang::from_path("README.md"), None);
    }
}
