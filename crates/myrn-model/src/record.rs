//! Validity metadata.

use serde::{Deserialize, Serialize};

/// State of an earned record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Unverified,
    Verified,
    Suspect,
    Superseded,
}

impl Status {
    pub const ALL: [Status; 4] = [
        Status::Unverified,
        Status::Verified,
        Status::Suspect,
        Status::Superseded,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Status::Unverified => "unverified",
            Status::Verified => "verified",
            Status::Suspect => "suspect",
            Status::Superseded => "superseded",
        }
    }

    pub fn parse(s: &str) -> Option<Status> {
        Self::ALL.into_iter().find(|t| t.as_str() == s)
    }
}

/// How a derived edge was established. Tree-sitter alone cannot resolve every
/// call, so guesses are marked `Inferred` rather than presented as fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeStatus {
    Resolved,
    Inferred,
}

impl EdgeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            EdgeStatus::Resolved => "resolved",
            EdgeStatus::Inferred => "inferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Shared,
}

impl Visibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Visibility::Private => "private",
            Visibility::Shared => "shared",
        }
    }
}
