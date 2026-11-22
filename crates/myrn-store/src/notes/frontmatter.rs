//! Parse and write the typed YAML header (id, type, status, typed links, anchors, evidence). Malformed headers are flagged, never dropped.
//!
//! ```text
//! ---
//! id: bug-7f3a91
//! type: bug
//! status: verified
//! caused_by: "[[rootcause-a12b08]]"
//! fixed_in: 4be2a07
//! anchors:
//!   - auth::session::refresh@9c1e44
//! evidence:
//!   - test: tests/auth/refresh_expired.rs
//! ---
//! Refreshing an expired session returns 401 instead of a new token.
//! ```
//!
//! Relation fields use the closed relation names. `fixed_in: <commit>` is the
//! one inverse spelling accepted: it means `<commit> fixes <this note>`.

use myrn_model::{Relation, Status};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use super::layout;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(rename = "type")]
    pub ntype: String,
    pub status: String,
    pub title: Option<String>,
    pub confidence: Option<f64>,
    /// Which agent and session wrote it (`claude:<session>`, `episode:<session>`).
    pub source: Option<String>,
    /// Normalized error signature (bugs): `TypeError: unhashable type: 'list'`.
    #[serde(default)]
    pub signature: Option<String>,
    pub links: Vec<Link>,
    pub anchors: Vec<Anchor>,
    pub evidence: Vec<Evidence>,
    /// Header fields Myrn does not use (Obsidian `tags`, `aliases`, ...), kept as written.
    pub extra: Vec<(String, Value)>,
    pub body: String,
}

/// A typed link in the header. `inverse` links point at this note
/// (`fixed_in: abc` is `abc fixes <note>`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub rel: Relation,
    pub target: String,
    #[serde(default)]
    pub inverse: bool,
}

/// `qualified::name@bodyhash`: a symbol the note depends on and the body hash
/// seen when it was written.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anchor {
    pub target: String,
    pub hash: Option<String>,
}

/// `- test: tests/x.rs`, `- commit: abc123`, or a plain string (empty kind).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub kind: String,
    pub value: String,
}

/// A parsed note and everything wrong with it. Problems never stop indexing.
#[derive(Debug, Clone)]
pub struct Parsed {
    pub note: Note,
    pub problems: Vec<String>,
}

const INVERSE_FIXES: &str = "fixed_in";
/// Header fields other tools commonly add; kept without complaint.
const TOLERATED: [&str; 8] = [
    "tags",
    "aliases",
    "cssclasses",
    "created",
    "updated",
    "date",
    "visibility",
    "publish",
];

impl Note {
    /// A new note with no id yet; `notes::write` assigns one from the content.
    pub fn new(ntype: &str, body: &str) -> Note {
        Note {
            id: String::new(),
            ntype: ntype.to_string(),
            status: Status::Unverified.as_str().to_string(),
            title: None,
            confidence: None,
            source: None,
            signature: None,
            links: Vec::new(),
            anchors: Vec::new(),
            evidence: Vec::new(),
            extra: Vec::new(),
            body: body.to_string(),
        }
    }

    /// The text a summary note gives its code: the body without a leading
    /// `# heading`, up to the first `## ` section (map notes keep their links there).
    pub fn summary_text(&self) -> String {
        let mut lines = self
            .body
            .lines()
            .skip_while(|l| l.trim().is_empty())
            .peekable();
        if lines.peek().is_some_and(|l| l.starts_with("# ")) {
            lines.next();
        }
        lines
            .take_while(|l| !l.starts_with("## "))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    /// Header `title`, else the first heading or line of the body.
    pub fn display_title(&self) -> String {
        if let Some(t) = self.title.as_deref().filter(|t| !t.trim().is_empty()) {
            return t.trim().to_string();
        }
        let line = self
            .body
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .unwrap_or("")
            .trim_start_matches('#')
            .trim();
        if line.is_empty() {
            return self.id.clone();
        }
        let mut t: String = line.chars().take(80).collect();
        if line.chars().count() > 80 {
            t.push('…');
        }
        t
    }
}

/// Split `---\nheader\n---\nbody`. Returns (header, body, problem).
fn split(text: &str) -> (Option<&str>, &str, Option<String>) {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let first_end = text.find('\n').map(|i| i + 1).unwrap_or(text.len());
    if text[..first_end].trim_end() != "---" {
        return (None, text, Some("no YAML header".into()));
    }
    let mut pos = first_end;
    while pos < text.len() {
        let end = text[pos..]
            .find('\n')
            .map(|i| pos + i + 1)
            .unwrap_or(text.len());
        let line = text[pos..end].trim_end();
        if line == "---" || line == "..." {
            let body = text[end..]
                .strip_prefix("\r\n")
                .or_else(|| text[end..].strip_prefix('\n'));
            return (
                Some(&text[first_end..pos]),
                body.unwrap_or(&text[end..]),
                None,
            );
        }
        pos = end;
    }
    (
        None,
        &text[first_end..],
        Some("header has no closing `---`".into()),
    )
}

fn scalar(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// A string or a list of strings.
fn strings(v: &Value) -> Option<Vec<String>> {
    match v {
        Value::Sequence(items) => items.iter().map(scalar).collect(),
        Value::Null => Some(Vec::new()),
        other => scalar(other).map(|s| vec![s]),
    }
}

/// Parse a note file. `stem` is the file name without `.md` (the fallback id);
/// `folder` is the knowledge folder it sits in (the fallback type).
pub fn parse(text: &str, stem: &str, folder: Option<&str>) -> Parsed {
    let (header, body, split_problem) = split(text);
    let mut problems: Vec<String> = split_problem.into_iter().collect();
    let mut note = Note::new("", body);
    note.status = String::new();

    let map = match header.map(serde_yaml::from_str::<Value>) {
        None => Mapping::new(),
        Some(Ok(Value::Mapping(m))) => m,
        Some(Ok(Value::Null)) => Mapping::new(),
        Some(Ok(_)) => {
            problems.push("header is not a YAML mapping".into());
            Mapping::new()
        }
        Some(Err(e)) => {
            problems.push(format!("header is not valid YAML: {e}"));
            Mapping::new()
        }
    };

    for (k, v) in &map {
        let Some(key) = k.as_str() else {
            problems.push(format!("header key {k:?} is not a string"));
            continue;
        };
        let mut bad = |what: &str| problems.push(format!("`{key}` {what}"));
        match key {
            "id" => match scalar(v) {
                Some(s) if !s.trim().is_empty() => note.id = s.trim().to_string(),
                _ => bad("must be a string"),
            },
            "type" => match scalar(v) {
                Some(s) => note.ntype = s.trim().to_string(),
                None => bad("must be a string"),
            },
            "status" => match scalar(v) {
                Some(s) => note.status = s.trim().to_string(),
                None => bad("must be a string"),
            },
            "title" => match scalar(v) {
                Some(s) => note.title = Some(s),
                None => bad("must be a string"),
            },
            "source" => match scalar(v) {
                Some(s) => note.source = Some(s),
                None => bad("must be a string"),
            },
            "signature" => match scalar(v) {
                Some(s) => note.signature = Some(s),
                None => bad("must be a string"),
            },
            "confidence" => match v.as_f64() {
                Some(c) if (0.0..=1.0).contains(&c) => note.confidence = Some(c),
                _ => bad("must be a number from 0 to 1"),
            },
            "anchors" => match strings(v) {
                Some(list) => {
                    for a in list {
                        let a = super::wikilink::unwrap_link(&a);
                        match a.rsplit_once('@') {
                            Some((t, h)) if !t.is_empty() && !h.is_empty() => {
                                note.anchors.push(Anchor {
                                    target: t.trim().to_string(),
                                    hash: Some(h.trim().to_string()),
                                })
                            }
                            _ => {
                                problems.push(format!("anchor `{a}` has no @bodyhash"));
                                note.anchors.push(Anchor {
                                    target: a.trim_end_matches('@').to_string(),
                                    hash: None,
                                });
                            }
                        }
                    }
                }
                None => bad("must be a list of qualified::name@bodyhash"),
            },
            "evidence" => {
                let items = match v {
                    Value::Sequence(items) => items.clone(),
                    Value::Null => Vec::new(),
                    other => vec![other.clone()],
                };
                for item in items {
                    match &item {
                        Value::Mapping(m) if m.len() == 1 => {
                            let (k, v) = m.iter().next().expect("one entry");
                            match (k.as_str(), scalar(v)) {
                                (Some(k), Some(v)) => note.evidence.push(Evidence {
                                    kind: k.to_string(),
                                    value: v,
                                }),
                                _ => problems.push("evidence item is not `kind: value`".into()),
                            }
                        }
                        other => match scalar(other) {
                            Some(s) => note.evidence.push(Evidence {
                                kind: String::new(),
                                value: s,
                            }),
                            None => problems.push("evidence item is not `kind: value`".into()),
                        },
                    }
                }
            }
            _ => {
                let (rel, inverse) = if key == INVERSE_FIXES {
                    (Some(Relation::Fixes), true)
                } else {
                    (Relation::parse(key), false)
                };
                match (rel, strings(v)) {
                    (Some(rel), Some(targets)) => {
                        for t in targets {
                            let target = super::wikilink::unwrap_link(&t);
                            if !target.is_empty() {
                                note.links.push(Link {
                                    rel,
                                    target,
                                    inverse,
                                });
                            }
                        }
                    }
                    (Some(_), None) => bad("must be a link or a list of links"),
                    (None, _) => {
                        if !TOLERATED.contains(&key) {
                            problems.push(format!(
                                "unknown field `{key}` (relations must use the closed relation names)"
                            ));
                        }
                        note.extra.push((key.to_string(), v.clone()));
                    }
                }
            }
        }
    }

    if note.id.is_empty() {
        if header.is_some() {
            problems.push("no `id`; using the file name".into());
        }
        note.id = stem.to_string();
    } else if layout::safe_stem(&note.id) != stem.to_lowercase() {
        problems.push(format!(
            "file name `{stem}.md` does not match id `{}`",
            note.id
        ));
    }
    let from_folder = folder.and_then(layout::type_for_folder);
    if note.ntype.is_empty() {
        if header.is_some() {
            problems.push("no `type`".into());
        }
        note.ntype = from_folder.unwrap_or("fact").to_string();
    } else if !layout::NOTE_TYPES.contains(&note.ntype.as_str()) {
        problems.push(format!(
            "unknown type `{}`; indexed as {}",
            note.ntype,
            from_folder.unwrap_or("fact")
        ));
        note.ntype = from_folder.unwrap_or("fact").to_string();
    }
    if note.status.is_empty() {
        note.status = Status::Unverified.as_str().to_string();
    } else if Status::parse(&note.status).is_none() {
        problems.push(format!(
            "unknown status `{}`; treated as unverified",
            note.status
        ));
        note.status = Status::Unverified.as_str().to_string();
    }
    Parsed { note, problems }
}

/// Write a note as Markdown. Fields come out in a fixed order, so the same
/// note always produces the same bytes. Note links are written as `[[id]]` so
/// Obsidian shows them in its graph.
pub fn render(note: &Note, note_ids: &dyn Fn(&str) -> bool) -> String {
    let mut m = Mapping::new();
    let s = |v: &str| Value::String(v.to_string());
    m.insert(s("id"), s(&note.id));
    m.insert(s("type"), s(&note.ntype));
    m.insert(s("status"), s(&note.status));
    if let Some(t) = &note.title {
        m.insert(s("title"), s(t));
    }
    if let Some(c) = note.confidence {
        m.insert(s("confidence"), Value::from(c));
    }
    if let Some(src) = &note.source {
        m.insert(s("source"), s(src));
    }
    if let Some(sig) = &note.signature {
        m.insert(s("signature"), s(sig));
    }
    let link_value = |t: &str| {
        if note_ids(t) {
            s(&format!("[[{t}]]"))
        } else {
            s(t)
        }
    };
    let mut keys: Vec<(String, Vec<Value>)> = Vec::new();
    for l in &note.links {
        let key = if l.inverse && l.rel == Relation::Fixes {
            INVERSE_FIXES.to_string()
        } else {
            l.rel.as_str().to_string()
        };
        match keys.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(link_value(&l.target)),
            None => keys.push((key, vec![link_value(&l.target)])),
        }
    }
    for (k, mut v) in keys {
        let value = if v.len() == 1 {
            v.remove(0)
        } else {
            Value::Sequence(v)
        };
        m.insert(s(&k), value);
    }
    if !note.anchors.is_empty() {
        let list = note
            .anchors
            .iter()
            .map(|a| match &a.hash {
                Some(h) => s(&format!("{}@{h}", a.target)),
                None => s(&a.target),
            })
            .collect();
        m.insert(s("anchors"), Value::Sequence(list));
    }
    if !note.evidence.is_empty() {
        let list = note
            .evidence
            .iter()
            .map(|e| {
                if e.kind.is_empty() {
                    s(&e.value)
                } else {
                    let mut one = Mapping::new();
                    one.insert(s(&e.kind), s(&e.value));
                    Value::Mapping(one)
                }
            })
            .collect();
        m.insert(s("evidence"), Value::Sequence(list));
    }
    for (k, v) in &note.extra {
        m.insert(s(k), v.clone());
    }
    let header = serde_yaml::to_string(&Value::Mapping(m)).unwrap_or_default();
    let mut out = format!("---\n{header}---\n{}", note.body);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC_EXAMPLE: &str = "---
id: bug-7f3a91
type: bug
status: verified
caused_by: \"[[rootcause-a12b08]]\"
fixed_in: 4be2a07
anchors:
  - auth::session::refresh@9c1e44
evidence:
  - test: tests/auth/refresh_expired.rs
---
Refreshing an expired session returns 401 instead of a new token.
Seen in [[session-middleware]]; related decision: [[refresh-window-config]].
";

    #[test]
    fn parses_the_design_example() {
        let p = parse(DOC_EXAMPLE, "bug-7f3a91", Some("bugs"));
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        let n = &p.note;
        assert_eq!(
            (n.id.as_str(), n.ntype.as_str(), n.status.as_str()),
            ("bug-7f3a91", "bug", "verified")
        );
        assert_eq!(
            n.links,
            [
                Link {
                    rel: Relation::CausedBy,
                    target: "rootcause-a12b08".into(),
                    inverse: false
                },
                Link {
                    rel: Relation::Fixes,
                    target: "4be2a07".into(),
                    inverse: true
                },
            ]
        );
        assert_eq!(
            n.anchors,
            [Anchor {
                target: "auth::session::refresh".into(),
                hash: Some("9c1e44".into())
            }]
        );
        assert_eq!(
            n.evidence,
            [Evidence {
                kind: "test".into(),
                value: "tests/auth/refresh_expired.rs".into()
            }]
        );
        assert!(n.body.starts_with("Refreshing"));
        assert_eq!(
            n.display_title(),
            "Refreshing an expired session returns 401 instead of a new token."
        );
    }

    #[test]
    fn round_trips() {
        let p = parse(DOC_EXAMPLE, "bug-7f3a91", Some("bugs"));
        let is_note = |t: &str| t.starts_with("rootcause-");
        let text = render(&p.note, &is_note);
        assert!(text.contains("caused_by: '[[rootcause-a12b08]]'"), "{text}");
        assert!(text.contains("fixed_in: 4be2a07"), "{text}");
        let again = parse(&text, "bug-7f3a91", Some("bugs"));
        assert!(again.problems.is_empty(), "{:?}", again.problems);
        assert_eq!(again.note, p.note);
        assert_eq!(render(&again.note, &is_note), text);
    }

    #[test]
    fn round_trips_lists_extra_and_crlf() {
        let text = "---\r\nid: fact-1\r\ntype: fact\r\nabout:\r\n  - a::b\r\n  - '[[decision-2]]'\r\ntags: [x, y]\r\nconfidence: 0.5\r\nsource: claude:s1\r\nevidence:\r\n  - plain words\r\n---\r\nBody\r\n";
        let p = parse(text, "fact-1", Some("facts"));
        assert!(p.problems.is_empty(), "{:?}", p.problems);
        assert_eq!(p.note.links.len(), 2);
        assert_eq!(p.note.extra.len(), 1);
        assert_eq!(p.note.evidence[0].kind, "");
        let out = render(&p.note, &|t: &str| t.starts_with("decision-"));
        assert_eq!(parse(&out, "fact-1", Some("facts")).note, p.note);
    }

    #[test]
    fn malformed_headers_are_flagged_not_dropped() {
        let p = parse("Just some prose.\n", "fact-9", Some("facts"));
        assert_eq!(p.problems, ["no YAML header"]);
        assert_eq!(
            (p.note.id.as_str(), p.note.ntype.as_str()),
            ("fact-9", "fact")
        );
        assert_eq!(p.note.body, "Just some prose.\n");

        let p = parse(
            "---\nid: [unclosed\n---\nbody [[x]]\n",
            "bug-1",
            Some("bugs"),
        );
        assert!(
            p.problems[0].starts_with("header is not valid YAML"),
            "{:?}",
            p.problems
        );
        assert_eq!(
            (p.note.id.as_str(), p.note.ntype.as_str()),
            ("bug-1", "bug")
        );
        assert_eq!(p.note.body, "body [[x]]\n");

        let p = parse("---\nid: bug-2\n", "bug-2", Some("bugs"));
        assert_eq!(p.problems, ["header has no closing `---`"]);

        let p = parse(
            "---\nid: bug-3\ntype: gremlin\nstatus: maybe\ndepends_on: x\nanchors: [a::b]\n---\n",
            "other",
            Some("bugs"),
        );
        let joined = p.problems.join(" | ");
        for want in [
            "does not match id",
            "unknown type",
            "unknown status",
            "unknown field `depends_on`",
            "no @bodyhash",
        ] {
            assert!(joined.contains(want), "{want}: {joined}");
        }
        assert_eq!(p.note.ntype, "bug");
        assert_eq!(p.note.status, "unverified");
        assert_eq!(p.note.anchors[0].target, "a::b");
    }
}
