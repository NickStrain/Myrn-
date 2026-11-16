//! Tree-sitter front end: one source file in, definitions, call references
//! and imports out. No model involved; nothing is resolved across files here.

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tree_sitter::{Node, Parser};

use super::body_hash::body_hash;
use super::lang::{self, Family, Lang};

/// One definition: a function, method, type, or test case.
#[derive(Debug, Clone)]
pub struct Def {
    pub qname: String,
    pub name: String,
    pub kind: &'static str,
    pub start_line: i64,
    pub end_line: i64,
    pub signature: String,
    pub doc: Option<String>,
    pub body_hash: String,
    pub is_test: bool,
    /// Rust only: module path including inline `mod` blocks (for `super::`/`self::`).
    pub module: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefKind {
    /// `f()`
    Call,
    /// `Type::f()`, `module::f()`
    Path,
    /// `recv.f()`
    Method,
}

impl RefKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RefKind::Call => "call",
            RefKind::Path => "path",
            RefKind::Method => "method",
        }
    }
    pub fn parse(s: &str) -> Option<RefKind> {
        match s {
            "call" => Some(RefKind::Call),
            "path" => Some(RefKind::Path),
            "method" => Some(RefKind::Method),
            _ => None,
        }
    }
}

/// A call site inside a definition.
#[derive(Debug, Clone)]
pub struct Ref {
    /// Index into `Parsed::defs` of the innermost enclosing definition.
    pub from: usize,
    pub kind: RefKind,
    pub name: String,
    pub qualifier: Option<String>,
    pub line: i64,
}

/// An import, with the local name it binds.
///
/// `spec` formats: Rust `crate::a::B`, Python `pkg.mod.name` (leading dots for
/// relative imports), JS/TS `./path#member` (`#default`, `#*`, or `#` for a
/// bare side-effect import).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub spec: String,
    pub alias: Option<String>,
    pub line: i64,
    /// Rust only: module path of the `use` site, including inline `mod` blocks.
    pub module: Option<String>,
}

#[derive(Debug, Default)]
pub struct Parsed {
    pub defs: Vec<Def>,
    pub refs: Vec<Ref>,
    pub imports: Vec<Import>,
    /// The tree contains syntax errors (indexed as far as possible).
    pub has_errors: bool,
}

/// Holds one Tree-sitter parser per language.
#[derive(Default)]
pub struct FileParser {
    parsers: HashMap<Lang, Parser>,
}

impl FileParser {
    pub fn new() -> FileParser {
        FileParser::default()
    }

    pub fn parse(&mut self, lang: Lang, path: &str, src: &[u8]) -> Result<Parsed> {
        let parser = match self.parsers.entry(lang) {
            std::collections::hash_map::Entry::Occupied(e) => e.into_mut(),
            std::collections::hash_map::Entry::Vacant(e) => {
                let mut p = Parser::new();
                p.set_language(&lang.grammar())?;
                e.insert(p)
            }
        };
        let tree = parser
            .parse(src, None)
            .ok_or_else(|| anyhow!("tree-sitter could not parse {path}"))?;
        let root = tree.root_node();
        let mut w = Walker {
            lang,
            path,
            src,
            prefix: lang::module_prefix(lang, path),
            scope: Vec::new(),
            enclosing: None,
            seen: HashMap::new(),
            locals: HashMap::new(),
            out: Parsed {
                has_errors: root.has_error(),
                ..Default::default()
            },
        };
        w.visit(root, 0);
        Ok(w.out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeKind {
    /// impl, class, trait, interface: functions inside are methods.
    Type,
    /// inline module, namespace, describe block.
    Module,
    Func,
}

enum Action<'t> {
    Def {
        name: String,
        kind: &'static str,
        scope: ScopeKind,
        is_test: bool,
        /// Node whose start marks the end of the signature.
        body: Option<Node<'t>>,
        /// Node to read the doc comment around (defaults to the def node).
        doc_anchor: Option<Node<'t>>,
    },
    Scope(String, ScopeKind),
    Ref(RefKind, String, Option<String>),
    /// Do not descend.
    Skip,
    /// Nothing here; descend.
    Descend,
}

const MAX_DEPTH: usize = 512;

struct Walker<'a> {
    lang: Lang,
    path: &'a str,
    src: &'a [u8],
    prefix: Vec<String>,
    scope: Vec<(String, ScopeKind)>,
    enclosing: Option<usize>,
    seen: HashMap<String, usize>,
    /// Local variable -> type name, for the current definition.
    locals: HashMap<String, String>,
    out: Parsed,
}

impl<'a> Walker<'a> {
    fn text(&self, n: Node<'_>) -> &'a str {
        std::str::from_utf8(&self.src[n.byte_range()]).unwrap_or("")
    }

    fn field_text(&self, n: Node<'_>, field: &str) -> Option<&'a str> {
        n.child_by_field_name(field).map(|c| self.text(c))
    }

    fn line(n: Node<'_>) -> i64 {
        n.start_position().row as i64 + 1
    }

    /// Rust module path at the current position: file module plus inline `mod` blocks.
    fn rust_mod_path(&self) -> Option<String> {
        if self.lang.family() != Family::Rust {
            return None;
        }
        let mut parts: Vec<&str> = self.prefix.iter().map(String::as_str).collect();
        parts.extend(
            self.scope
                .iter()
                .filter(|(_, k)| *k == ScopeKind::Module)
                .map(|(s, _)| s.as_str()),
        );
        Some(parts.join("::"))
    }

    fn push_import(&mut self, mut imp: Import) {
        imp.module = self.rust_mod_path();
        self.out.imports.push(imp);
    }

    fn push_ref(
        &mut self,
        mut kind: RefKind,
        name: String,
        mut qualifier: Option<String>,
        line: i64,
    ) {
        // `x.f()` where the type of `x` is known locally: treat as `Type::f`.
        if kind == RefKind::Method {
            if let Some(t) = qualifier.as_ref().and_then(|q| self.locals.get(q)) {
                qualifier = Some(t.clone());
                kind = RefKind::Path;
            }
        }
        if let Some(from) = self.enclosing {
            if !name.is_empty() {
                self.out.refs.push(Ref {
                    from,
                    kind,
                    name,
                    qualifier,
                    line,
                });
            }
        }
    }

    /// Calls inside Rust macro arguments (`assert!(f(x))`), which Tree-sitter
    /// leaves as raw tokens: `name(`, `a::b(`, `recv.name(`.
    fn rust_macro_calls(&mut self, tree: Node<'_>) {
        let mut cursor = tree.walk();
        let toks: Vec<Node<'_>> = tree.children(&mut cursor).collect();
        for (i, t) in toks.iter().enumerate() {
            if t.kind() == "token_tree" {
                self.rust_macro_calls(*t);
                continue;
            }
            if t.kind() != "identifier" {
                continue;
            }
            let opens_call = toks
                .get(i + 1)
                .is_some_and(|n| n.kind() == "token_tree" && self.text(*n).starts_with('('));
            if !opens_call {
                continue;
            }
            let name = self.text(*t).to_string();
            if matches!(
                name.as_str(),
                "if" | "while" | "match" | "return" | "in" | "Some" | "Ok" | "Err"
            ) {
                continue;
            }
            let line = Self::line(*t);
            let prev = if i > 0 {
                Some(self.text(toks[i - 1]))
            } else {
                None
            };
            match prev {
                Some(".") => {
                    let recv = if i > 1 && matches!(toks[i - 2].kind(), "identifier" | "self") {
                        Some(self.text(toks[i - 2]).to_string())
                    } else {
                        None
                    };
                    self.push_ref(RefKind::Method, name, recv, line);
                }
                Some("::") => {
                    let mut j = i;
                    let mut path: Vec<&str> = Vec::new();
                    while j >= 2 && self.text(toks[j - 1]) == "::" {
                        let seg = toks[j - 2];
                        if !matches!(seg.kind(), "identifier" | "self" | "super" | "crate") {
                            break;
                        }
                        path.push(self.text(seg));
                        j -= 2;
                    }
                    path.reverse();
                    if !path.is_empty() {
                        self.push_ref(RefKind::Path, name, Some(path.join("::")), line);
                    }
                }
                _ => self.push_ref(RefKind::Call, name, None, line),
            }
        }
    }

    /// Record parameter types of a definition.
    fn note_params(&mut self, def: Node<'_>) {
        let params = def.child_by_field_name("parameters").or_else(|| {
            def.child_by_field_name("value")
                .and_then(|v| v.child_by_field_name("parameters"))
        });
        let Some(params) = params else { return };
        let mut cursor = params.walk();
        let list: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        for p in list {
            let (name, ty) = match p.kind() {
                // Rust
                "parameter" => (
                    p.child_by_field_name("pattern"),
                    p.child_by_field_name("type"),
                ),
                // Python
                "typed_parameter" => (p.named_child(0), p.child_by_field_name("type")),
                "typed_default_parameter" => {
                    (p.child_by_field_name("name"), p.child_by_field_name("type"))
                }
                // TypeScript
                "required_parameter" | "optional_parameter" => (
                    p.child_by_field_name("pattern"),
                    p.child_by_field_name("type"),
                ),
                _ => continue,
            };
            if let (Some(n), Some(t)) = (name, ty) {
                self.bind_local(n, t);
            }
        }
    }

    /// Record `let x = Type::new()`, `let x: Type`, `x = Type()`, `const x = new Type()`.
    fn note_local(&mut self, n: Node<'_>) {
        let (name, ty, value) = match n.kind() {
            "let_declaration" => (
                n.child_by_field_name("pattern"),
                n.child_by_field_name("type"),
                n.child_by_field_name("value"),
            ),
            "assignment" => (
                n.child_by_field_name("left"),
                n.child_by_field_name("type"),
                n.child_by_field_name("right"),
            ),
            "variable_declarator" => (
                n.child_by_field_name("name"),
                n.child_by_field_name("type"),
                n.child_by_field_name("value"),
            ),
            _ => return,
        };
        let Some(name) = name else { return };
        if let Some(t) = ty {
            self.bind_local(name, t);
            return;
        }
        let Some(mut v) = value else { return };
        while matches!(v.kind(), "try_expression" | "await_expression" | "await") {
            match v.named_child(0) {
                Some(c) => v = c,
                None => return,
            }
        }
        let ty = match v.kind() {
            // Rust Type::new(..), Python Type(..)
            "call_expression" | "call" => {
                let Some(mut f) = v.child_by_field_name("function") else {
                    return;
                };
                if f.kind() == "generic_function" {
                    match f.child_by_field_name("function") {
                        Some(i) => f = i,
                        None => return,
                    }
                }
                match f.kind() {
                    "scoped_identifier" => {
                        let ctor = f
                            .child_by_field_name("name")
                            .map(|n| self.text(n))
                            .is_some_and(is_constructor_name);
                        if ctor {
                            f.child_by_field_name("path")
                        } else {
                            None
                        }
                    }
                    "identifier" if self.lang.family() == Family::Python => Some(f),
                    "attribute" => f.child_by_field_name("attribute"),
                    _ => None,
                }
            }
            "struct_expression" => v.child_by_field_name("name"),
            "new_expression" => v.child_by_field_name("constructor"),
            _ => None,
        };
        if let Some(t) = ty {
            self.bind_local(name, t);
        }
    }

    fn bind_local(&mut self, name: Node<'_>, ty: Node<'_>) {
        let mut name = name;
        if name.kind() == "mut_pattern" {
            match name.named_child(0) {
                Some(n) => name = n,
                None => return,
            }
        }
        if name.kind() != "identifier" {
            return;
        }
        if let Some(t) = type_name(self.text(ty)) {
            self.locals.insert(self.text(name).to_string(), t);
        }
    }

    fn in_type_scope(&self) -> bool {
        matches!(self.scope.last(), Some((_, ScopeKind::Type)))
    }

    fn visit(&mut self, node: Node<'_>, depth: usize) {
        if depth > MAX_DEPTH {
            return;
        }
        if self.enclosing.is_some() {
            self.note_local(node);
        }
        match self.classify(node) {
            Action::Def {
                name,
                kind,
                scope,
                is_test,
                body,
                doc_anchor,
            } => {
                let idx = self.push_def(node, name.clone(), kind, is_test, body, doc_anchor);
                self.scope.push((name, scope));
                let prev = self.enclosing.replace(idx);
                let saved = self.locals.clone();
                self.note_params(node);
                self.visit_children(node, depth);
                self.locals = saved;
                self.enclosing = prev;
                self.scope.pop();
            }
            Action::Scope(name, kind) => {
                self.scope.push((name, kind));
                self.visit_children(node, depth);
                self.scope.pop();
            }
            Action::Ref(kind, name, qualifier) => {
                self.push_ref(kind, name, qualifier, Self::line(node));
                self.visit_children(node, depth);
            }
            Action::Skip => {}
            Action::Descend => self.visit_children(node, depth),
        }
    }

    fn visit_children(&mut self, node: Node<'_>, depth: usize) {
        let mut cursor = node.walk();
        let children: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        for c in children {
            self.visit(c, depth + 1);
        }
    }

    fn push_def(
        &mut self,
        node: Node<'_>,
        name: String,
        kind: &'static str,
        is_test: bool,
        body: Option<Node<'_>>,
        doc_anchor: Option<Node<'_>>,
    ) -> usize {
        let sep = self.lang.sep();
        let mut parts: Vec<&str> = self.prefix.iter().map(String::as_str).collect();
        parts.extend(self.scope.iter().map(|(s, _)| s.as_str()));
        parts.push(&name);
        let mut qname = parts.join(sep);
        let n = self.seen.entry(qname.clone()).or_insert(0);
        *n += 1;
        if *n > 1 {
            qname = format!("{qname}#{n}");
        }
        let signature = self.signature(node, body);
        let doc = self.doc(node, doc_anchor);
        self.out.defs.push(Def {
            qname,
            name,
            kind,
            start_line: Self::line(node),
            end_line: node.end_position().row as i64 + 1,
            signature,
            doc,
            body_hash: body_hash(node, self.src),
            is_test,
            module: self.rust_mod_path(),
        });
        self.out.defs.len() - 1
    }

    fn signature(&self, node: Node<'_>, body: Option<Node<'_>>) -> String {
        let end = match body {
            Some(b) if b.start_byte() > node.start_byte() => b.start_byte(),
            _ => {
                let t = self.text(node);
                node.start_byte() + t.find('\n').unwrap_or(t.len())
            }
        };
        let raw = String::from_utf8_lossy(&self.src[node.start_byte()..end]);
        let mut s = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        loop {
            if let Some(t) = s.strip_suffix("=>") {
                s = t.trim_end().to_string();
            } else if s.ends_with(['{', ':', '=']) {
                s.pop();
                s = s.trim_end().to_string();
            } else {
                break;
            }
        }
        truncate(&s, 200)
    }

    fn doc(&self, node: Node<'_>, anchor: Option<Node<'_>>) -> Option<String> {
        let raw = match self.lang.family() {
            Family::Python => {
                let body = node.child_by_field_name("body")?;
                let first = body.named_child(0)?;
                if first.kind() != "expression_statement" {
                    return None;
                }
                let s = first.named_child(0)?;
                if s.kind() != "string" {
                    return None;
                }
                unquote(self.text(s)).to_string()
            }
            Family::Rust => {
                let mut lines = Vec::new();
                let mut cur = anchor.unwrap_or(node).prev_sibling();
                while let Some(c) = cur {
                    let t = self.text(c);
                    match c.kind() {
                        "attribute_item" => {}
                        "line_comment" if t.starts_with("///") && !t.starts_with("////") => {
                            lines.push(t.trim_start_matches('/').trim().to_string())
                        }
                        "block_comment" if t.starts_with("/**") => {
                            lines.push(clean_block_comment(t))
                        }
                        _ => break,
                    }
                    cur = c.prev_sibling();
                }
                lines.reverse();
                lines.join("\n")
            }
            Family::Js => {
                let mut anchor = anchor.unwrap_or(node);
                if let Some(p) = anchor.parent() {
                    if p.kind() == "export_statement" {
                        anchor = p;
                    }
                }
                let c = anchor.prev_named_sibling()?;
                let t = self.text(c);
                if c.kind() != "comment" || !t.starts_with("/**") {
                    return None;
                }
                clean_block_comment(t)
            }
        };
        let para = raw.trim().split("\n\n").next().unwrap_or("").trim();
        if para.is_empty() {
            None
        } else {
            Some(truncate(
                &para.split_whitespace().collect::<Vec<_>>().join(" "),
                300,
            ))
        }
    }

    fn classify<'t>(&mut self, n: Node<'t>) -> Action<'t> {
        match self.lang.family() {
            Family::Rust => self.classify_rust(n),
            Family::Python => self.classify_python(n),
            Family::Js => self.classify_js(n),
        }
    }

    // ---------------------------------------------------------------- Rust

    fn classify_rust<'t>(&mut self, n: Node<'t>) -> Action<'t> {
        let def = |name: Option<&str>, kind: &'static str, scope| match name {
            Some(name) => Action::Def {
                name: name.to_string(),
                kind,
                scope,
                is_test: false,
                body: n.child_by_field_name("body"),
                doc_anchor: None,
            },
            None => Action::Descend,
        };
        match n.kind() {
            "function_item" | "function_signature_item" => {
                let Some(name) = self.field_text(n, "name") else {
                    return Action::Descend;
                };
                let kind = if self.in_type_scope() {
                    "method"
                } else {
                    "function"
                };
                Action::Def {
                    name: name.to_string(),
                    kind,
                    scope: ScopeKind::Func,
                    is_test: self.rust_has_test_attr(n),
                    body: n.child_by_field_name("body"),
                    doc_anchor: None,
                }
            }
            "struct_item" => def(self.field_text(n, "name"), "struct", ScopeKind::Type),
            "enum_item" => def(self.field_text(n, "name"), "enum", ScopeKind::Type),
            "union_item" => def(self.field_text(n, "name"), "union", ScopeKind::Type),
            "trait_item" => def(self.field_text(n, "name"), "trait", ScopeKind::Type),
            "type_item" => def(self.field_text(n, "name"), "type", ScopeKind::Type),
            "const_item" => def(self.field_text(n, "name"), "const", ScopeKind::Func),
            "static_item" => def(self.field_text(n, "name"), "static", ScopeKind::Func),
            "macro_definition" => def(self.field_text(n, "name"), "macro", ScopeKind::Func),
            "impl_item" => match n.child_by_field_name("type") {
                Some(t) => Action::Scope(self.rust_type_name(t), ScopeKind::Type),
                None => Action::Descend,
            },
            "mod_item" => {
                let Some(name) = self.field_text(n, "name") else {
                    return Action::Skip;
                };
                if n.child_by_field_name("body").is_some() {
                    Action::Scope(name.to_string(), ScopeKind::Module)
                } else {
                    self.push_import(Import {
                        module: None,
                        spec: format!("self::{name}"),
                        alias: Some(name.to_string()),
                        line: Self::line(n),
                    });
                    Action::Skip
                }
            }
            "use_declaration" => {
                if let Some(arg) = n.child_by_field_name("argument") {
                    let mut paths = Vec::new();
                    self.rust_use_paths(arg, "", &mut paths);
                    for (spec, alias) in paths {
                        self.push_import(Import {
                            module: None,
                            spec,
                            alias: Some(alias),
                            line: Self::line(n),
                        });
                    }
                }
                Action::Skip
            }
            "call_expression" => {
                let Some(mut f) = n.child_by_field_name("function") else {
                    return Action::Descend;
                };
                if f.kind() == "generic_function" {
                    match f.child_by_field_name("function") {
                        Some(inner) => f = inner,
                        None => return Action::Descend,
                    }
                }
                match f.kind() {
                    "identifier" => Action::Ref(RefKind::Call, self.text(f).to_string(), None),
                    "scoped_identifier" => Action::Ref(
                        RefKind::Path,
                        self.field_text(f, "name").unwrap_or("").to_string(),
                        self.field_text(f, "path").map(strip_generics),
                    ),
                    "field_expression" => Action::Ref(
                        RefKind::Method,
                        self.field_text(f, "field").unwrap_or("").to_string(),
                        self.field_text(f, "value").map(|s| truncate(s, 80)),
                    ),
                    _ => Action::Descend,
                }
            }
            "macro_invocation" => {
                let mut cursor = n.walk();
                let trees: Vec<Node<'t>> = n
                    .children(&mut cursor)
                    .filter(|c| c.kind() == "token_tree")
                    .collect();
                for t in trees {
                    self.rust_macro_calls(t);
                }
                Action::Skip
            }
            "line_comment" | "block_comment" | "string_literal" | "raw_string_literal" => {
                Action::Skip
            }
            _ => Action::Descend,
        }
    }

    fn rust_type_name(&self, t: Node<'_>) -> String {
        match t.kind() {
            "generic_type" => t
                .child_by_field_name("type")
                .map(|c| self.rust_type_name(c))
                .unwrap_or_else(|| strip_generics(self.text(t))),
            "scoped_type_identifier" => self.field_text(t, "name").unwrap_or("").to_string(),
            "reference_type" | "pointer_type" => t
                .child_by_field_name("type")
                .map(|c| self.rust_type_name(c))
                .unwrap_or_default(),
            _ => strip_generics(self.text(t)),
        }
    }

    fn rust_has_test_attr(&self, n: Node<'_>) -> bool {
        let mut cur = n.prev_sibling();
        while let Some(c) = cur {
            match c.kind() {
                "attribute_item" => {
                    let t = self.text(c).trim_start_matches("#[").trim_end_matches(']');
                    let path = t.split(['(', ' ']).next().unwrap_or("");
                    let last = path.rsplit("::").next().unwrap_or(path);
                    if last == "test" || last.starts_with("test_") || last == "rstest" {
                        return true;
                    }
                }
                "line_comment" | "block_comment" => {}
                _ => break,
            }
            cur = c.prev_sibling();
        }
        false
    }

    fn rust_use_paths(&self, n: Node<'_>, prefix: &str, out: &mut Vec<(String, String)>) {
        let join = |p: &str, x: &str| {
            if p.is_empty() {
                x.to_string()
            } else {
                format!("{p}::{x}")
            }
        };
        let last = |p: &str| p.rsplit("::").next().unwrap_or(p).to_string();
        match n.kind() {
            "identifier" | "scoped_identifier" | "crate" | "self" | "super" => {
                let mut path = join(prefix, self.text(n));
                if let Some(stripped) = path.strip_suffix("::self") {
                    path = stripped.to_string();
                }
                let alias = last(&path);
                out.push((path, alias));
            }
            "use_as_clause" => {
                let path = join(prefix, self.field_text(n, "path").unwrap_or(""));
                let alias = self
                    .field_text(n, "alias")
                    .map(str::to_string)
                    .unwrap_or_else(|| last(&path));
                out.push((path, alias));
            }
            "use_wildcard" => {
                let base = self.text(n).trim_end_matches('*').trim_end_matches("::");
                out.push((
                    join(prefix, &format!("{base}::*")).replace("::::", "::"),
                    "*".to_string(),
                ));
            }
            "scoped_use_list" => {
                let p = match self.field_text(n, "path") {
                    Some(p) => join(prefix, p),
                    None => prefix.to_string(),
                };
                if let Some(list) = n.child_by_field_name("list") {
                    self.rust_use_paths(list, &p, out);
                }
            }
            "use_list" => {
                let mut cursor = n.walk();
                for c in n.named_children(&mut cursor) {
                    self.rust_use_paths(c, prefix, out);
                }
            }
            _ => {}
        }
    }

    // -------------------------------------------------------------- Python

    fn classify_python<'t>(&mut self, n: Node<'t>) -> Action<'t> {
        match n.kind() {
            "function_definition" => {
                let Some(name) = self.field_text(n, "name") else {
                    return Action::Descend;
                };
                let in_class = self.in_type_scope();
                let class_is_test = in_class
                    && self
                        .scope
                        .last()
                        .is_some_and(|(s, _)| s.starts_with("Test"));
                let is_test = name.starts_with("test")
                    && (lang::is_python_test_file(self.path) || class_is_test);
                Action::Def {
                    name: name.to_string(),
                    kind: if in_class { "method" } else { "function" },
                    scope: ScopeKind::Func,
                    is_test,
                    body: n.child_by_field_name("body"),
                    doc_anchor: None,
                }
            }
            "class_definition" => match self.field_text(n, "name") {
                Some(name) => Action::Def {
                    name: name.to_string(),
                    kind: "class",
                    scope: ScopeKind::Type,
                    is_test: false,
                    body: n.child_by_field_name("body"),
                    doc_anchor: None,
                },
                None => Action::Descend,
            },
            "call" => {
                let Some(f) = n.child_by_field_name("function") else {
                    return Action::Descend;
                };
                match f.kind() {
                    "identifier" => Action::Ref(RefKind::Call, self.text(f).to_string(), None),
                    "attribute" => Action::Ref(
                        RefKind::Method,
                        self.field_text(f, "attribute").unwrap_or("").to_string(),
                        self.field_text(f, "object").map(|s| truncate(s, 80)),
                    ),
                    _ => Action::Descend,
                }
            }
            "import_statement" => {
                let mut cursor = n.walk();
                let names: Vec<Node<'_>> = n.children_by_field_name("name", &mut cursor).collect();
                for c in names {
                    let (spec, alias) = match c.kind() {
                        "aliased_import" => (
                            self.field_text(c, "name").unwrap_or("").to_string(),
                            self.field_text(c, "alias").map(str::to_string),
                        ),
                        _ => (self.text(c).to_string(), Some(self.text(c).to_string())),
                    };
                    self.push_import(Import {
                        module: None,
                        spec,
                        alias,
                        line: Self::line(n),
                    });
                }
                Action::Skip
            }
            "import_from_statement" => {
                let module = self.field_text(n, "module_name").unwrap_or("").to_string();
                let join = |m: &str, x: &str| {
                    if m.is_empty() || m.ends_with('.') {
                        format!("{m}{x}")
                    } else {
                        format!("{m}.{x}")
                    }
                };
                let mut cursor = n.walk();
                let names: Vec<Node<'_>> = n.children_by_field_name("name", &mut cursor).collect();
                for c in &names {
                    let (name, alias) = match c.kind() {
                        "aliased_import" => (
                            self.field_text(*c, "name").unwrap_or(""),
                            self.field_text(*c, "alias").unwrap_or(""),
                        ),
                        _ => (self.text(*c), self.text(*c)),
                    };
                    self.push_import(Import {
                        module: None,
                        spec: join(&module, name),
                        alias: Some(alias.to_string()),
                        line: Self::line(n),
                    });
                }
                let mut cursor = n.walk();
                if n.named_children(&mut cursor)
                    .any(|c| c.kind() == "wildcard_import")
                {
                    self.push_import(Import {
                        module: None,
                        spec: join(&module, "*"),
                        alias: Some("*".to_string()),
                        line: Self::line(n),
                    });
                }
                Action::Skip
            }
            "comment" | "string" => Action::Skip,
            _ => Action::Descend,
        }
    }

    // ----------------------------------------------------- JavaScript / TS

    fn classify_js<'t>(&mut self, n: Node<'t>) -> Action<'t> {
        let def = |name: &str, kind: &'static str, scope| Action::Def {
            name: name.to_string(),
            kind,
            scope,
            is_test: false,
            body: n.child_by_field_name("body"),
            doc_anchor: None,
        };
        match n.kind() {
            "function_declaration" | "generator_function_declaration" => {
                match self.field_text(n, "name") {
                    Some(name) => def(name, "function", ScopeKind::Func),
                    None => Action::Descend,
                }
            }
            "class_declaration" | "abstract_class_declaration" | "class" => {
                match self.field_text(n, "name") {
                    Some(name) => def(name, "class", ScopeKind::Type),
                    None => Action::Descend,
                }
            }
            "method_definition" | "abstract_method_signature" | "method_signature" => {
                match self.field_text(n, "name") {
                    Some(name) => def(name, "method", ScopeKind::Func),
                    None => Action::Descend,
                }
            }
            "interface_declaration" => match self.field_text(n, "name") {
                Some(name) => def(name, "interface", ScopeKind::Type),
                None => Action::Descend,
            },
            "type_alias_declaration" => match self.field_text(n, "name") {
                Some(name) => def(name, "type", ScopeKind::Type),
                None => Action::Descend,
            },
            "enum_declaration" => match self.field_text(n, "name") {
                Some(name) => def(name, "enum", ScopeKind::Type),
                None => Action::Descend,
            },
            "internal_module" | "module" => match self.field_text(n, "name") {
                Some(name) => Action::Scope(unquote(name).to_string(), ScopeKind::Module),
                None => Action::Descend,
            },
            "variable_declarator" => {
                let name = n.child_by_field_name("name");
                let value = n.child_by_field_name("value");
                match (name, value) {
                    (Some(nm), Some(v))
                        if nm.kind() == "identifier"
                            && matches!(
                                v.kind(),
                                "arrow_function"
                                    | "function_expression"
                                    | "function"
                                    | "generator_function"
                            ) =>
                    {
                        let decl = n.parent().filter(|p| p.kind().ends_with("declaration"));
                        Action::Def {
                            name: self.text(nm).to_string(),
                            kind: "function",
                            scope: ScopeKind::Func,
                            is_test: false,
                            body: v.child_by_field_name("body"),
                            doc_anchor: decl,
                        }
                    }
                    (Some(nm), Some(v)) if v.kind() == "call_expression" => {
                        self.js_require(nm, v);
                        Action::Descend
                    }
                    _ => Action::Descend,
                }
            }
            "call_expression" => {
                let Some(f) = n.child_by_field_name("function") else {
                    return Action::Descend;
                };
                let (callee, prop) = match f.kind() {
                    "identifier" => (self.text(f), None),
                    "member_expression" => (
                        self.field_text(f, "object").unwrap_or(""),
                        self.field_text(f, "property"),
                    ),
                    _ => return Action::Descend,
                };
                let first_arg = n
                    .child_by_field_name("arguments")
                    .and_then(|a| a.named_child(0))
                    .filter(|a| matches!(a.kind(), "string" | "template_string"))
                    .map(|a| unquote(self.text(a)).to_string());
                let modifier_ok =
                    prop.is_none_or(|p| matches!(p, "only" | "skip" | "concurrent" | "todo"));
                if let (Some(title), true) = (&first_arg, modifier_ok) {
                    match callee {
                        "test" | "it" => {
                            return Action::Def {
                                name: title.clone(),
                                kind: "test",
                                scope: ScopeKind::Func,
                                is_test: true,
                                body: None,
                                doc_anchor: None,
                            }
                        }
                        "describe" | "suite" => {
                            return Action::Scope(title.clone(), ScopeKind::Module)
                        }
                        _ => {}
                    }
                }
                match f.kind() {
                    "identifier" => Action::Ref(RefKind::Call, callee.to_string(), None),
                    _ => Action::Ref(
                        RefKind::Method,
                        prop.unwrap_or("").to_string(),
                        Some(truncate(callee, 80)),
                    ),
                }
            }
            "new_expression" => {
                let Some(c) = n.child_by_field_name("constructor") else {
                    return Action::Descend;
                };
                match c.kind() {
                    "identifier" => Action::Ref(RefKind::Call, self.text(c).to_string(), None),
                    "member_expression" => Action::Ref(
                        RefKind::Method,
                        self.field_text(c, "property").unwrap_or("").to_string(),
                        self.field_text(c, "object").map(|s| truncate(s, 80)),
                    ),
                    _ => Action::Descend,
                }
            }
            "import_statement" => {
                self.js_import(n);
                Action::Skip
            }
            "export_statement" => {
                if let Some(src) = self.field_text(n, "source") {
                    self.push_import(Import {
                        module: None,
                        spec: format!("{}#*", unquote(src)),
                        alias: None,
                        line: Self::line(n),
                    });
                }
                Action::Descend
            }
            "comment" | "string" | "template_string" | "regex" => Action::Skip,
            _ => Action::Descend,
        }
    }

    fn js_import(&mut self, n: Node<'_>) {
        let Some(src) = self.field_text(n, "source") else {
            return;
        };
        let base = unquote(src).to_string();
        let line = Self::line(n);
        let mut found = false;
        let mut cursor = n.walk();
        let clauses: Vec<Node<'_>> = n
            .named_children(&mut cursor)
            .filter(|c| c.kind() == "import_clause")
            .collect();
        for clause in clauses {
            let mut cc = clause.walk();
            let parts: Vec<Node<'_>> = clause.named_children(&mut cc).collect();
            for part in parts {
                match part.kind() {
                    "identifier" => {
                        found = true;
                        self.push_import(Import {
                            module: None,
                            spec: format!("{base}#default"),
                            alias: Some(self.text(part).to_string()),
                            line,
                        });
                    }
                    "namespace_import" => {
                        found = true;
                        let mut c3 = part.walk();
                        let alias = part
                            .named_children(&mut c3)
                            .find(|c| c.kind() == "identifier")
                            .map(|c| self.text(c).to_string());
                        self.push_import(Import {
                            module: None,
                            spec: format!("{base}#*"),
                            alias,
                            line,
                        });
                    }
                    "named_imports" => {
                        let mut c3 = part.walk();
                        let specs: Vec<Node<'_>> = part
                            .named_children(&mut c3)
                            .filter(|c| c.kind() == "import_specifier")
                            .collect();
                        for s in specs {
                            found = true;
                            let name = self.field_text(s, "name").unwrap_or("");
                            let alias = self.field_text(s, "alias").unwrap_or(name);
                            self.push_import(Import {
                                module: None,
                                spec: format!("{base}#{}", unquote(name)),
                                alias: Some(alias.to_string()),
                                line,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        if !found {
            self.push_import(Import {
                module: None,
                spec: format!("{base}#"),
                alias: None,
                line,
            });
        }
    }

    /// `const x = require("./m")` and `const { a, b } = require("./m")`.
    fn js_require(&mut self, name: Node<'_>, call: Node<'_>) {
        let is_require = call
            .child_by_field_name("function")
            .is_some_and(|f| f.kind() == "identifier" && self.text(f) == "require");
        if !is_require {
            return;
        }
        let Some(arg) = call
            .child_by_field_name("arguments")
            .and_then(|a| a.named_child(0))
            .filter(|a| a.kind() == "string")
        else {
            return;
        };
        let base = unquote(self.text(arg)).to_string();
        let line = Self::line(call);
        match name.kind() {
            "identifier" => self.push_import(Import {
                module: None,
                spec: format!("{base}#*"),
                alias: Some(self.text(name).to_string()),
                line,
            }),
            "object_pattern" => {
                let mut cursor = name.walk();
                let props: Vec<Node<'_>> = name.named_children(&mut cursor).collect();
                for p in props {
                    let (member, alias) = match p.kind() {
                        "shorthand_property_identifier_pattern" => (self.text(p), self.text(p)),
                        "pair_pattern" => (
                            self.field_text(p, "key").unwrap_or(""),
                            self.field_text(p, "value").unwrap_or(""),
                        ),
                        _ => continue,
                    };
                    self.push_import(Import {
                        module: None,
                        spec: format!("{base}#{member}"),
                        alias: Some(alias.to_string()),
                        line,
                    });
                }
            }
            _ => {}
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max - 1).collect();
    out.push('…');
    out
}

/// Associated functions that conventionally return `Self`.
fn is_constructor_name(n: &str) -> bool {
    matches!(
        n,
        "new" | "default" | "open" | "create" | "connect" | "load" | "init" | "build"
    ) || n.starts_with("new_")
        || n.starts_with("with_")
        || n.starts_with("from")
        || n.starts_with("open_")
}

/// The nominal type in an annotation: `&mut Engine` -> Engine, `: Foo` -> Foo,
/// `crate::a::Db` -> Db. Generic wrappers, unions and lowercase names give `None`.
fn type_name(raw: &str) -> Option<String> {
    let t = raw.trim().trim_start_matches(':').trim();
    let t = t
        .trim_start_matches('&')
        .trim_start_matches("mut ")
        .trim_start_matches("dyn ")
        .trim();
    let t = strip_generics(t);
    let t = t.split('[').next().unwrap_or("").trim();
    let last = t.rsplit("::").next().unwrap_or(t);
    let last = last.rsplit('.').next().unwrap_or(last).trim();
    let ok = last.chars().next().is_some_and(|c| c.is_uppercase())
        && last.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !matches!(
            last,
            "Self"
                | "Option"
                | "Result"
                | "Vec"
                | "Box"
                | "Rc"
                | "Arc"
                | "Optional"
                | "List"
                | "Dict"
        );
    ok.then(|| last.to_string())
}

/// `Vec<T>` -> `Vec`; `a::B<T>` -> `a::B`.
fn strip_generics(s: &str) -> String {
    let mut depth = 0;
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out.trim_start_matches('&').trim().to_string()
}

/// Strip quotes and Python string prefixes.
pub fn unquote(s: &str) -> &str {
    // Python prefixes (r"", b'', f"", rb"") only count when a quote follows.
    let prefix = s.find(['"', '\'', '`']).unwrap_or(0);
    let s = if prefix <= 2 && s[..prefix].chars().all(|c| "rbfuRBFU".contains(c)) {
        &s[prefix..]
    } else {
        s
    };
    for q in ["\"\"\"", "'''", "\"", "'", "`"] {
        if s.len() >= 2 * q.len() && s.starts_with(q) && s.ends_with(q) {
            return &s[q.len()..s.len() - q.len()];
        }
    }
    s
}

fn clean_block_comment(t: &str) -> String {
    t.trim_start_matches("/**")
        .trim_end_matches("*/")
        .lines()
        .map(|l| l.trim().trim_start_matches('*').trim())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(lang: Lang, path: &str, src: &str) -> Parsed {
        FileParser::new().parse(lang, path, src.as_bytes()).unwrap()
    }

    fn def<'p>(p: &'p Parsed, qname: &str) -> &'p Def {
        p.defs.iter().find(|d| d.qname == qname).unwrap_or_else(|| {
            panic!(
                "no def {qname}; have {:?}",
                p.defs.iter().map(|d| &d.qname).collect::<Vec<_>>()
            )
        })
    }

    fn refs_from(p: &Parsed, qname: &str) -> Vec<(RefKind, String, Option<String>)> {
        let idx = p.defs.iter().position(|d| d.qname == qname).unwrap();
        p.refs
            .iter()
            .filter(|r| r.from == idx)
            .map(|r| (r.kind, r.name.clone(), r.qualifier.clone()))
            .collect()
    }

    #[test]
    fn rust_defs_refs_imports() {
        let src = r#"
use crate::store::{TokenStore, clock::Clock as C};
mod helpers;

/// Issues a new session.
///
/// More detail.
pub fn refresh(token: &Token) -> Result<Session, AuthError> {
    let s = TokenStore::lookup(token)?;
    let now = C::now();
    s.validate(now);
    Session::new(s)
}

impl<T> Display for Wrapper<T> {
    fn fmt(&self) {}
}
impl Debug for Wrapper<u8> {
    fn fmt(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn refreshes() { super::refresh(&t()); }
}
"#;
        let p = parse(Lang::Rust, "src/auth/session.rs", src);
        let f = def(&p, "auth::session::refresh");
        assert_eq!(f.kind, "function");
        assert_eq!(
            f.signature,
            "pub fn refresh(token: &Token) -> Result<Session, AuthError>"
        );
        assert_eq!(f.doc.as_deref(), Some("Issues a new session."));
        assert_eq!(f.start_line, 8);
        assert_eq!(def(&p, "auth::session::Wrapper::fmt").kind, "method");
        assert!(def(&p, "auth::session::Wrapper::fmt#2").kind == "method");
        assert!(def(&p, "auth::session::tests::refreshes").is_test);
        let refs = refs_from(&p, "auth::session::refresh");
        assert!(refs.contains(&(RefKind::Path, "lookup".into(), Some("TokenStore".into()))));
        assert!(refs.contains(&(RefKind::Path, "now".into(), Some("C".into()))));
        assert!(refs.contains(&(RefKind::Method, "validate".into(), Some("s".into()))));
        assert!(refs.contains(&(RefKind::Path, "new".into(), Some("Session".into()))));
        let specs: Vec<(String, Option<String>)> = p
            .imports
            .iter()
            .map(|i| (i.spec.clone(), i.alias.clone()))
            .collect();
        assert!(specs.contains(&("crate::store::TokenStore".into(), Some("TokenStore".into()))));
        assert!(specs.contains(&("crate::store::clock::Clock".into(), Some("C".into()))));
        assert!(specs.contains(&("self::helpers".into(), Some("helpers".into()))));
    }

    #[test]
    fn local_types_turn_method_calls_into_paths() {
        let rs = parse(
            Lang::Rust,
            "src/lib.rs",
            "fn f(engine: &mut Engine) { let mut r = Resolver::new(); let s = Store { a: 1 }; \
             let d: Db = open(); engine.explain(); r.resolve(); s.get(); d.query(); other.x(); }",
        );
        let refs = refs_from(&rs, "f");
        for (name, ty) in [
            ("explain", "Engine"),
            ("resolve", "Resolver"),
            ("get", "Store"),
            ("query", "Db"),
        ] {
            assert!(
                refs.contains(&(RefKind::Path, name.into(), Some(ty.into()))),
                "{name}: {refs:?}"
            );
        }
        assert!(refs.contains(&(RefKind::Method, "x".into(), Some("other".into()))));

        let py = parse(
            Lang::Python,
            "a.py",
            "def f(db: Db):\n    s = Session()\n    s.go()\n    db.q()\n",
        );
        let refs = refs_from(&py, "a.f");
        assert!(refs.contains(&(RefKind::Path, "go".into(), Some("Session".into()))));
        assert!(refs.contains(&(RefKind::Path, "q".into(), Some("Db".into()))));

        let ts = parse(
            Lang::TypeScript,
            "a.ts",
            "function f(db: Db) { const s = new Session(); s.go(); db.q(); }",
        );
        let refs = refs_from(&ts, "f");
        assert!(
            refs.contains(&(RefKind::Path, "go".into(), Some("Session".into()))),
            "{refs:?}"
        );
        assert!(refs.contains(&(RefKind::Path, "q".into(), Some("Db".into()))));
    }

    #[test]
    fn unquote_strings() {
        assert_eq!(unquote("refresh"), "refresh");
        assert_eq!(unquote("\"./store\""), "./store");
        assert_eq!(unquote("r'raw'"), "raw");
        assert_eq!(unquote("f\"\"\"doc\"\"\""), "doc");
        assert_eq!(unquote("`tpl`"), "tpl");
    }

    #[test]
    fn rust_calls_inside_macros() {
        let p = parse(
            Lang::Rust,
            "src/lib.rs",
            "fn t() { let s = Store::new(); assert!(refresh(&s, \"x\").is_none()); \
             assert_eq!(crate::a::f(1), s.get(2)); println!(\"{}\", g()); }",
        );
        let refs = refs_from(&p, "t");
        assert!(
            refs.contains(&(RefKind::Call, "refresh".into(), None)),
            "{refs:?}"
        );
        assert!(refs.contains(&(RefKind::Path, "f".into(), Some("crate::a".into()))));
        assert!(refs.contains(&(RefKind::Path, "get".into(), Some("Store".into()))));
        assert!(refs.contains(&(RefKind::Call, "g".into(), None)));
    }

    #[test]
    fn python_defs_refs_imports() {
        let src = r#"
import os.path
from .store import TokenStore as TS, lookup
from ..util import *

class Session:
    """A login session."""
    def refresh(self, token):
        t = TS.find(token)
        self.validate(t)
        return lookup(t)

def test_refresh():
    Session().refresh(1)
"#;
        let p = parse(Lang::Python, "pkg/auth/test_session.py", src);
        assert_eq!(
            def(&p, "pkg.auth.test_session.Session").doc.as_deref(),
            Some("A login session.")
        );
        assert_eq!(
            def(&p, "pkg.auth.test_session.Session.refresh").kind,
            "method"
        );
        assert!(def(&p, "pkg.auth.test_session.test_refresh").is_test);
        let refs = refs_from(&p, "pkg.auth.test_session.Session.refresh");
        assert!(refs.contains(&(RefKind::Method, "find".into(), Some("TS".into()))));
        assert!(refs.contains(&(RefKind::Method, "validate".into(), Some("self".into()))));
        assert!(refs.contains(&(RefKind::Call, "lookup".into(), None)));
        let specs: Vec<(String, Option<String>)> = p
            .imports
            .iter()
            .map(|i| (i.spec.clone(), i.alias.clone()))
            .collect();
        assert!(specs.contains(&("os.path".into(), Some("os.path".into()))));
        assert!(specs.contains(&(".store.TokenStore".into(), Some("TS".into()))));
        assert!(specs.contains(&(".store.lookup".into(), Some("lookup".into()))));
        assert!(specs.contains(&("..util.*".into(), Some("*".into()))));
    }

    #[test]
    fn typescript_defs_refs_imports_tests() {
        let src = r#"
import { lookup as find, Clock } from "./store";
import * as util from "../util";
const legacy = require("./legacy");

/** Issues a new session. */
export function refresh(token: Token): Session {
  const s = find(token);
  util.log(s);
  return new Session(s);
}

export class Session {
  validate(): boolean { return this.check(); }
  check() { return true; }
}

export const helper = (x: number) => x + 1;

describe("refresh", () => {
  it("returns a session", () => { refresh(t); });
});
"#;
        let p = parse(Lang::TypeScript, "src/auth/session.ts", src);
        let f = def(&p, "refresh");
        assert_eq!(f.doc.as_deref(), Some("Issues a new session."));
        assert_eq!(f.signature, "function refresh(token: Token): Session");
        assert_eq!(def(&p, "Session.validate").kind, "method");
        assert_eq!(def(&p, "helper").kind, "function");
        let t = def(&p, "refresh.returns a session");
        assert!(t.is_test);
        assert_eq!(
            refs_from(&p, "refresh.returns a session"),
            [(RefKind::Call, "refresh".into(), None)]
        );
        let refs = refs_from(&p, "refresh");
        assert!(refs.contains(&(RefKind::Call, "find".into(), None)));
        assert!(refs.contains(&(RefKind::Method, "log".into(), Some("util".into()))));
        assert!(refs.contains(&(RefKind::Call, "Session".into(), None)));
        assert_eq!(
            refs_from(&p, "Session.validate"),
            [(RefKind::Method, "check".into(), Some("this".into()))]
        );
        let specs: Vec<(String, Option<String>)> = p
            .imports
            .iter()
            .map(|i| (i.spec.clone(), i.alias.clone()))
            .collect();
        assert!(specs.contains(&("./store#lookup".into(), Some("find".into()))));
        assert!(specs.contains(&("../util#*".into(), Some("util".into()))));
        assert!(specs.contains(&("./legacy#*".into(), Some("legacy".into()))));
    }
}
