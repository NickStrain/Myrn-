//! Finding starting nodes: named symbols, file paths, `path:line` locations,
//! and full-text matches over names, qualified-name words and docs.

use std::collections::HashSet;

use anyhow::Result;
use myrn_store::graph::{self, Node};
use myrn_store::{fts, Db};

/// Look up a symbol, file or node the way a person would name it:
/// an id, `path:line`, a file path, a qualified name (`::` or `.`), or a bare name.
/// Best matches first.
pub fn find(db: &Db, query: &str) -> Result<Vec<Node>> {
    let c = &db.conn;
    let q = query.trim().trim_matches('`').trim_end_matches("()");
    if q.is_empty() {
        return Ok(Vec::new());
    }
    if let Some(n) = graph::node(c, q)? {
        return Ok(vec![n]);
    }
    // path:line
    if let Some((path, line)) = q.rsplit_once(':') {
        if let Ok(line) = line.parse::<i64>() {
            let hits = graph::nodes_where(
                c,
                "path = ?1 AND type IN ('symbol', 'test') AND start_line <= ?2 AND end_line >= ?2
                 ORDER BY (end_line - start_line) ASC LIMIT 1",
                &[&path, &line],
            )?;
            if !hits.is_empty() {
                return Ok(hits);
            }
        }
    }
    // A file path, full or trailing part.
    if q.contains('/') || myrn_index_lang(q) {
        let like = format!("%/{q}");
        let hits = graph::nodes_where(
            c,
            "type = 'file' AND (path = ?1 OR path LIKE ?2) ORDER BY length(path)",
            &[&q, &like],
        )?;
        if !hits.is_empty() {
            return Ok(hits);
        }
    }
    // Qualified or bare name.
    let segs = segments(q);
    let Some(last) = segs.last() else {
        return Ok(Vec::new());
    };
    let mut hits = graph::nodes_where(c, "title = ?1 AND type IN ('symbol', 'test')", &[last])?;
    if hits.is_empty() {
        // Case-insensitive fallback.
        hits = graph::nodes_where(
            c,
            "title = ?1 COLLATE NOCASE AND type IN ('symbol', 'test')",
            &[last],
        )?;
    }
    hits.retain(|n| ends_with_segments(n.qname.as_deref().unwrap_or(""), &segs));
    drop_shadowed_stubs(&mut hits);
    hits.sort_by_key(|n| {
        let qn = n.qname.as_deref().unwrap_or("");
        (
            segments(qn) != segs, // exact qualified name first
            n.ntype == "test",    // code before tests
            segments(qn).len(),   // shallower first
            n.path.clone(),
        )
    });
    Ok(hits)
}

/// Type stubs (`.pyi`, `.d.ts`) only declare what the implementation defines.
pub fn is_stub(path: &str) -> bool {
    path.ends_with(".pyi") || path.ends_with(".d.ts") || path.ends_with(".d.mts")
}

/// Remove stub declarations whose implementation is also in the list, so one
/// function never shows up twice.
pub fn drop_shadowed_stubs(nodes: &mut Vec<Node>) {
    let real: HashSet<String> = nodes
        .iter()
        .filter(|n| !n.path.as_deref().is_some_and(is_stub))
        .filter_map(|n| n.qname.clone())
        .collect();
    nodes.retain(|n| {
        !(n.path.as_deref().is_some_and(is_stub)
            && n.qname.as_ref().is_some_and(|q| real.contains(q)))
    });
}

fn myrn_index_lang(q: &str) -> bool {
    crate::indexer::lang::Lang::from_path(q).is_some()
}

/// Split a qualified name on `::` and `.`, dropping a `#n` collision suffix.
pub fn segments(q: &str) -> Vec<String> {
    let q = match q.rfind('#') {
        Some(i) if q[i + 1..].chars().all(|c| c.is_ascii_digit()) => &q[..i],
        _ => q,
    };
    q.replace("::", ".")
        .split('.')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn ends_with_segments(qname: &str, want: &[String]) -> bool {
    let have = segments(qname);
    have.len() >= want.len() && have[have.len() - want.len()..] == *want
}

/// A starting node for a task, with why it was chosen.
#[derive(Debug, Clone)]
pub struct Seed {
    pub id: String,
    pub score: f64,
    pub why: &'static str,
}

const STOPWORDS: &[&str] = &[
    "the", "and", "for", "with", "that", "this", "from", "into", "when", "what", "where", "which",
    "how", "why", "are", "was", "were", "has", "have", "had", "not", "but", "all", "any", "can",
    "could", "should", "would", "will", "make", "made", "add", "fix", "use", "used", "using",
    "get", "set", "new", "out", "its", "it's", "our", "you", "your", "them", "they", "there",
    "then", "than", "also", "just", "some", "more", "need", "needs", "want", "please", "code",
    "file", "files", "function", "method", "does", "doesn", "don", "isn", "work", "works",
    "working", "change", "update", "instead", "about", "after", "before", "only", "each", "other",
];

/// Words worth searching for in a task description.
pub fn task_words(task: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    fts::split_identifier(task)
        .into_iter()
        .filter(|w| {
            w.len() >= 3
                && !STOPWORDS.contains(&w.as_str())
                && !w.chars().all(|c| c.is_ascii_digit())
        })
        .filter(|w| seen.insert(w.clone()))
        .collect()
}

/// Tokens that look like code: `snake_case`, `CamelCase`, `a::b`, `a.b()`, paths, `file:line`.
fn code_tokens(task: &str) -> Vec<String> {
    task.split(|c: char| {
        c.is_whitespace()
            || matches!(
                c,
                ',' | ';' | '(' | ')' | '"' | '\'' | '`' | '[' | ']' | '{' | '}'
            )
    })
    .map(|t| t.trim_matches(|c: char| matches!(c, '.' | ':' | '!' | '?')))
    .filter(|t| t.len() >= 3)
    .filter(|t| {
        t.contains('_')
            || t.contains("::")
            || t.contains('/')
            || (t.contains('.') && !t.ends_with('.'))
            || t.chars().skip(1).any(|c| c.is_uppercase()) && t.chars().any(|c| c.is_lowercase())
    })
    .map(str::to_string)
    .collect()
}

/// Identifiers written in backticks: `` `chunk` ``, `` `chunk([1, 2], 2)` ``,
/// `` `GlobSet::new` ``. A plain word in backticks is code, even lowercase.
fn backticked(task: &str) -> Vec<String> {
    task.split('`')
        .skip(1)
        .step_by(2)
        .filter_map(|span| {
            let ident: String = span
                .trim()
                .chars()
                .take_while(|c| c.is_alphanumeric() || matches!(c, '_' | ':' | '.'))
                .collect();
            let ident = ident.trim_end_matches(['.', ':']).to_string();
            (ident.len() >= 3 && ident.chars().next().is_some_and(|c| c.is_alphabetic()))
                .then_some(ident)
        })
        .collect()
}

/// Seeds for a free-text task.
pub fn seeds(db: &Db, task: &str) -> Result<Vec<Seed>> {
    let mut out: Vec<Seed> = Vec::new();
    let mut tokens = code_tokens(task);
    for t in backticked(task) {
        if !tokens.contains(&t) {
            tokens.push(t);
        }
    }
    for tok in tokens {
        let hits = find(db, &tok)?;
        if hits.is_empty() || hits.len() > 12 {
            continue;
        }
        let exactish = hits.len() as f64;
        for n in hits.iter().take(5) {
            let why = if n.ntype == "file" {
                "named file"
            } else {
                "named in task"
            };
            out.push(Seed {
                id: n.id.clone(),
                score: 10.0 / exactish.sqrt(),
                why,
            });
        }
    }
    let words = task_words(task);
    if !words.is_empty() {
        let mut ids: Vec<String> = fts::query(&db.conn, &words, 40)?
            .into_iter()
            .map(|(id, _bm25)| id)
            .collect();
        // Code whose summary says what the task is about, even when its name does not.
        for id in fts::query_summaries(&db.conn, &words, 30)? {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        let want = words.len().min(4) as f64;
        for id in ids {
            let Some(n) = graph::node(&db.conn, &id)? else {
                continue;
            };
            if n.ntype == "module" {
                continue; // directories are described in the brief
            }
            let text = format!(
                "{} {} {} {} {}",
                fts::split_identifier(n.qname.as_deref().unwrap_or("")).join(" "),
                fts::split_identifier(n.path.as_deref().unwrap_or("")).join(" "),
                n.title.as_deref().unwrap_or("").to_lowercase(),
                n.doc.as_deref().unwrap_or("").to_lowercase(),
                n.summary.as_deref().unwrap_or("").to_lowercase()
            );
            let have: HashSet<&str> = text.split(|c: char| !c.is_alphanumeric()).collect();
            let matched = words
                .iter()
                .filter(|w| have.iter().any(|h| h.starts_with(w.as_str())))
                .count() as f64;
            let relevance = (matched / want).min(1.0);
            if relevance < 0.5 && !(words.len() == 1 && matched >= 1.0) {
                continue;
            }
            let why = if n.ntype == "commit" {
                "matching commit"
            } else {
                "matches task words"
            };
            out.push(Seed {
                id,
                score: 6.0 * relevance,
                why,
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_like_tokens() {
        let t = code_tokens("Why does `TokenStore::lookup` fail in src/auth/session.rs:42 and refresh_token()? Fix it.");
        assert!(t.contains(&"TokenStore::lookup".to_string()));
        assert!(t.contains(&"src/auth/session.rs:42".to_string()));
        assert!(t.contains(&"refresh_token".to_string()));
        assert!(!t.contains(&"Fix".to_string()));
    }

    #[test]
    fn backticked_identifiers() {
        assert_eq!(
            backticked("`chunk` drops it: `chunk([1, 2], 2)` and `*.gz`, `GlobSet::new`."),
            ["chunk", "chunk", "GlobSet::new"]
        );
    }

    #[test]
    fn words_drop_noise() {
        assert_eq!(
            task_words("Fix the refresh token bug in the session"),
            ["refresh", "token", "bug", "session"]
        );
    }

    #[test]
    fn segment_matching() {
        assert!(ends_with_segments(
            "auth::session::Session::refresh",
            &segments("Session.refresh")
        ));
        assert!(!ends_with_segments(
            "auth::session::refresh",
            &segments("Session::refresh")
        ));
    }
}
