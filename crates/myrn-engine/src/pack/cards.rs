//! Cards: what an agent reads instead of opening files.

use std::fmt::Write;

use myrn_store::graph::Node;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub id: String,
    pub name: String,
    pub location: String,
    /// `resolved` or `inferred` for call edges.
    pub status: Option<String>,
}

impl Link {
    pub fn new(n: &Node, status: Option<String>) -> Link {
        let location = if n.ntype == "file" {
            String::new()
        } else {
            n.location()
        };
        Link {
            id: n.id.clone(),
            name: n.display_name().to_string(),
            location,
            status,
        }
    }
}

/// One symbol: signature, summary, callers, callees, tests, history.
#[derive(Debug, Clone, Serialize)]
pub struct SymbolCard {
    pub node: Node,
    pub calls: Vec<Link>,
    pub called_by: Vec<Link>,
    pub members: Vec<Link>,
    pub tests: Vec<Link>,
    pub commits: Vec<Link>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileCard {
    pub node: Node,
    pub symbols: Vec<Link>,
    pub imports: Vec<Link>,
    pub imported_by: Vec<Link>,
    pub commits: Vec<Link>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImpactReport {
    pub target: Link,
    pub members: Vec<Link>,
    pub direct_callers: Vec<Link>,
    /// Callers two or more hops away, with distance.
    pub indirect_callers: Vec<(Link, usize)>,
    pub importers: Vec<Link>,
    pub direct_tests: Vec<Link>,
    pub indirect_tests: Vec<(Link, usize)>,
    pub files_affected: usize,
    pub truncated: bool,
}

const LABEL: usize = 11;

fn row(out: &mut String, label: &str, value: &str) {
    let _ = writeln!(out, "{label:<LABEL$}{value}");
}

fn list(out: &mut String, label: &str, items: &[Link], limit: usize) {
    if items.is_empty() {
        row(out, label, "none found");
        return;
    }
    let width = items
        .iter()
        .take(limit)
        .map(|l| l.name.chars().count())
        .max()
        .unwrap_or(0)
        .min(48);
    for (i, l) in items.iter().take(limit).enumerate() {
        let mark = if l.status.as_deref() == Some("inferred") {
            "~"
        } else {
            " "
        };
        let line = format!("{mark}{:<width$}  {}", l.name, l.location);
        row(out, if i == 0 { label } else { "" }, &line);
    }
    if items.len() > limit {
        row(out, "", &format!(" … {} more", items.len() - limit));
    }
}

fn kind_of(n: &Node) -> &str {
    n.kind.as_deref().unwrap_or(&n.ntype)
}

impl SymbolCard {
    pub fn render(&self, limit: usize) -> String {
        let n = &self.node;
        let mut out = String::new();
        let _ = writeln!(
            out,
            "{:<LABEL$}{}    {}",
            kind_of(n),
            n.display_name(),
            n.location()
        );
        if let Some(s) = n.signature.as_deref().filter(|s| !s.is_empty()) {
            row(&mut out, "signature", s);
        }
        let summary = n
            .summary
            .as_deref()
            .or(n.doc.as_deref())
            .unwrap_or("none yet");
        row(&mut out, "summary", summary);
        if !self.members.is_empty() {
            list(&mut out, "members", &self.members, limit);
        }
        list(&mut out, "calls", &self.calls, limit);
        list(&mut out, "called by", &self.called_by, limit);
        list(&mut out, "tests", &self.tests, limit);
        if !self.commits.is_empty() {
            list(&mut out, "changed in", &self.commits, 3);
        }
        row(&mut out, "known", "no learned records yet");
        let hash = n.body_hash.as_deref().unwrap_or("");
        row(
            &mut out,
            "status",
            &format!("current (indexed body hash {hash})"),
        );
        if self
            .calls
            .iter()
            .chain(&self.called_by)
            .chain(&self.tests)
            .any(|l| l.status.as_deref() == Some("inferred"))
        {
            row(
                &mut out,
                "",
                "~ = inferred from a name match, not resolved through scope or imports",
            );
        }
        out
    }
}

impl FileCard {
    pub fn render(&self, limit: usize) -> String {
        let n = &self.node;
        let mut out = String::new();
        let _ = writeln!(out, "{:<LABEL$}{}", "file", n.path.as_deref().unwrap_or(""));
        row(&mut out, "language", n.lang.as_deref().unwrap_or("?"));
        list(&mut out, "defines", &self.symbols, limit);
        list(&mut out, "imports", &self.imports, limit);
        list(&mut out, "imported by", &self.imported_by, limit);
        if !self.commits.is_empty() {
            list(&mut out, "changed in", &self.commits, 5);
        }
        out
    }
}

impl ImpactReport {
    pub fn render(&self, limit: usize) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "impact of {}    {}",
            self.target.name, self.target.location
        );
        if !self.members.is_empty() {
            row(
                &mut out,
                "",
                &format!("(includes {} members)", self.members.len()),
            );
        }
        list(&mut out, "callers", &self.direct_callers, limit);
        let indirect: Vec<Link> = self
            .indirect_callers
            .iter()
            .map(|(l, d)| Link {
                name: format!("{} [{d} hops]", l.name),
                ..l.clone()
            })
            .collect();
        if !indirect.is_empty() {
            list(&mut out, "indirect", &indirect, limit);
        }
        if !self.importers.is_empty() {
            list(&mut out, "importers", &self.importers, limit);
        }
        let mut tests = self.direct_tests.clone();
        tests.extend(self.indirect_tests.iter().map(|(l, d)| Link {
            name: format!("{} [via {} hops]", l.name, d),
            ..l.clone()
        }));
        list(&mut out, "tests", &tests, limit);
        row(
            &mut out,
            "summary",
            &format!(
                "{} direct callers, {} indirect, {} tests, {} files affected{}",
                self.direct_callers.len(),
                self.indirect_callers.len(),
                tests.len(),
                self.files_affected,
                if self.truncated {
                    " (walk truncated)"
                } else {
                    ""
                }
            ),
        );
        out
    }
}
