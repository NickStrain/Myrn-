//! Node types. Derived nodes are computed from the code and can be rebuilt;
//! earned nodes come from agent sessions and are stored durably.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    Derived,
    Earned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    // derived
    File,
    Symbol,
    Module,
    Commit,
    Test,
    // earned
    Feature,
    Decision,
    Fact,
    Bug,
    RootCause,
    Workflow,
    Episode,
}

impl NodeType {
    pub const ALL: [NodeType; 12] = [
        NodeType::File,
        NodeType::Symbol,
        NodeType::Module,
        NodeType::Commit,
        NodeType::Test,
        NodeType::Feature,
        NodeType::Decision,
        NodeType::Fact,
        NodeType::Bug,
        NodeType::RootCause,
        NodeType::Workflow,
        NodeType::Episode,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            NodeType::File => "file",
            NodeType::Symbol => "symbol",
            NodeType::Module => "module",
            NodeType::Commit => "commit",
            NodeType::Test => "test",
            NodeType::Feature => "feature",
            NodeType::Decision => "decision",
            NodeType::Fact => "fact",
            NodeType::Bug => "bug",
            NodeType::RootCause => "root_cause",
            NodeType::Workflow => "workflow",
            NodeType::Episode => "episode",
        }
    }

    pub fn parse(s: &str) -> Option<NodeType> {
        Self::ALL.into_iter().find(|t| t.as_str() == s)
    }

    pub fn origin(self) -> Origin {
        match self {
            NodeType::File
            | NodeType::Symbol
            | NodeType::Module
            | NodeType::Commit
            | NodeType::Test => Origin::Derived,
            _ => Origin::Earned,
        }
    }
}

impl Origin {
    pub fn as_str(self) -> &'static str {
        match self {
            Origin::Derived => "derived",
            Origin::Earned => "earned",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        for t in NodeType::ALL {
            assert_eq!(NodeType::parse(t.as_str()), Some(t));
        }
        assert_eq!(NodeType::Bug.origin(), Origin::Earned);
    }
}
