//! Error signatures: the same failure, seen again on another machine or a day
//! later, must normalize to the same string.
//!
//! Keep the error type and the stable words of its message. Strip paths, line
//! and column numbers, addresses, timestamps, ids and other numbers.

use std::sync::OnceLock;

use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Signature {
    /// `TypeError`, `E0308`, `TS2345`, `panic`, `error`.
    pub kind: String,
    /// The message with volatile parts replaced.
    pub message: String,
}

impl Signature {
    /// `kind: message`, the form stored on bug notes and compared on recall.
    pub fn text(&self) -> String {
        if self.message.is_empty() {
            self.kind.clone()
        } else {
            format!("{}: {}", self.kind, self.message)
        }
    }

    /// Words of the message, for fuzzy matching.
    pub fn words(&self) -> Vec<String> {
        self.message
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| w.len() >= 2 && *w != "N" && *w != "PATH" && *w != "ID")
            .map(str::to_lowercase)
            .collect()
    }

    /// How alike two signatures are, 0 to 1. Different kinds never match.
    pub fn similarity(&self, other: &Signature) -> f64 {
        if !self.kind.eq_ignore_ascii_case(&other.kind) {
            return 0.0;
        }
        if self.message == other.message {
            return 1.0;
        }
        let a: std::collections::HashSet<String> = self.words().into_iter().collect();
        let b: std::collections::HashSet<String> = other.words().into_iter().collect();
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        a.intersection(&b).count() as f64 / a.union(&b).count() as f64
    }
}

fn re(cell: &'static OnceLock<Regex>, pat: &str) -> &'static Regex {
    cell.get_or_init(|| Regex::new(pat).expect("valid regex"))
}

macro_rules! regex {
    ($pat:expr) => {{
        static CELL: OnceLock<Regex> = OnceLock::new();
        re(&CELL, $pat)
    }};
}

/// Normalize error output into a signature. `None` when nothing looks like an error.
pub fn normalize(text: &str) -> Option<Signature> {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    if lines.is_empty() {
        return None;
    }
    let (kind, message) = pick(&lines)?;
    Some(Signature {
        kind,
        message: scrub(&message),
    })
}

/// Find the line that names the error, most specific form first.
fn pick(lines: &[&str]) -> Option<(String, String)> {
    // Rust compiler: error[E0308]: mismatched types
    let rustc = regex!(r"^error\[(E\d{4})\]:\s*(.*)$");
    // TypeScript compiler: src/a.ts(3,5): error TS2345: Argument of type ...
    let tsc = regex!(r"error (TS\d{3,5}):\s*(.*)$");
    // Rust panic: thread 'main' panicked at src/a.rs:3:5:\nmessage   (or 'msg', src/a.rs:3:5 in older Rust)
    let panic = regex!(r"panicked at (?:'(.*)', )?\S+?(?::\d+)+:?\s*(.*)$");
    // Python / JS / Java style: TypeError: message, pkg.errors.AuthError: message
    let typed = regex!(
        r"^(?:Uncaught |raise |E\s+)?((?:[A-Za-z_][\w$]*\.)*[A-Z][\w$]*(?:Error|Exception|Exit|Interrupt|Warning|Failure|Fault)):\s*(.*)$"
    );
    let bare_typed = regex!(r"^((?:[A-Za-z_][\w$]*\.)*[A-Z][\w$]*(?:Error|Exception))$");

    for l in lines {
        if let Some(c) = rustc.captures(l) {
            return Some((c[1].to_string(), c[2].to_string()));
        }
        if let Some(c) = tsc.captures(l) {
            return Some((c[1].to_string(), c[2].to_string()));
        }
    }
    for (i, l) in lines.iter().enumerate() {
        if let Some(c) = panic.captures(l) {
            let inline = c.get(1).or(c.get(2)).map(|m| m.as_str()).unwrap_or("");
            let msg = if inline.is_empty() {
                lines.get(i + 1).copied().unwrap_or("")
            } else {
                inline
            };
            return Some(("panic".into(), msg.to_string()));
        }
    }
    // Python prints the exception last, after the traceback; JS prints it first.
    let python = lines
        .iter()
        .any(|l| l.starts_with("Traceback (most recent call last)"));
    let ordered: Vec<&&str> = if python {
        lines.iter().rev().collect()
    } else {
        lines.iter().collect()
    };
    for l in &ordered {
        if let Some(c) = typed.captures(l) {
            // Keep only the class name: `pkg.errors.AuthError` -> `AuthError`.
            let kind = c[1].rsplit('.').next().unwrap_or(&c[1]).to_string();
            return Some((kind, c[2].to_string()));
        }
        if let Some(c) = bare_typed.captures(l) {
            let kind = c[1].rsplit('.').next().unwrap_or(&c[1]).to_string();
            return Some((kind, String::new()));
        }
    }
    // Generic: `error: ...`, `Error: ...`, `FAILED ...`, assertion failures.
    let generic = regex!(r"(?i)^(?:error|fatal|failed|failure|assertion failed)\b[:!]?\s*(.*)$");
    for l in lines {
        if let Some(c) = generic.captures(l) {
            if !c[1].trim().is_empty() {
                return Some(("error".into(), c[1].to_string()));
            }
        }
    }
    lines
        .iter()
        .find(|l| l.to_lowercase().contains("error") || l.contains("FAILED"))
        .map(|l| ("error".to_string(), l.to_string()))
}

/// Replace the volatile parts of a message.
fn scrub(msg: &str) -> String {
    let mut s = msg.to_string();
    let rules: [(&Regex, &str); 11] = [
        // ISO timestamps and clock times
        (
            regex!(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}(:\d{2}(\.\d+)?)?(Z|[+-]\d{2}:?\d{2})?"),
            "TIME",
        ),
        (regex!(r"\b\d{1,2}:\d{2}:\d{2}(\.\d+)?\b"), "TIME"),
        // UUIDs
        (
            regex!(r"(?i)\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b"),
            "ID",
        ),
        // Memory addresses
        (regex!(r"(?i)\b0x[0-9a-f]+\b"), "ADDR"),
        // Windows and Unix paths, with optional :line:col
        (regex!(r#"(?i)\b[a-z]:\\[^\s'"`,;)]+"#), "PATH"),
        (
            regex!(r#"(?:\.{0,2}/)?(?:[\w.@-]+/)+[\w.@-]+(?::\d+)*"#),
            "PATH",
        ),
        // file.ext:line:col left without a directory
        (
            regex!(r"\b[\w-]+\.(?:rs|py|pyi|ts|tsx|js|jsx|mjs|cjs)(?::\d+)+"),
            "PATH",
        ),
        // Long digit runs are numbers (epoch times, ids), not hashes
        (regex!(r"\b\d{7,}\b"), "N"),
        // Long hex ids and hashes
        (regex!(r"(?i)\b[0-9a-f]{7,}\b"), "ID"),
        // Numbered names: user_42, worker-3 (but keep utf8, i32, md5)
        (regex!(r"([_-])\d+\b"), "${1}N"),
        // Remaining numbers (line numbers, counts, ports, ids)
        (regex!(r"\b\d+(\.\d+)?\b"), "N"),
    ];
    for (re, with) in rules {
        s = re.replace_all(&s, with).into_owned();
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(t: &str) -> String {
        normalize(t).unwrap().text()
    }

    #[test]
    fn rust_compiler_and_panics() {
        let a = "error[E0308]: mismatched types\n --> src/auth/session.rs:42:17\n  |";
        let b = "   Compiling x v0.1.0\nerror[E0308]: mismatched types\n --> C:\\work\\x\\src\\auth\\session.rs:50:3";
        assert_eq!(sig(a), "E0308: mismatched types");
        assert_eq!(sig(a), sig(b));

        let new = "thread 'auth::refresh_expired' panicked at src/auth/session.rs:88:9:\ncalled `Result::unwrap()` on an `Err` value: Expired(1731801234)";
        let old = "thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Expired(1731809999)', src/lib.rs:3:5";
        assert_eq!(
            sig(new),
            "panic: called `Result::unwrap()` on an `Err` value: Expired(N)"
        );
        assert_eq!(sig(new), sig(old));
    }

    #[test]
    fn python_tracebacks_use_the_last_line() {
        let a = "Traceback (most recent call last):\n  File \"/home/a/proj/more_itertools/recipes.py\", line 412, in unique_everseen\n    seenset.add(element)\nTypeError: unhashable type: 'list'";
        let b = "Traceback (most recent call last):\n  File \"C:\\Users\\b\\proj\\more_itertools\\recipes.py\", line 415, in unique_everseen\nTypeError: unhashable type: 'list'";
        assert_eq!(sig(a), "TypeError: unhashable type: 'list'");
        assert_eq!(sig(a), sig(b));
        assert_eq!(sig("E       KeyError: 'user_42'"), "KeyError: 'user_N'");
        assert_eq!(
            sig("myapp.errors.AuthError: token 3f9a2c1d8e expired at 2026-11-17T10:22:01Z"),
            "AuthError: token ID expired at TIME"
        );
    }

    #[test]
    fn typescript_and_javascript() {
        let a = "src/chunk.ts(12,5): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.";
        let b = "packages/x/src/chunk.ts(40,9): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.";
        assert_eq!(sig(a), sig(b));
        assert!(sig(a).starts_with("TS2345: Argument of type"));

        let js1 = "TypeError: Cannot read properties of undefined (reading 'length')\n    at chunk (/home/a/remeda/src/chunk.ts:14:22)\n    at Object.<anonymous> (/home/a/remeda/src/chunk.test.ts:5:3)";
        let js2 = "Uncaught TypeError: Cannot read properties of undefined (reading 'length')\n    at chunk (file:///C:/b/remeda/dist/index.js:200:11)";
        assert_eq!(
            sig(js1),
            "TypeError: Cannot read properties of undefined (reading 'length')"
        );
        assert_eq!(sig(js1), sig(js2));
    }

    #[test]
    fn generic_and_addresses() {
        assert_eq!(
            sig("Error: connect ECONNREFUSED 127.0.0.1:5432"),
            "error: connect ECONNREFUSED N.N:N"
        );
        assert_eq!(
            sig("fatal: object at 0x7ffd5e8c2a10 was freed"),
            "error: object at ADDR was freed"
        );
        assert!(normalize("all good\n3 passed").is_none());
    }

    #[test]
    fn similarity() {
        let a = normalize("TypeError: unhashable type: 'list'").unwrap();
        let b = normalize("TypeError: unhashable type: 'dict'").unwrap();
        let c = normalize("ValueError: unhashable type: 'list'").unwrap();
        assert!(a.similarity(&b) >= 0.5);
        assert_eq!(a.similarity(&c), 0.0);
        assert_eq!(a.similarity(&a), 1.0);
    }
}
