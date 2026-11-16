/// Holds issued tokens.
pub struct TokenStore {
    tokens: Vec<String>,
}

impl TokenStore {
    pub fn new() -> TokenStore {
        TokenStore { tokens: Vec::new() }
    }

    /// Find a token.
    pub fn lookup(&self, token: &str) -> Option<String> {
        self.tokens.iter().find(|t| t.as_str() == token).cloned()
    }

    pub fn insert(&mut self, token: &str) {
        self.tokens.push(token.to_string());
    }
}

pub fn now() -> u64 {
    42
}
