use crate::store::{self, TokenStore};

pub struct Session {
    pub token: String,
    pub issued: u64,
}

impl Session {
    pub fn new(token: String) -> Session {
        Session { token, issued: store::now() }
    }

    pub fn is_fresh(&self) -> bool {
        self.issued + 10 > store::now()
    }
}

/// Issues a new session when the token is known.
pub fn refresh(store: &TokenStore, token: &str) -> Option<Session> {
    let found = store.lookup(token)?;
    let s = Session::new(found);
    if s.is_fresh() {
        Some(s)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_unknown_token() {
        let store = TokenStore::new();
        assert!(refresh(&store, "x").is_none());
    }
}
