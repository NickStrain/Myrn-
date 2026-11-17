//! Cross-file resolution: raw references and imports into `calls`, `imports`
//! and `tests` edges.
//!
//! Resolution runs over the whole graph after every refresh, so edges into a
//! changed file are always rebuilt. An edge is `resolved` when it follows from
//! scope or an import, and `inferred` when it rests on a unique name match;
//! anything more ambiguous is left out rather than guessed.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Result;
use myrn_model::id::{file_id, parent_dir};
use myrn_model::EdgeStatus;
use rusqlite::{params, Connection};

use super::lang::{self, Family, Lang};
use super::parse::RefKind;

/// A symbol or test, as the resolver sees it.
#[derive(Debug, Clone)]
pub struct Sym {
    pub id: String,
    pub qname: String,
    pub name: String,
    pub kind: String,
    pub path: String,
    pub lang: Lang,
    pub is_test: bool,
}

impl Sym {
    /// Qualified name without a `#n` collision suffix.
    fn base_qname(&self) -> &str {
        strip_suffix(&self.qname)
    }

    /// Enclosing scope's qualified name.
    fn parent(&self) -> &str {
        let q = self.base_qname();
        match q.rfind(self.lang.sep()) {
            Some(i) => &q[..i],
            None => "",
        }
    }

    fn callable(&self) -> bool {
        !self.is_test
            && matches!(
                self.kind.as_str(),
                "function" | "method" | "class" | "struct"
            )
    }
}

fn strip_suffix(q: &str) -> &str {
    match q.rfind('#') {
        Some(i) if q[i + 1..].chars().all(|c| c.is_ascii_digit()) => &q[..i],
        _ => q,
    }
}

/// What a local name refers to after an import. Both fields `None` means the
/// import points outside the repository: calls through it are never guessed.
#[derive(Debug, Clone)]
struct Binding {
    file: Option<String>,
    /// Qualified name of the imported item or module.
    qname: Option<String>,
}

impl Binding {
    fn external(&self) -> bool {
        self.file.is_none() && self.qname.is_none()
    }
}

/// One raw reference row.
#[derive(Debug, Clone)]
pub struct RawRef {
    pub src: String,
    pub path: String,
    pub kind: String,
    pub name: String,
    pub qualifier: Option<String>,
    pub scope: Option<String>,
}

/// Method names too common to infer from the name alone.
const COMMON: &[&str] = &[
    "add",
    "append",
    "apply",
    "call",
    "clear",
    "clone",
    "close",
    "contains",
    "debug",
    "default",
    "error",
    "extend",
    "filter",
    "find",
    "first",
    "flush",
    "format",
    "from",
    "get",
    "info",
    "init",
    "insert",
    "into",
    "is_empty",
    "items",
    "iter",
    "join",
    "keys",
    "last",
    "len",
    "load",
    "map",
    "new",
    "next",
    "open",
    "parse",
    "pop",
    "push",
    "read",
    "remove",
    "run",
    "save",
    "send",
    "set",
    "sort",
    "split",
    "start",
    "stop",
    "to_string",
    "trim",
    "unwrap",
    "update",
    "values",
    "warn",
    "write",
];

pub struct Resolver {
    syms: Vec<Sym>,
    by_name: HashMap<String, Vec<usize>>,
    by_qname: HashMap<String, Vec<usize>>,
    by_path: HashMap<String, Vec<usize>>,
    by_id: HashMap<String, usize>,
    rust_mods: HashMap<(String, String), String>,
    rust_crates: HashMap<String, String>,
    py_mods: HashMap<String, String>,
    js_files: HashSet<String>,
    bindings: HashMap<String, HashMap<String, Binding>>,
    /// Wildcard imports per file: (target file, module qname).
    globs: HashMap<String, Vec<(String, String)>>,
    /// Files each file imports.
    imports: HashMap<String, HashSet<String>>,
}

impl Resolver {
    /// `crates` maps a Rust crate name (underscored) to its directory.
    pub fn new(
        syms: Vec<Sym>,
        files: &[(String, Lang)],
        crates: HashMap<String, String>,
    ) -> Resolver {
        let mut r = Resolver {
            by_name: HashMap::new(),
            by_qname: HashMap::new(),
            by_path: HashMap::new(),
            by_id: HashMap::new(),
            rust_mods: HashMap::new(),
            rust_crates: crates,
            py_mods: HashMap::new(),
            js_files: HashSet::new(),
            bindings: HashMap::new(),
            globs: HashMap::new(),
            imports: HashMap::new(),
            syms,
        };
        for (i, s) in r.syms.iter().enumerate() {
            r.by_name.entry(s.name.clone()).or_default().push(i);
            r.by_qname
                .entry(s.base_qname().to_string())
                .or_default()
                .push(i);
            r.by_path.entry(s.path.clone()).or_default().push(i);
            r.by_id.insert(s.id.clone(), i);
        }
        let mut sorted: Vec<&(String, Lang)> = files.iter().collect();
        // lib.rs before main.rs, so a crate root resolves to the library.
        sorted.sort_by_key(|(p, _)| (!p.ends_with("lib.rs"), p.clone()));
        for (path, l) in sorted {
            match l.family() {
                Family::Rust => {
                    let (krate, m) = lang::rust_module(path);
                    r.rust_mods
                        .entry((krate, m.join("::")))
                        .or_insert_with(|| path.clone());
                }
                Family::Python => {
                    r.py_mods
                        .entry(lang::python_module(path).join("."))
                        .or_insert_with(|| path.clone());
                }
                Family::Js => {
                    r.js_files.insert(path.clone());
                }
            }
        }
        r
    }

    pub fn sym(&self, i: usize) -> &Sym {
        &self.syms[i]
    }

    // ------------------------------------------------------------ imports

    /// Record an import; returns the imported file, if it is in this repository.
    pub fn add_import(
        &mut self,
        file: &str,
        spec: &str,
        alias: Option<&str>,
        scope: Option<&str>,
    ) -> Option<String> {
        let l = Lang::from_path(file)?;
        let resolved = match l.family() {
            Family::Rust => self.rust_import(file, spec, scope),
            Family::Python => self.py_import(file, spec),
            Family::Js => self.js_import(file, spec),
        };
        if let Some((t, _, _)) = &resolved {
            self.imports
                .entry(file.to_string())
                .or_default()
                .insert(t.clone());
        }
        let Some((target, qname, glob)) = resolved else {
            if let Some(a) = alias.filter(|a| *a != "*") {
                self.bindings
                    .entry(file.to_string())
                    .or_default()
                    .entry(a.to_string())
                    .or_insert(Binding {
                        file: None,
                        qname: None,
                    });
            }
            return None;
        };
        if glob && alias.is_some_and(|a| a != "*") {
            // A named namespace import (`import * as R from "./index"`): `R`
            // stands for the whole file; `R.f()` resolves through it.
            self.bindings.entry(file.to_string()).or_default().insert(
                alias.unwrap().to_string(),
                Binding {
                    file: Some(target.clone()),
                    qname: None,
                },
            );
        } else if glob {
            self.globs
                .entry(file.to_string())
                .or_default()
                .push((target.clone(), qname.unwrap_or_default()));
        } else if let Some(a) = alias {
            self.bindings.entry(file.to_string()).or_default().insert(
                a.to_string(),
                Binding {
                    file: Some(target.clone()),
                    qname,
                },
            );
        }
        Some(target)
    }

    /// Resolve a Rust path to (module file, full qualified name).
    fn rust_path(
        &self,
        file: &str,
        scope: Option<&str>,
        segs: &[&str],
    ) -> Option<(String, String)> {
        let (krate, file_mod) = lang::rust_module(file);
        let cur: Vec<String> = match scope {
            Some(s) if !s.is_empty() => s.split("::").map(str::to_string).collect(),
            Some(_) => Vec::new(),
            None => file_mod,
        };
        let first = *segs.first()?;
        let mut attempts: Vec<(String, Vec<String>, &[&str], bool)> = Vec::new();
        match first {
            "crate" => attempts.push((krate.clone(), Vec::new(), &segs[1..], true)),
            "self" => attempts.push((krate.clone(), cur.clone(), &segs[1..], true)),
            "super" => {
                let ups = segs.iter().take_while(|s| **s == "super").count();
                let base = cur[..cur.len().saturating_sub(ups)].to_vec();
                attempts.push((krate.clone(), base, &segs[ups..], true));
            }
            name if self.rust_crates.contains_key(name) => {
                attempts.push((self.rust_crates[name].clone(), Vec::new(), &segs[1..], true));
            }
            _ => {
                attempts.push((krate.clone(), cur.clone(), segs, false));
                attempts.push((krate.clone(), Vec::new(), segs, false));
            }
        }
        for (k, base, rest, explicit) in attempts {
            let full: Vec<String> = base
                .iter()
                .cloned()
                .chain(rest.iter().map(|s| s.to_string()))
                .collect();
            for n in (0..=full.len()).rev() {
                if n < base.len() {
                    break;
                }
                let Some(f) = self.rust_mods.get(&(k.clone(), full[..n].join("::"))) else {
                    continue;
                };
                let qname = full.join("::");
                let ok = explicit || n > base.len() || self.lookup_in_crate(&qname, f).is_some();
                if ok {
                    return Some((f.clone(), qname));
                }
                break;
            }
        }
        None
    }

    fn rust_import(
        &self,
        file: &str,
        spec: &str,
        scope: Option<&str>,
    ) -> Option<(String, Option<String>, bool)> {
        let (path, glob) = match spec.strip_suffix("::*") {
            Some(p) => (p, true),
            None => (spec, false),
        };
        let segs: Vec<&str> = path.split("::").collect();
        let (f, q) = self.rust_path(file, scope, &segs)?;
        Some((f, Some(q), glob))
    }

    fn py_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)> {
        let (spec, glob) = match spec.strip_suffix(".*").or_else(|| spec.strip_suffix('*')) {
            Some(p) => (p, true),
            None => (spec, false),
        };
        let dots = spec.chars().take_while(|c| *c == '.').count();
        let rest: Vec<String> = spec[dots..]
            .split('.')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        let mut full: Vec<String> = Vec::new();
        let mut min = 1;
        if dots > 0 {
            let mut pkg = lang::python_module(file);
            if !file.ends_with("__init__.py") {
                pkg.pop();
            }
            for _ in 1..dots {
                pkg.pop();
            }
            min = pkg.len();
            full.extend(pkg);
        }
        full.extend(rest);
        for n in (min.max(1)..=full.len()).rev() {
            let key = full[..n].join(".");
            if let Some(f) = self.py_mods.get(&key) {
                return Some((f.clone(), Some(full.join(".")), glob));
            }
            if dots == 0 {
                // Source roots below the repository root (`py/app/x.py` imported as `app.x`).
                let suffix = format!(".{key}");
                let mut hits = self.py_mods.iter().filter(|(k, _)| k.ends_with(&suffix));
                if let (Some((k, f)), None) = (hits.next(), hits.next()) {
                    let prefix = &k[..k.len() - key.len()];
                    return Some((f.clone(), Some(format!("{prefix}{}", full.join("."))), glob));
                }
            }
        }
        None
    }

    fn js_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)> {
        let (p, member) = spec.split_once('#').unwrap_or((spec, ""));
        if !p.starts_with('.') {
            return None;
        }
        let joined = normalize(&format!("{}/{}", parent_dir(file), p));
        let stem = joined
            .strip_suffix(".js")
            .or_else(|| joined.strip_suffix(".jsx"))
            .or_else(|| joined.strip_suffix(".mjs"))
            .unwrap_or(&joined);
        const EXTS: [&str; 8] = [".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".mts", ".cts"];
        let mut candidates = vec![joined.clone()];
        for base in [stem.to_string(), format!("{joined}/index")] {
            candidates.extend(EXTS.iter().map(|e| format!("{base}{e}")));
        }
        let target = candidates.into_iter().find(|c| self.js_files.contains(c))?;
        let qname = match member {
            "" | "*" | "default" => None,
            m => Some(m.to_string()),
        };
        // `export * from "./x"` (no alias) re-exports everything; a named
        // `import * as R` becomes a binding in `add_import`.
        Some((target, qname, member == "*"))
    }

    // -------------------------------------------------------------- calls

    /// Symbols with this qualified name, preferring the given file, then its crate/package.
    fn lookup(&self, qname: &str, near: Option<&str>) -> Vec<usize> {
        let Some(all) = self.by_qname.get(qname) else {
            return Vec::new();
        };
        // Never across languages: a Rust path must not land on a TypeScript symbol.
        let family = near.and_then(Lang::from_path).map(Lang::family);
        let all: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&i| {
                !self.syms[i].is_test && family.is_none_or(|f| self.syms[i].lang.family() == f)
            })
            .collect();
        if all.len() <= 1 {
            return all;
        }
        if let Some(near) = near {
            let same: Vec<usize> = all
                .iter()
                .copied()
                .filter(|&i| self.syms[i].path == near)
                .collect();
            if !same.is_empty() {
                return same;
            }
            if let Some(i) = self.lookup_in_crate(qname, near) {
                return vec![i];
            }
        }
        all
    }

    fn lookup_in_crate(&self, qname: &str, near: &str) -> Option<usize> {
        let krate = lang::rust_module(near).0;
        self.by_qname.get(qname)?.iter().copied().find(|&i| {
            let s = &self.syms[i];
            !s.is_test && s.lang.family() == Family::Rust && lang::rust_module(&s.path).0 == krate
        })
    }

    /// Resolve one call reference made from symbol `src`.
    pub fn resolve(&self, src: usize, r: &RawRef) -> Vec<(usize, EdgeStatus)> {
        let kind = RefKind::parse(&r.kind).unwrap_or(RefKind::Call);
        let s = &self.syms[src];
        let found = match kind {
            RefKind::Call => self.resolve_bare(s, &r.name),
            RefKind::Path => self.resolve_path(
                s,
                &r.name,
                r.qualifier.as_deref().unwrap_or(""),
                r.scope.as_deref(),
            ),
            RefKind::Method => {
                self.resolve_method(s, &r.name, r.qualifier.as_deref().unwrap_or(""))
            }
        };
        let mut seen = HashSet::new();
        found.into_iter().filter(|(i, _)| seen.insert(*i)).collect()
    }

    fn resolved(v: Vec<usize>) -> Vec<(usize, EdgeStatus)> {
        v.into_iter().map(|i| (i, EdgeStatus::Resolved)).collect()
    }

    fn binding(&self, file: &str, name: &str) -> Option<&Binding> {
        self.bindings.get(file)?.get(name)
    }

    fn resolve_bare(&self, s: &Sym, name: &str) -> Vec<(usize, EdgeStatus)> {
        let fam = s.lang.family();
        // 1. An imported name.
        if let Some(b) = self.binding(&s.path, name) {
            if b.external() {
                return Vec::new();
            }
            if let Some(q) = &b.qname {
                let mut hits = self.lookup(q, b.file.as_deref());
                if hits.is_empty() {
                    if let Some(f) = &b.file {
                        hits = self.reexported(f, last_seg(q, s.lang.sep()));
                    }
                }
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
            }
            if let Some(f) = &b.file {
                // JS default import: a top-level symbol of that name in the target file.
                let hits = self.in_file(f, |t| t.name == name && t.parent().is_empty());
                if hits.len() == 1 {
                    return vec![(hits[0], EdgeStatus::Inferred)];
                }
            }
        }
        // 2. Defined in the same file (not a method: bare calls cannot reach those).
        let local = self.in_file(&s.path, |t| {
            t.name == name && t.callable() && t.kind != "method"
        });
        if !local.is_empty() {
            let best = local
                .iter()
                .copied()
                .max_by_key(|&i| common_prefix(self.syms[i].parent(), s.base_qname()))
                .unwrap();
            return vec![(best, EdgeStatus::Resolved)];
        }
        // 3. Wildcard imports.
        if let Some(globs) = self.globs.get(&s.path) {
            for (file, module) in globs {
                let q = if module.is_empty() {
                    name.to_string()
                } else {
                    format!("{module}{}{name}", s.lang.sep())
                };
                let hits = self.lookup(&q, Some(file));
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
            }
        }
        // 4. A unique function of that name anywhere in the same language family.
        let global: Vec<usize> = self
            .by_name
            .get(name)
            .map(|v| {
                v.iter()
                    .copied()
                    .filter(|&i| {
                        let t = &self.syms[i];
                        t.lang.family() == fam
                            && t.callable()
                            && t.kind != "method"
                            && self.reachable(&s.path, &t.path)
                    })
                    .collect()
            })
            .unwrap_or_default();
        if global.len() == 1 && !COMMON.contains(&name) {
            return vec![(global[0], EdgeStatus::Inferred)];
        }
        Vec::new()
    }

    fn resolve_path(
        &self,
        s: &Sym,
        name: &str,
        q: &str,
        scope: Option<&str>,
    ) -> Vec<(usize, EdgeStatus)> {
        if s.lang.family() != Family::Rust {
            return self.resolve_method(s, name, q);
        }
        // Self::f
        if q == "Self" {
            let hits = self.lookup(&format!("{}::{name}", s.parent()), Some(&s.path));
            return Self::resolved(hits);
        }
        let segs: Vec<&str> = q.split("::").chain(std::iter::once(name)).collect();
        // Alias::f, alias::sub::f
        if let Some(b) = self.binding(&s.path, segs[0]) {
            if b.external() {
                return Vec::new();
            }
            if let (Some(bq), Some(bf)) = (&b.qname, &b.file) {
                let q = std::iter::once(bq.as_str())
                    .chain(segs[1..].iter().copied())
                    .collect::<Vec<_>>()
                    .join("::");
                let hits = self.follow_rust(bf, &q, 3);
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
            }
        }
        // crate::a::f, super::f, module::f, Type::f in scope
        if let Some((f, qn)) = self.rust_path(&s.path, scope, &segs) {
            let hits = self.follow_rust(&f, &qn, 3);
            if !hits.is_empty() {
                return Self::resolved(hits);
            }
        }
        // Type::f where Type is defined elsewhere: match by type name.
        let type_name = segs[segs.len() - 2];
        let cands: Vec<usize> = self
            .by_name
            .get(name)
            .map(|v| {
                v.iter()
                    .copied()
                    .filter(|&i| {
                        let t = &self.syms[i];
                        t.lang.family() == Family::Rust
                            && t.callable()
                            && last_seg(t.parent(), "::") == type_name
                            && self.reachable(&s.path, &t.path)
                    })
                    .collect()
            })
            .unwrap_or_default();
        let krate = lang::rust_module(&s.path).0;
        let same_crate: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|&i| lang::rust_module(&self.syms[i].path).0 == krate)
            .collect();
        match (same_crate.len(), cands.len()) {
            (1, _) => vec![(same_crate[0], EdgeStatus::Inferred)],
            (_, 1) => vec![(cands[0], EdgeStatus::Inferred)],
            _ => Vec::new(),
        }
    }

    /// Look up a Rust qualified name whose module lives in `file`, following
    /// `pub use` re-exports there (`krate::Searcher` where lib.rs has
    /// `pub use crate::searcher::Searcher`), up to `depth` hops.
    fn follow_rust(&self, file: &str, qname: &str, depth: usize) -> Vec<usize> {
        let hits = self.lookup(qname, Some(file));
        if !hits.is_empty() || depth == 0 {
            return hits;
        }
        let module = lang::rust_module(file).1.join("::");
        let item = if module.is_empty() {
            qname
        } else {
            qname.strip_prefix(&format!("{module}::")).unwrap_or(qname)
        };
        let mut parts = item.split("::");
        let Some(head) = parts.next() else {
            return Vec::new();
        };
        match self.binding(file, head) {
            Some(Binding {
                file: Some(bf),
                qname: Some(bq),
            }) if bq != qname => {
                let full = std::iter::once(bq.as_str())
                    .chain(parts)
                    .collect::<Vec<_>>()
                    .join("::");
                self.follow_rust(bf, &full, depth - 1)
            }
            _ => Vec::new(),
        }
    }

    /// Follow a `use` binding in `file` for `name` (re-exports).
    fn reexported(&self, file: &str, name: &str) -> Vec<usize> {
        match self.binding(file, name) {
            Some(Binding {
                file: bf,
                qname: Some(q),
            }) => self.lookup(q, bf.as_deref()),
            _ => self.exported_from(file, name, 3),
        }
    }

    /// `name` as seen from outside `file`: a top-level symbol of the file, a
    /// named re-export, or a wildcard re-export (`from .recipes import *`,
    /// `export * from "./chunk"`), followed up to `depth` files deep.
    fn exported_from(&self, file: &str, name: &str, depth: usize) -> Vec<usize> {
        let own = self.in_file(file, |t| t.name == name && self.is_top_level(t));
        if !own.is_empty() || depth == 0 {
            return own;
        }
        if let Some(Binding {
            file: bf,
            qname: Some(q),
        }) = self.binding(file, name)
        {
            let hits = self.lookup(q, bf.as_deref());
            if !hits.is_empty() {
                return hits;
            }
        }
        if let Some(globs) = self.globs.get(file) {
            for (target, _) in globs {
                if target != file {
                    let hits = self.exported_from(target, name, depth - 1);
                    if !hits.is_empty() {
                        return hits;
                    }
                }
            }
        }
        Vec::new()
    }

    fn resolve_method(&self, s: &Sym, name: &str, q: &str) -> Vec<(usize, EdgeStatus)> {
        let fam = s.lang.family();
        let sep = s.lang.sep();
        // self.f() / this.f() / cls.f(): a method of the enclosing type.
        // super.f() / super().f(): the parent class is not known here.
        if q == "super" || q.starts_with("super(") {
            return Vec::new();
        }
        if matches!(q, "self" | "this" | "cls") {
            let mut prefix = s.parent();
            while !prefix.is_empty() {
                let hits = self.lookup(&format!("{prefix}{sep}{name}"), Some(&s.path));
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
                prefix = match prefix.rfind(sep) {
                    Some(i) => &prefix[..i],
                    None => "",
                };
            }
        }
        // A receiver rooted in an outside import (`Command::new(..).output()`, `os.path.join`).
        let root: String = q
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if self.binding(&s.path, &root).is_some_and(Binding::external) {
            return Vec::new();
        }
        // module.f() / Class.f() through an import binding.
        let head = q.split('.').next().unwrap_or(q);
        let mut known_receiver = false;
        for key in [q, head] {
            let Some(b) = self.binding(&s.path, key) else {
                continue;
            };
            known_receiver = true;
            let tail = &q[key.len()..];
            if let Some(bq) = &b.qname {
                let hits = self.lookup(&format!("{bq}{}{sep}{name}", tail), b.file.as_deref());
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
            }
            if let (Some(f), true) = (&b.file, tail.is_empty()) {
                // Namespace import: a top-level symbol of the target file,
                // or one it re-exports (`import pkg as p; p.f()`).
                let hits: Vec<usize> = self
                    .exported_from(f, name, 3)
                    .into_iter()
                    .filter(|&i| self.syms[i].callable())
                    .collect();
                if !hits.is_empty() {
                    return Self::resolved(hits);
                }
            }
        }
        // Class.f() for a class in the same file.
        let local = self.in_file(&s.path, |t| {
            t.name == name && t.callable() && last_seg(t.parent(), sep) == q
        });
        if !local.is_empty() {
            return Self::resolved(local);
        }
        // The receiver is a known import but has no such member: do not guess.
        if known_receiver {
            return Vec::new();
        }
        // A unique method name, or one whose type matches the receiver's name.
        let cands: Vec<usize> = self
            .by_name
            .get(name)
            .map(|v| {
                v.iter()
                    .copied()
                    .filter(|&i| {
                        let t = &self.syms[i];
                        t.lang.family() == fam
                            && t.callable()
                            && t.kind == "method"
                            && self.parent_is_type(t)
                    })
                    .collect()
            })
            .unwrap_or_default();
        // The receiver names the type (`SummaryBuilder::new().stats()`, `token_store.find()`).
        let words: HashSet<String> = myrn_store::fts::split_identifier(q).into_iter().collect();
        let hinted: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|&i| {
                let ty = myrn_store::fts::split_identifier(last_seg(self.syms[i].parent(), sep));
                !ty.is_empty() && ty.iter().all(|w| words.contains(w))
            })
            .collect();
        match hinted.len() {
            1 => return vec![(hinted[0], EdgeStatus::Inferred)],
            0 => {}
            _ => return Vec::new(),
        }
        // Otherwise only a method name that is unique in the whole repository,
        // and defined where the caller can reach it.
        if cands.len() == 1
            && !COMMON.contains(&name)
            && self.reachable(&s.path, &self.syms[cands[0]].path)
        {
            return vec![(cands[0], EdgeStatus::Inferred)];
        }
        Vec::new()
    }

    /// Whether code in `from` can plausibly reach `to` without naming it:
    /// same file, a file it imports (two hops), or the same Rust crate.
    fn reachable(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        if Lang::from_path(from).map(Lang::family) == Some(Family::Rust)
            && lang::rust_module(from).0 == lang::rust_module(to).0
        {
            return true;
        }
        let Some(direct) = self.imports.get(from) else {
            return false;
        };
        direct.contains(to)
            || direct
                .iter()
                .any(|f| self.imports.get(f).is_some_and(|s| s.contains(to)))
    }

    /// A method of a class, struct, trait or interface, as opposed to a
    /// method of an object literal inside some function.
    fn parent_is_type(&self, t: &Sym) -> bool {
        if t.lang.family() == Family::Rust {
            return true; // impl blocks may be for types defined elsewhere
        }
        self.by_qname.get(t.parent()).is_some_and(|v| {
            v.iter().any(|&i| {
                matches!(
                    self.syms[i].kind.as_str(),
                    "class" | "interface" | "struct" | "enum" | "trait"
                )
            })
        })
    }

    fn is_top_level(&self, t: &Sym) -> bool {
        match t.lang.family() {
            Family::Js => t.parent().is_empty(),
            Family::Python => t.parent() == lang::python_module(&t.path).join("."),
            Family::Rust => t.parent() == lang::rust_module(&t.path).1.join("::"),
        }
    }

    fn in_file(&self, file: &str, pred: impl Fn(&Sym) -> bool) -> Vec<usize> {
        self.by_path
            .get(file)
            .map(|v| v.iter().copied().filter(|&i| pred(&self.syms[i])).collect())
            .unwrap_or_default()
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.by_id.get(id).copied()
    }
}

fn last_seg<'s>(q: &'s str, sep: &str) -> &'s str {
    q.rsplit(sep).next().unwrap_or(q)
}

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

/// Collapse `.` and `..` in a forward-slash path.
fn normalize(p: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for seg in p.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            s => out.push(s),
        }
    }
    out.join("/")
}

/// Rust crate names in the repository, from each crate's Cargo.toml.
fn rust_crates(root: &Path, files: &[(String, Lang)]) -> HashMap<String, String> {
    let mut dirs: HashSet<String> = HashSet::new();
    for (p, l) in files {
        if *l == Lang::Rust {
            dirs.insert(lang::rust_module(p).0);
        }
    }
    let mut out = HashMap::new();
    for dir in dirs {
        let manifest = root.join(&dir).join("Cargo.toml");
        let name = std::fs::read_to_string(&manifest)
            .ok()
            .and_then(|s| toml::from_str::<toml::Table>(&s).ok())
            .and_then(|t| t.get("package")?.get("name")?.as_str().map(str::to_string))
            .or_else(|| {
                dir.rsplit('/')
                    .next()
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            });
        if let Some(n) = name {
            out.insert(n.replace('-', "_"), dir);
        }
    }
    out
}

/// Edge counts produced by one resolution pass.
#[derive(Debug, Default, Clone, Copy, serde::Serialize)]
pub struct EdgeCounts {
    pub calls: usize,
    pub calls_inferred: usize,
    pub imports: usize,
    pub tests: usize,
}

/// Rebuild every `calls`, `imports` and `tests` edge from the stored refs.
pub fn resolve_all(conn: &Connection, root: &Path) -> Result<EdgeCounts> {
    conn.execute(
        "DELETE FROM edges WHERE rel IN ('calls', 'imports', 'tests')",
        [],
    )?;

    let files: Vec<(String, Lang)> = {
        let mut stmt = conn.prepare("SELECT path, lang FROM files")?;
        let rows = stmt.query_map([], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
        })?;
        rows.filter_map(|r| r.ok())
            .filter_map(|(p, l)| Some((p, Lang::parse_name(l.as_deref()?)?)))
            .collect()
    };
    let syms: Vec<Sym> = {
        let mut stmt =
            conn.prepare("SELECT id, qname, title, kind, path, lang, type FROM nodes WHERE type IN ('symbol', 'test')")?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })?;
        rows.filter_map(|r| r.ok())
            .filter_map(|(id, qname, name, kind, path, l, t)| {
                Some(Sym {
                    id,
                    qname,
                    name,
                    kind,
                    path,
                    lang: Lang::parse_name(&l)?,
                    is_test: t == "test",
                })
            })
            .collect()
    };
    let refs: Vec<RawRef> = {
        let mut stmt = conn.prepare("SELECT src, path, kind, name, qualifier, scope FROM refs")?;
        let rows = stmt.query_map([], |r| {
            Ok(RawRef {
                src: r.get(0)?,
                path: r.get(1)?,
                kind: r.get(2)?,
                name: r.get(3)?,
                qualifier: r.get(4)?,
                scope: r.get(5)?,
            })
        })?;
        rows.collect::<rusqlite::Result<_>>()?
    };

    let crates = rust_crates(root, &files);
    let mut res = Resolver::new(syms, &files, crates);
    let mut counts = EdgeCounts::default();
    let mut insert = conn
        .prepare("INSERT OR IGNORE INTO edges (src, rel, dst, status) VALUES (?1, ?2, ?3, ?4)")?;

    let mut import_edges: HashSet<(String, String)> = HashSet::new();
    for r in refs.iter().filter(|r| r.kind == "import") {
        if let Some(target) =
            res.add_import(&r.path, &r.name, r.qualifier.as_deref(), r.scope.as_deref())
        {
            if target != r.path {
                import_edges.insert((r.path.clone(), target));
            }
        }
    }
    for (a, b) in &import_edges {
        insert.execute(params![
            file_id(a),
            "imports",
            file_id(b),
            EdgeStatus::Resolved.as_str()
        ])?;
    }
    counts.imports = import_edges.len();

    for r in refs.iter().filter(|r| r.kind != "import") {
        let Some(src) = res.index_of(&r.src) else {
            continue;
        };
        for (dst, status) in res.resolve(src, r) {
            let (s, d) = (res.sym(src), res.sym(dst));
            if insert.execute(params![s.id, "calls", d.id, status.as_str()])? > 0 {
                counts.calls += 1;
                if status == EdgeStatus::Inferred {
                    counts.calls_inferred += 1;
                }
            }
            if s.is_test && insert.execute(params![s.id, "tests", d.id, status.as_str()])? > 0 {
                counts.tests += 1;
            }
        }
    }
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sym(path: &str, qname: &str, kind: &str) -> Sym {
        let lang = Lang::from_path(path).unwrap();
        let name = strip_suffix(qname)
            .rsplit(lang.sep())
            .next()
            .unwrap()
            .to_string();
        Sym {
            id: myrn_model::id::symbol_id(path, qname),
            qname: qname.into(),
            name,
            kind: kind.into(),
            path: path.into(),
            lang,
            is_test: kind == "test",
        }
    }

    fn call(src: &Sym, kind: &str, name: &str, q: Option<&str>, scope: Option<&str>) -> RawRef {
        RawRef {
            src: src.id.clone(),
            path: src.path.clone(),
            kind: kind.into(),
            name: name.into(),
            qualifier: q.map(str::to_string),
            scope: scope.map(str::to_string),
        }
    }

    fn targets(r: &Resolver, src: &Sym, raw: RawRef) -> Vec<(String, EdgeStatus)> {
        let i = r.index_of(&src.id).unwrap();
        r.resolve(i, &raw)
            .into_iter()
            .map(|(d, s)| (r.sym(d).qname.clone(), s))
            .collect()
    }

    fn files(syms: &[Sym]) -> Vec<(String, Lang)> {
        let mut f: Vec<(String, Lang)> = syms.iter().map(|s| (s.path.clone(), s.lang)).collect();
        f.dedup();
        f
    }

    #[test]
    fn rust_paths_imports_and_self() {
        let refresh = sym("src/auth/session.rs", "auth::session::refresh", "function");
        let lookup = sym("src/store.rs", "store::TokenStore::lookup", "method");
        let new = sym(
            "src/auth/session.rs",
            "auth::session::Session::new",
            "method",
        );
        let validate = sym(
            "src/auth/session.rs",
            "auth::session::Session::validate",
            "method",
        );
        let other_new = sym("src/other.rs", "other::Session::new", "method");
        let t = sym(
            "src/auth/session.rs",
            "auth::session::tests::refreshes",
            "test",
        );
        let all = vec![
            refresh.clone(),
            lookup.clone(),
            new.clone(),
            validate.clone(),
            other_new,
            t.clone(),
        ];
        let fs = files(&all);
        let mut r = Resolver::new(all, &fs, HashMap::new());
        assert_eq!(
            r.add_import(
                "src/auth/session.rs",
                "crate::store::TokenStore",
                Some("TokenStore"),
                Some("auth::session")
            ),
            Some("src/store.rs".to_string())
        );
        let res = |r: &Resolver, raw| targets(r, &refresh, raw);
        assert_eq!(
            res(
                &r,
                call(
                    &refresh,
                    "path",
                    "lookup",
                    Some("TokenStore"),
                    Some("auth::session")
                )
            ),
            [("store::TokenStore::lookup".into(), EdgeStatus::Resolved)]
        );
        // Session::new resolves to the same module's Session, not other::Session.
        assert_eq!(
            res(
                &r,
                call(
                    &refresh,
                    "path",
                    "new",
                    Some("Session"),
                    Some("auth::session")
                )
            ),
            [("auth::session::Session::new".into(), EdgeStatus::Resolved)]
        );
        // super:: from an inline test module.
        assert_eq!(
            targets(
                &r,
                &t,
                call(
                    &t,
                    "path",
                    "refresh",
                    Some("super"),
                    Some("auth::session::tests")
                )
            ),
            [("auth::session::refresh".into(), EdgeStatus::Resolved)]
        );
        // self.validate() inside Session::new
        assert_eq!(
            targets(
                &r,
                &new,
                call(&new, "method", "validate", Some("self"), None)
            ),
            [(
                "auth::session::Session::validate".into(),
                EdgeStatus::Resolved
            )]
        );
        // An external path does not resolve to anything.
        assert!(res(
            &r,
            call(
                &refresh,
                "path",
                "new",
                Some("std::collections::HashMap"),
                None
            )
        )
        .is_empty());
    }

    #[test]
    fn external_imports_block_guesses() {
        let caller = sym("src/a.rs", "a::run", "function");
        let output = sym("tests/util.rs", "TestCommand::output", "method");
        let parse = sym("crates/x/src/p.rs", "p::Parser::new", "method");
        let all = vec![caller.clone(), output, parse];
        let fs = files(&all);
        let mut r = Resolver::new(all, &fs, HashMap::new());
        assert!(r
            .add_import(
                "src/a.rs",
                "std::process::Command",
                Some("Command"),
                Some("a")
            )
            .is_none());
        assert!(r
            .add_import(
                "src/a.rs",
                "regex_syntax::ast::parse::Parser",
                Some("Parser"),
                Some("a")
            )
            .is_none());
        let res = |raw| targets(&r, &caller, raw);
        assert!(res(call(
            &caller,
            "method",
            "output",
            Some("Command::new(p).arg(x)"),
            None
        ))
        .is_empty());
        assert!(res(call(&caller, "path", "new", Some("Parser"), Some("a"))).is_empty());
    }

    #[test]
    fn python_imports_and_methods() {
        let refresh = sym(
            "pkg/auth/session.py",
            "pkg.auth.session.Session.refresh",
            "method",
        );
        let validate = sym(
            "pkg/auth/session.py",
            "pkg.auth.session.Session.validate",
            "method",
        );
        let find = sym(
            "pkg/auth/store.py",
            "pkg.auth.store.TokenStore.find",
            "method",
        );
        let lookup = sym("pkg/auth/store.py", "pkg.auth.store.lookup", "function");
        let all = vec![refresh.clone(), validate, find, lookup];
        let fs = files(&all);
        let mut r = Resolver::new(all, &fs, HashMap::new());
        r.add_import("pkg/auth/session.py", ".store.TokenStore", Some("TS"), None)
            .unwrap();
        r.add_import("pkg/auth/session.py", ".store.lookup", Some("lookup"), None)
            .unwrap();
        let res = |raw| targets(&r, &refresh, raw);
        assert_eq!(
            res(call(&refresh, "method", "find", Some("TS"), None)),
            [(
                "pkg.auth.store.TokenStore.find".into(),
                EdgeStatus::Resolved
            )]
        );
        assert_eq!(
            res(call(&refresh, "call", "lookup", None, None)),
            [("pkg.auth.store.lookup".into(), EdgeStatus::Resolved)]
        );
        assert_eq!(
            res(call(&refresh, "method", "validate", Some("self"), None)),
            [(
                "pkg.auth.session.Session.validate".into(),
                EdgeStatus::Resolved
            )]
        );
    }

    #[test]
    fn js_relative_imports_and_inference() {
        let refresh = sym("src/session.ts", "refresh", "function");
        let lookup = sym("src/store/index.ts", "lookup", "function");
        let save = sym("src/store/index.ts", "TokenStore.persistToken", "method");
        let store_class = sym("src/store/index.ts", "TokenStore", "class");
        let get_a = sym("src/a.ts", "A.get", "method");
        let get_b = sym("src/b.ts", "B.get", "method");
        let all = vec![refresh.clone(), lookup, save, store_class, get_a, get_b];
        let fs = files(&all);
        let mut r = Resolver::new(all, &fs, HashMap::new());
        assert_eq!(
            r.add_import("src/session.ts", "./store#lookup", Some("find"), None),
            Some("src/store/index.ts".into())
        );
        let res = |raw| targets(&r, &refresh, raw);
        assert_eq!(
            res(call(&refresh, "call", "find", None, None)),
            [("lookup".into(), EdgeStatus::Resolved)]
        );
        // Unique method name: inferred.
        assert_eq!(
            res(call(
                &refresh,
                "method",
                "persistToken",
                Some("store"),
                None
            )),
            [("TokenStore.persistToken".into(), EdgeStatus::Inferred)]
        );
        // Ambiguous common name: nothing.
        assert!(res(call(&refresh, "method", "get", Some("x"), None)).is_empty());
    }
}
