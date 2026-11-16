//! The closed relation vocabulary. Nothing outside this list is accepted:
//! letting a model invent relation names makes the graph unqueryable.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    DefinedIn,
    PartOf,
    Calls,
    Imports,
    Tests,
    Touches,
    ImplementedBy,
    About,
    RealizedIn,
    Produced,
    Encountered,
    CausedBy,
    LocatedIn,
    Fixes,
    Avoids,
    VerifiedBy,
    Supersedes,
    Contradicts,
    Related,
}

impl Relation {
    pub const ALL: [Relation; 19] = [
        Relation::DefinedIn,
        Relation::PartOf,
        Relation::Calls,
        Relation::Imports,
        Relation::Tests,
        Relation::Touches,
        Relation::ImplementedBy,
        Relation::About,
        Relation::RealizedIn,
        Relation::Produced,
        Relation::Encountered,
        Relation::CausedBy,
        Relation::LocatedIn,
        Relation::Fixes,
        Relation::Avoids,
        Relation::VerifiedBy,
        Relation::Supersedes,
        Relation::Contradicts,
        Relation::Related,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Relation::DefinedIn => "defined_in",
            Relation::PartOf => "part_of",
            Relation::Calls => "calls",
            Relation::Imports => "imports",
            Relation::Tests => "tests",
            Relation::Touches => "touches",
            Relation::ImplementedBy => "implemented_by",
            Relation::About => "about",
            Relation::RealizedIn => "realized_in",
            Relation::Produced => "produced",
            Relation::Encountered => "encountered",
            Relation::CausedBy => "caused_by",
            Relation::LocatedIn => "located_in",
            Relation::Fixes => "fixes",
            Relation::Avoids => "avoids",
            Relation::VerifiedBy => "verified_by",
            Relation::Supersedes => "supersedes",
            Relation::Contradicts => "contradicts",
            Relation::Related => "related",
        }
    }

    pub fn parse(s: &str) -> Option<Relation> {
        Self::ALL.into_iter().find(|r| r.as_str() == s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_vocabulary() {
        for r in Relation::ALL {
            assert_eq!(Relation::parse(r.as_str()), Some(r));
        }
        assert_eq!(Relation::parse("depends_on"), None);
    }
}
