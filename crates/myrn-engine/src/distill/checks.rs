//! Reject records without existing evidence or with types outside the closed vocabulary.
//!
//! References in a record (`about`, `located_in`, evidence, relation targets)
//! are resolved against the code graph and the notes here. Every failure comes
//! back as a sentence the agent can act on.

use std::collections::BTreeMap;

use anyhow::Result;
use myrn_model::id::{symbol_id, test_id};
use myrn_store::graph::{self, Node};
use myrn_store::notes::{Anchor, Evidence};

use super::{AGENT_RELATIONS, EVIDENCE_KINDS, RECORD_TYPES};
use crate::api::{Engine, Lookup};

/// Body hash digits kept in an anchor.
pub const ANCHOR_HEX: usize = 12;

/// A resolved reference.
#[derive(Debug, Clone)]
pub struct Ref {
    /// Node id (`sym:...`, `file:...`, `commit:...`, or a note id).
    pub id: String,
    pub ntype: String,
    /// How the note header names it: `qname`, `path#qname`, a path, a short sha or a note id.
    pub header: String,
    /// For symbols: the anchor capturing the current body hash.
    pub anchor: Option<Anchor>,
}

pub fn check_type(kind: &str) -> Option<String> {
    (!RECORD_TYPES.contains(&kind)).then(|| {
        format!(
            "`type` must be one of {}; got `{kind}`",
            RECORD_TYPES.join(", ")
        )
    })
}

pub fn check_relation(rel: &str) -> Option<String> {
    (!AGENT_RELATIONS.contains(&rel)).then(|| {
        format!(
            "relation `{rel}` is not allowed; use one of {}",
            AGENT_RELATIONS.join(", ")
        )
    })
}

/// The test step of a shell command line, without `cd`, pipes, redirections
/// or leading env assignments.
pub fn test_step(cmd: &str) -> Option<String> {
    cmd.split(['&', ';', '|', '\n'])
        .map(str::trim)
        .find(|seg| crate::capture::outcome::is_test_command(seg))
        .map(|seg| {
            seg.split_whitespace()
                .skip_while(|w| w.contains('=') && !w.starts_with('-'))
                .take_while(|w| !w.contains('>') && !w.starts_with('<'))
                .collect::<Vec<_>>()
                .join(" ")
        })
}

fn looks_like_sha(t: &str) -> bool {
    (7..=40).contains(&t.len()) && t.chars().all(|c| c.is_ascii_hexdigit())
}

impl Engine {
    fn commit_ref(&self, sha: &str) -> Result<Option<Ref>> {
        let like = format!("commit:{}%", sha.to_lowercase());
        let hits = graph::nodes_where(
            &self.db.conn,
            "type = 'commit' AND id LIKE ?1 LIMIT 2",
            &[&like],
        )?;
        Ok((hits.len() == 1).then(|| {
            let full = hits[0].id.trim_start_matches("commit:").to_string();
            Ref {
                id: hits[0].id.clone(),
                ntype: "commit".into(),
                header: full[..12.min(full.len())].to_string(),
                anchor: None,
            }
        }))
    }

    fn note_ref(&self, name: &str) -> Result<Option<Ref>> {
        let row = self.db.conn.query_row(
            "SELECT id, type FROM notes WHERE lower(id) = lower(?1) LIMIT 1",
            [name],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        );
        Ok(row.ok().map(|(id, ntype)| Ref {
            header: id.clone(),
            id,
            ntype,
            anchor: None,
        }))
    }

    fn code_ref(&self, n: Node) -> Result<Ref> {
        let header = match n.ntype.as_str() {
            "symbol" | "test" => {
                let qname = n.qname.clone().unwrap_or_default();
                let same: i64 = self.db.conn.query_row(
                    "SELECT COUNT(*) FROM nodes WHERE qname = ?1 AND type IN ('symbol', 'test')",
                    [&qname],
                    |r| r.get(0),
                )?;
                if same == 1 {
                    qname
                } else {
                    format!("{}#{qname}", n.path.as_deref().unwrap_or(""))
                }
            }
            _ => n.path.clone().unwrap_or_else(|| n.id.clone()),
        };
        let anchor = matches!(n.ntype.as_str(), "symbol" | "test")
            .then(|| {
                n.body_hash.as_ref().map(|h| Anchor {
                    target: header.clone(),
                    hash: Some(h[..ANCHOR_HEX.min(h.len())].to_string()),
                })
            })
            .flatten();
        Ok(Ref {
            id: n.id,
            ntype: n.ntype,
            header,
            anchor,
        })
    }

    /// Resolve a name from a record. `notes` allows record ids as targets.
    pub(crate) fn resolve_ref(
        &self,
        name: &str,
        notes: bool,
    ) -> Result<std::result::Result<Ref, String>> {
        let name = name.trim().trim_matches('`');
        if name.is_empty() {
            return Ok(Err("empty reference".into()));
        }
        if notes {
            if let Some(r) = self.note_ref(name)? {
                return Ok(Ok(r));
            }
        }
        if looks_like_sha(name) {
            if let Some(r) = self.commit_ref(name)? {
                return Ok(Ok(r));
            }
        }
        if let Some((path, qname)) = name.split_once('#') {
            for id in [symbol_id(path, qname), test_id(path, qname)] {
                if let Some(n) = graph::node(&self.db.conn, &id)? {
                    return Ok(Ok(self.code_ref(n)?));
                }
            }
        }
        Ok(match self.pick(name)? {
            Lookup::Found(n) => Ok(self.code_ref(n)?),
            Lookup::Ambiguous { candidates } => {
                let names: Vec<String> = candidates
                    .iter()
                    .take(5)
                    .map(|c| format!("{} ({})", c.name, c.location))
                    .collect();
                Err(format!(
                    "`{name}` matches several symbols: {}; use a qualified name or path#name",
                    names.join(", ")
                ))
            }
            Lookup::NotFound { .. } => Err(format!(
                "`{name}` matches no symbol, file, commit{} in this repository",
                if notes { " or record" } else { "" }
            )),
        })
    }

    /// The test command recorded in a recent session that `cmd` cites, as the
    /// bare test step: `cd /x && cargo test -p a 2>&1 | grep ok` -> `cargo test -p a`.
    fn recorded_test_run(&self, cmd: &str) -> Result<Option<String>> {
        let Some(want) = test_step(cmd) else {
            return Ok(None);
        };
        for (_, path) in myrn_store::episodes::list(&self.layout.episodes_dir())?
            .into_iter()
            .take(20)
        {
            for e in myrn_store::episodes::read(&path)? {
                if e.test.is_none() {
                    continue;
                }
                if let Some(have) = e.command.as_deref().and_then(test_step) {
                    if have.contains(&want) || want.contains(&have) {
                        return Ok(Some(have));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Check evidence items. Returns the valid ones (as note evidence), the
    /// code they point at, and a problem for each invalid one.
    pub(crate) fn check_evidence(
        &self,
        items: &[BTreeMap<String, String>],
    ) -> Result<(Vec<Evidence>, Vec<Ref>, Vec<String>)> {
        let mut ok = Vec::new();
        let mut refs = Vec::new();
        let mut problems = Vec::new();
        for item in items {
            let Some((kind, value)) = item.iter().next().filter(|_| item.len() == 1) else {
                problems.push(format!(
                    "each evidence item is one `kind: value` pair with kind in {}",
                    EVIDENCE_KINDS.join(", ")
                ));
                continue;
            };
            let value = value.trim();
            match kind.as_str() {
                "symbol" | "test" => match self.resolve_ref(value, false)? {
                    Ok(r) if kind == "test" && !matches!(r.ntype.as_str(), "test" | "file") => {
                        problems.push(format!(
                            "evidence test `{value}` is not a test; cite a test function or test file, \
                             or use test_run for a command you ran"
                        ));
                    }
                    Ok(r) if r.ntype == "commit" => problems.push(format!(
                        "evidence `{value}` is a commit; cite it as {{\"commit\": \"{value}\"}}"
                    )),
                    Ok(r) => {
                        ok.push(Evidence {
                            kind: kind.clone(),
                            value: r.header.clone(),
                        });
                        refs.push(r);
                    }
                    Err(e) => problems.push(format!("evidence {kind}: {e}")),
                },
                "commit" => match self.commit_ref(value)? {
                    Some(r) => {
                        ok.push(Evidence {
                            kind: "commit".into(),
                            value: r.header.clone(),
                        });
                        refs.push(r);
                    }
                    None => problems.push(format!(
                        "evidence commit `{value}` is not a commit in this repository's indexed history"
                    )),
                },
                "test_run" => {
                    if let Some(run) = self.recorded_test_run(value)? {
                        ok.push(Evidence {
                            kind: "test_run".into(),
                            value: run,
                        });
                    } else {
                        problems.push(format!(
                            "evidence test_run `{value}` was not found among the test commands recorded \
                             in recent sessions; cite the command exactly as you ran it, or cite the test"
                        ));
                    }
                }
                other => problems.push(format!(
                    "unknown evidence kind `{other}`; use one of {}",
                    EVIDENCE_KINDS.join(", ")
                )),
            }
        }
        Ok((ok, refs, problems))
    }
}

#[cfg(test)]
mod tests {
    use super::test_step;

    #[test]
    fn test_steps() {
        assert_eq!(
            test_step("cd /tmp/x/ripgrep && cargo test -p globset 2>&1 | grep -E \"FAILED\"")
                .as_deref(),
            Some("cargo test -p globset")
        );
        assert_eq!(
            test_step("RUST_LOG=1 pytest tests/a.py -q").as_deref(),
            Some("pytest tests/a.py -q")
        );
        assert_eq!(test_step("ls && cat x"), None);
    }
}
