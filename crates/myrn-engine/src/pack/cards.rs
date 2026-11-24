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
    /// Learned records about this symbol: facts, decisions, root causes, bugs.
    pub known: Vec<Known>,
    /// The summary was written for an older body of this symbol.
    pub summary_outdated: bool,
}

/// A learned record shown on a card.
#[derive(Debug, Clone, Serialize)]
pub struct Known {
    pub id: String,
    #[serde(rename = "type")]
    pub ntype: String,
    pub status: String,
    pub title: String,
    /// Shared or private, who wrote it and when.
    pub from: String,
}

impl Known {
    fn line(&self) -> String {
        format!(
            "{} ({}; {}): {}",
            self.id, self.status, self.from, self.title
        )
    }
}

fn summary_text<'a>(n: &'a Node, outdated: bool) -> Option<std::borrow::Cow<'a, str>> {
    match (&n.summary, outdated) {
        (Some(s), true) => Some(format!("(possibly outdated: the code changed since) {s}").into()),
        (Some(s), false) => Some(s.as_str().into()),
        (None, _) => n.doc.as_deref().map(Into::into),
    }
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
        let summary = summary_text(n, self.summary_outdated);
        row(
            &mut out,
            "summary",
            summary.as_deref().unwrap_or("none yet"),
        );
        if !self.members.is_empty() {
            list(&mut out, "members", &self.members, limit);
        }
        list(&mut out, "calls", &self.calls, limit);
        list(&mut out, "called by", &self.called_by, limit);
        list(&mut out, "tests", &self.tests, limit);
        if !self.commits.is_empty() {
            list(&mut out, "changed in", &self.commits, 3);
        }
        if self.known.is_empty() {
            row(&mut out, "known", "no learned records yet");
        }
        for (i, k) in self.known.iter().take(limit).enumerate() {
            row(&mut out, if i == 0 { "known" } else { "" }, &k.line());
        }
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

/// `9 in tests/test_recipes.py (7), tests/test_more.py (2)`
pub fn tests_summary(tests: &[Link]) -> String {
    if tests.is_empty() {
        return "none found".into();
    }
    let mut by_file: Vec<(String, usize)> = Vec::new();
    for t in tests {
        let file = t.location.split(':').next().unwrap_or("").to_string();
        match by_file.iter_mut().find(|(f, _)| *f == file) {
            Some((_, n)) => *n += 1,
            None => by_file.push((file, 1)),
        }
    }
    by_file.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let files: Vec<String> = by_file
        .iter()
        .take(3)
        .map(|(f, n)| format!("{f} ({n})"))
        .collect();
    let more = if by_file.len() > 3 {
        format!(", {} more files", by_file.len() - 3)
    } else {
        String::new()
    };
    format!("{} in {}{more}", tests.len(), files.join(", "))
}

impl SymbolCard {
    /// The card injected into a prompt: short relation lists, tests as one
    /// line, commits only when asked for, and the code when it is short.
    pub fn render_compact(
        &self,
        code: Option<&str>,
        with_commits: bool,
        with_tests: bool,
        edit_hint: bool,
    ) -> String {
        let n = &self.node;
        let mut out = String::new();
        let _ = writeln!(
            out,
            "{:<LABEL$}{}    {}",
            kind_of(n),
            n.display_name(),
            n.location()
        );
        if code.is_none() {
            if let Some(s) = n.signature.as_deref().filter(|s| !s.is_empty()) {
                row(&mut out, "signature", s);
            }
        }
        if let Some(s) = summary_text(n, self.summary_outdated) {
            // The summary is what lets the agent act without opening the file: keep it whole.
            let first = s.lines().next().unwrap_or(&s);
            row(
                &mut out,
                "summary",
                &crate::capture::episode::clip(first, 400),
            );
        }
        if !self.calls.is_empty() {
            list(&mut out, "calls", &self.calls, 3);
        }
        if !self.called_by.is_empty() {
            list(&mut out, "called by", &self.called_by, 3);
        }
        if with_tests {
            row(&mut out, "tests", &tests_summary(&self.tests));
        }
        for (i, k) in self.known.iter().take(2).enumerate() {
            row(&mut out, if i == 0 { "known" } else { "" }, &k.line());
        }
        if with_commits && !self.commits.is_empty() {
            list(&mut out, "changed in", &self.commits, 2);
        }
        if let Some(code) = code {
            let lang = n.lang.as_deref().unwrap_or("");
            let _ = writeln!(out, "```{lang}\n{}\n```", code.trim_end());
            // Say plainly that this is all of it, so the agent need not open the
            // file to check; and, for a change, the exact range an editor must
            // read first (agents re-read whole files otherwise).
            if let (Some(a), Some(b)) = (n.start_line, n.end_line) {
                row(
                    &mut out,
                    "shown",
                    &format!("complete, lines {a}-{b}, as the file is now"),
                );
                if edit_hint {
                    if let Some(p) = n.path.as_deref() {
                        row(
                            &mut out,
                            "to edit",
                            &format!(
                                "Read {p} with offset {a} and limit {} covers exactly this code",
                                b - a + 1
                            ),
                        );
                    }
                }
            }
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
