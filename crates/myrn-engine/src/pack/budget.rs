//! Token budgets. Counting is approximate (about four characters per token),
//! which is close enough to keep packs inside their limits.

pub fn tokens(s: &str) -> usize {
    s.chars().count().div_ceil(4)
}

/// Accumulates text up to a token budget.
pub struct Budget {
    limit: usize,
    used: usize,
    pub out: String,
}

impl Budget {
    pub fn new(limit: usize) -> Budget {
        Budget {
            limit,
            used: 0,
            out: String::new(),
        }
    }

    pub fn remaining(&self) -> usize {
        self.limit.saturating_sub(self.used)
    }

    pub fn used(&self) -> usize {
        self.used
    }

    /// Append if it fits; returns whether it did.
    pub fn push(&mut self, s: &str) -> bool {
        let t = tokens(s);
        if self.used + t > self.limit {
            return false;
        }
        self.used += t;
        self.out.push_str(s);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_limit() {
        let mut b = Budget::new(3);
        assert!(b.push("12345678"));
        assert!(!b.push("12345678"));
        assert!(b.push("1234"));
        assert_eq!(b.remaining(), 0);
    }
}
