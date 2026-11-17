//! Distillation: the agent writes records through `record`; the engine checks them.
//!
//! A record is one decision, bug (with its root cause and fix), workflow, fact,
//! or symbol summary, in a fixed JSON schema. Before anything is stored:
//! - evidence must cite an existing symbol, commit, test or recorded test run;
//! - every type and relation must come from the closed vocabularies;
//! - a record that repeats a known one updates it instead of adding a copy;
//! - a record that conflicts with a known one gets a `contradicts` edge.
//!
//! Accepted records are written as private notes (status `unverified`, anchors
//! on the current body hashes, `source: episode:<session>`), then indexed.

pub mod checks;
pub mod contradiction;
pub mod dedupe;
pub mod mechanical;
pub mod record;
pub mod signature;

use serde::{Deserialize, Deserializer, Serialize};

/// Record types the agent may write.
pub const RECORD_TYPES: [&str; 5] = ["decision", "bug", "workflow", "fact", "summary"];

/// Relations an agent may state. The rest are derived from the code or made by the engine.
pub const AGENT_RELATIONS: [&str; 11] = [
    "about",
    "implemented_by",
    "realized_in",
    "caused_by",
    "located_in",
    "fixes",
    "avoids",
    "verified_by",
    "supersedes",
    "contradicts",
    "related",
];

/// Evidence kinds: a symbol, a commit, a test (function or file), or a test
/// command that was run and recorded in a session.
pub const EVIDENCE_KINDS: [&str; 4] = ["symbol", "commit", "test", "test_run"];

/// One item for the `record` tool.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordItem {
    /// decision | bug | workflow | fact | summary
    #[serde(rename = "type")]
    pub kind: String,
    /// Short title (optional; the first line of `body` otherwise).
    #[serde(default)]
    pub title: Option<String>,
    /// The statement itself. For a summary: what the symbol does, inputs,
    /// outputs, edge cases, gotchas.
    pub body: String,
    /// Symbols, files or records this concerns.
    #[serde(default, deserialize_with = "one_or_many")]
    pub about: Vec<String>,
    /// At least one item: `{"symbol": "..."}`, `{"commit": "..."}`,
    /// `{"test": "..."}` or `{"test_run": "<command>"}`.
    #[serde(default)]
    pub evidence: Vec<std::collections::BTreeMap<String, String>>,
    /// Bug: the error text as seen (normalized into a signature).
    #[serde(default)]
    pub error: Option<String>,
    /// Bug: why it happened.
    #[serde(default)]
    pub root_cause: Option<String>,
    /// Bug: the code responsible for the root cause.
    #[serde(default, deserialize_with = "one_or_many")]
    pub located_in: Vec<String>,
    /// Bug: how it was fixed.
    #[serde(default)]
    pub fix: Option<String>,
    /// Bug: the commit that fixed it.
    #[serde(default)]
    pub fixed_in: Option<String>,
    /// Workflow: ordered steps.
    #[serde(default)]
    pub steps: Vec<String>,
    /// Workflow or fact: tests or commits showing it works.
    #[serde(default, deserialize_with = "one_or_many")]
    pub verified_by: Vec<String>,
    /// Other typed links: `{"rel": "supersedes", "target": "fact-1a2b3c"}`.
    #[serde(default)]
    pub relations: Vec<RelationItem>,
    /// Session id; defaults to the most recently recorded session.
    #[serde(default)]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationItem {
    pub rel: String,
    pub target: String,
}

/// What happened to a record.
#[derive(Debug, Clone, Default, Serialize)]
pub struct RecordOutcome {
    pub accepted: bool,
    /// Notes written or updated.
    pub ids: Vec<String>,
    /// An existing record was updated instead of adding a new one.
    pub updated: bool,
    /// Rejection reasons, or notes about what was accepted.
    pub messages: Vec<String>,
}

impl RecordOutcome {
    pub fn reject(reasons: Vec<String>) -> RecordOutcome {
        RecordOutcome {
            accepted: false,
            messages: reasons,
            ..Default::default()
        }
    }

    pub fn render(&self) -> String {
        let mut out = if self.accepted {
            format!(
                "{} {} (status unverified until an outcome backs it).\n",
                if self.updated {
                    "Updated existing record"
                } else {
                    "Recorded"
                },
                self.ids.join(", ")
            )
        } else {
            "Rejected; nothing was stored. Fix these and call record again:\n".to_string()
        };
        for m in &self.messages {
            out.push_str(&format!("- {m}\n"));
        }
        out
    }
}

fn one_or_many<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(match Option::<OneOrMany>::deserialize(d)? {
        None => Vec::new(),
        Some(OneOrMany::One(s)) => vec![s],
        Some(OneOrMany::Many(v)) => v,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_accepts_strings_or_lists_and_rejects_unknown_fields() {
        let item: RecordItem = serde_json::from_value(serde_json::json!({
            "type": "fact", "body": "x", "about": "a::b", "evidence": [{"symbol": "a::b"}]
        }))
        .unwrap();
        assert_eq!(item.about, ["a::b"]);
        let err = serde_json::from_value::<RecordItem>(serde_json::json!({
            "type": "fact", "body": "x", "depends_on": "a"
        }))
        .unwrap_err();
        assert!(
            err.to_string().contains("unknown field `depends_on`"),
            "{err}"
        );
    }
}
