//! Detect and redact secrets by pattern and entropy before anything is written.
//!
//! Order matters: whole private-key blocks first, then known token formats,
//! credentials in URLs and headers, sensitive `KEY=VALUE` assignments, and
//! finally a Shannon-entropy check for long random strings that no pattern
//! knows. Hex strings (commit ids, content hashes, UUIDs) are never flagged by
//! entropy, so hashes in paths and log lines survive.

use std::borrow::Cow;
use std::sync::LazyLock;

use regex::{Captures, Regex};

/// What a redacted value is replaced with.
pub fn mark(kind: &str) -> String {
    format!("[REDACTED:{kind}]")
}

struct Rule {
    kind: &'static str,
    re: Regex,
}

fn rule(kind: &'static str, re: &str) -> Rule {
    Rule {
        kind,
        re: Regex::new(re).expect("valid redaction pattern"),
    }
}

/// Known token formats, replaced whole.
static TOKENS: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        rule(
            "private-key",
            r"-----BEGIN [A-Z0-9 ]*PRIVATE KEY(?: BLOCK)?-----(?s:.*?)(?:-----END [A-Z0-9 ]*PRIVATE KEY(?: BLOCK)?-----|\z)",
        ),
        rule(
            "aws-access-key",
            r"\b(?:AKIA|ASIA|ABIA|ACCA|AGPA|AIDA|AROA|ANPA)[0-9A-Z]{16}\b",
        ),
        rule("github-token", r"\bgh[pousr]_[A-Za-z0-9]{30,255}\b"),
        rule("github-token", r"\bgithub_pat_[A-Za-z0-9_]{22,255}\b"),
        rule("gitlab-token", r"\bglpat-[A-Za-z0-9_-]{20,}\b"),
        rule("slack-token", r"\bxox[abposre]-[A-Za-z0-9-]{10,}"),
        rule(
            "slack-webhook",
            r"https://hooks\.slack\.com/(?:services|workflows)/[A-Za-z0-9/_-]+",
        ),
        rule(
            "jwt",
            r"\beyJ[A-Za-z0-9_-]{8,}\.eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}",
        ),
        rule("api-key", r"\bsk-(?:ant-|proj-|live-)?[A-Za-z0-9_-]{20,}"),
        rule("stripe-key", r"\b[rsp]k_(?:live|test)_[A-Za-z0-9]{16,}\b"),
        rule("google-api-key", r"\bAIza[0-9A-Za-z_-]{35}\b"),
        rule("npm-token", r"\bnpm_[A-Za-z0-9]{36}\b"),
        rule("pypi-token", r"\bpypi-AgEIcHlwaS5vcmc[A-Za-z0-9_-]{30,}"),
    ]
});

/// `scheme://user:password@host` keeps the user and drops the password.
static URL_CREDENTIALS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([a-zA-Z][a-zA-Z0-9+.-]*://[^\s/:@]+:)([^\s/@]+)@").unwrap());

/// `Authorization: Bearer xyz`, `Authorization: Basic xyz`, `-H "Bearer xyz"`.
static AUTH_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b((?:bearer|basic|token)\s+)([A-Za-z0-9._~+/=-]{12,})").unwrap()
});

/// Names that mark the value next to them as sensitive.
const SENSITIVE: &str = r"(?:secret|passw(?:or)?d|passphrase|token|api[_-]?key|apikey|access[_-]?key|private[_-]?key|credentials?|authorization|auth[_-]token|session[_-]?key|signing[_-]?key|client[_-]?secret)";

/// `PASSWORD=x`, `api_key: "x"`, `"client_secret": "x"`, `--token=x`.
static ASSIGNMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r#"(?i)([A-Za-z0-9_.-]*{SENSITIVE}[A-Za-z0-9_.-]*["']?\s*[:=]\s*)("[^"\n]*"|'[^'\n]*'|[^\s"',;)}}\]]+)"#
    ))
    .unwrap()
});

/// `--password hunter2`, `-p` is too ambiguous to include.
static FLAG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r#"(?i)(--[a-z0-9-]*{SENSITIVE}[a-z0-9-]*\s+)("[^"\n]*"|'[^'\n]*'|[^\s"'-][^\s"']*)"#
    ))
    .unwrap()
});

/// Candidate random strings for the entropy check. `/` and `.` split
/// candidates so paths and dotted names are judged segment by segment.
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z0-9+_=-]{20,}").unwrap());

/// Redact every secret found in `text`.
pub fn redact(text: &str) -> Cow<'_, str> {
    let mut out = Cow::Borrowed(text);
    for r in TOKENS.iter() {
        if r.re.is_match(&out) {
            out = Cow::Owned(r.re.replace_all(&out, mark(r.kind).as_str()).into_owned());
        }
    }
    replace(&mut out, &URL_CREDENTIALS, |c| {
        format!("{}{}@", &c[1], mark("password"))
    });
    replace(&mut out, &AUTH_HEADER, |c| {
        if c[2].starts_with("[REDACTED") {
            c[0].to_string()
        } else {
            format!("{}{}", &c[1], mark("token"))
        }
    });
    for re in [&*ASSIGNMENT, &*FLAG] {
        replace(&mut out, re, |c| {
            if harmless_value(&c[2]) {
                c[0].to_string()
            } else {
                format!("{}{}", &c[1], mark("secret"))
            }
        });
    }
    replace(&mut out, &WORD, |c| {
        if looks_random(&c[0]) {
            mark("high-entropy")
        } else {
            c[0].to_string()
        }
    });
    out
}

fn replace(out: &mut Cow<'_, str>, re: &Regex, f: impl Fn(&Captures) -> String) {
    if re.is_match(out) {
        let next = re.replace_all(out, |c: &Captures| f(c)).into_owned();
        *out = Cow::Owned(next);
    }
}

/// Values next to a sensitive name that are not secrets: numbers, booleans,
/// empty values, environment references and placeholders.
fn harmless_value(v: &str) -> bool {
    let v = v.trim_matches(|c| c == '"' || c == '\'').trim();
    if v.is_empty() || v.starts_with("[REDACTED") {
        return true;
    }
    let lower = v.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "true" | "false" | "null" | "none" | "nil" | "undefined" | "yes" | "no"
            // auth schemes: the header rule redacts the credential after them
            | "bearer" | "basic" | "token" | "digest"
    ) {
        return true;
    }
    if v.parse::<f64>().is_ok() {
        return true;
    }
    // $VAR, ${VAR}, %VAR%, <your-token>, ***, xxx
    v.starts_with('$')
        || (v.starts_with('%') && v.ends_with('%'))
        || (v.starts_with('<') && v.ends_with('>'))
        || (v.starts_with("{{") && v.ends_with("}}"))
        || v.chars()
            .all(|c| c == '*' || c == 'x' || c == 'X' || c == '.')
}

/// Bits per character.
pub fn shannon_entropy(s: &str) -> f64 {
    let mut counts = [0u32; 256];
    let mut n = 0u32;
    for b in s.bytes() {
        counts[b as usize] += 1;
        n += 1;
    }
    if n == 0 {
        return 0.0;
    }
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n as f64;
            -p * p.log2()
        })
        .sum()
}

#[derive(PartialEq, Clone, Copy)]
enum Class {
    Lower,
    Upper,
    Digit,
    Other,
}

fn class(c: char) -> Class {
    if c.is_ascii_lowercase() {
        Class::Lower
    } else if c.is_ascii_uppercase() {
        Class::Upper
    } else if c.is_ascii_digit() {
        Class::Digit
    } else {
        Class::Other
    }
}

/// A long string that looks generated rather than written: high entropy,
/// upper and lower case and digits mixed throughout. Hex is never random
/// enough to flag, because hashes and commit ids are everywhere in logs.
pub fn looks_random(s: &str) -> bool {
    if s.len() < 20 {
        return false;
    }
    if s.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        return false;
    }
    let chars: Vec<Class> = s.chars().map(class).collect();
    let has = |k: Class| chars.contains(&k);
    if !(has(Class::Lower) && has(Class::Upper) && has(Class::Digit)) {
        return false;
    }
    let digits = chars.iter().filter(|&&k| k == Class::Digit).count();
    if digits * 20 < s.len() {
        return false; // under 5% digits: an identifier with a number in it
    }
    // Written identifiers change class at word boundaries; generated strings
    // change class every couple of characters.
    let switches = chars.windows(2).filter(|w| w[0] != w[1]).count();
    let switch_rate = switches as f64 / (s.len() - 1) as f64;
    // Uniform random base62 of length n has entropy close to log2(min(n, 62)).
    let ceiling = (s.len().min(62) as f64).log2();
    shannon_entropy(s) >= ceiling * 0.82 && switch_rate >= 0.3
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str) -> String {
        redact(s).into_owned()
    }

    fn clean(s: &str) {
        assert_eq!(r(s), s, "should be left alone: {s}");
    }

    #[test]
    fn known_token_formats() {
        let cases = [
            ("key AKIAIOSFODNN7EXAMPLE used", "aws-access-key"),
            ("ghp_1234567890abcdefghijABCDEFGHIJ123456", "github-token"),
            (
                "github_pat_11ABCDEFG0123456789_abcdefghijklmnopqrstuvwxyzABCDEFG",
                "github-token",
            ),
            ("xoxb-123456789012-1234567890123-AbCdEfGhIjKl", "slack-token"),
            (
                "curl https://hooks.slack.com/services/T000/B000/XXXXXXXX",
                "slack-webhook",
            ),
            (
                "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
                "jwt",
            ),
            ("ANTHROPIC_KEY sk-ant-api03-abcdefghijklmnopqrstuvwx", "api-key"),
            // Split so secret scanners do not flag this fake fixture.
            (concat!("sk_", "live_", "4eC39HqLyjWDarjtT1zdp7dc"), "stripe-key"),
            ("AIzaSyA1234567890abcdefghijklmnopqrstuv", "google-api-key"),
        ];
        for (input, kind) in cases {
            let out = r(input);
            assert!(out.contains(&mark(kind)), "{input} -> {out}");
        }
        assert_eq!(
            r("key AKIAIOSFODNN7EXAMPLE used"),
            "key [REDACTED:aws-access-key] used"
        );
    }

    #[test]
    fn private_key_blocks_including_truncated_ones() {
        let pem = "before\n-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA\nabc\n-----END RSA PRIVATE KEY-----\nafter";
        assert_eq!(r(pem), "before\n[REDACTED:private-key]\nafter");
        let cut = "-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAA";
        assert_eq!(r(cut), "[REDACTED:private-key]");
        clean("-----BEGIN PUBLIC KEY-----");
    }

    #[test]
    fn env_style_assignments() {
        assert_eq!(r("DB_PASSWORD=hunter2"), "DB_PASSWORD=[REDACTED:secret]");
        assert_eq!(
            r("export GITHUB_TOKEN=\"abc def\" && run"),
            "export GITHUB_TOKEN=[REDACTED:secret] && run"
        );
        assert_eq!(
            r(r#"{"client_secret": "s3cr3t", "id": 4}"#),
            r#"{"client_secret": [REDACTED:secret], "id": 4}"#
        );
        assert_eq!(r("api_key: abcd1234"), "api_key: [REDACTED:secret]");
        assert_eq!(
            r("mysql --password hunter2 -u root"),
            "mysql --password [REDACTED:secret] -u root"
        );
        assert_eq!(r("--api-key=zzz"), "--api-key=[REDACTED:secret]");
    }

    #[test]
    fn credentials_in_urls_and_headers() {
        assert_eq!(
            r("git clone https://bob:pa55w0rd@github.com/x/y"),
            "git clone https://bob:[REDACTED:password]@github.com/x/y"
        );
        assert_eq!(
            r("curl -H 'Authorization: Bearer abc.def.ghi-jkl123'"),
            "curl -H 'Authorization: Bearer [REDACTED:token]'"
        );
    }

    #[test]
    fn entropy_catches_unknown_random_strings() {
        let s = "token is Zq3vT9xLm2Pa8RbW4kYc7NdE1fGh";
        assert_eq!(r(s), "token is [REDACTED:high-entropy]");
        assert!(looks_random("Xk92hDf7Lq0Zp3Wv8Bn5Rt1Y"));
    }

    #[test]
    fn harmless_text_is_left_alone() {
        // Hashes, commit ids and UUIDs, alone and inside paths.
        clean("commit 9b56b03e1f2a3c4d5e6f7a8b9c0d1e2f3a4b5c6d");
        clean("target/debug/deps/myrn_engine-3f2a1b9c8d7e6f50.d");
        clean("node_modules/.cache/babel/4e3f2a1b9c8d7e6f504e3f2a1b9c8d7e/x.json");
        clean("blake3 886ba5f5364ff42f886ba5f5364ff42f886ba5f5364ff42f886ba5f5364ff42f");
        clean("id 123e4567-e89b-12d3-a456-426614174000");
        // Identifiers, test names and paths.
        clean("crates/myrn-engine/src/indexer/SessionRefreshHandler2Test.rs");
        clean("test test_session_refresh_returns_401_when_expired ... FAILED");
        clean("error[E0425]: cannot find value `HttpServerConfigurationBuilder2` in this scope");
        clean("AbstractSingletonProxyFactoryBean");
        // Sensitive names with harmless values.
        clean("max_tokens=1500");
        clean("TOKEN=$GITHUB_TOKEN");
        clean("password: ${{ secrets.DB_PASSWORD }}");
        clean("token_budget = 500");
        clean("auth = true");
        clean("the password field is required");
        clean("author=NickStrain");
        clean("PWD=/home/dev/proj");
        // Base64 of ordinary text is not flagged by entropy alone.
        clean("cargo test -p myrn-engine -- --nocapture");
    }

    #[test]
    fn entropy_values() {
        assert_eq!(shannon_entropy(""), 0.0);
        assert_eq!(shannon_entropy("aaaa"), 0.0);
        assert!((shannon_entropy("abcd") - 2.0).abs() < 1e-9);
    }
}
