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
    // Overloads in a stub get a `#n` suffix (`windowed#2`): still the same function.
    let real: HashSet<&str> = nodes
        .iter()
        .filter(|n| !n.path.as_deref().is_some_and(is_stub))
        .filter_map(|n| n.qname.as_deref().map(without_suffix))
        .collect();
    let shadowed: Vec<bool> = nodes
        .iter()
        .map(|n| {
            n.path.as_deref().is_some_and(is_stub)
                && n.qname
                    .as_deref()
                    .is_some_and(|q| real.contains(without_suffix(q)))
        })
        .collect();
    let mut i = 0;
    nodes.retain(|_| {
        i += 1;
        !shadowed[i - 1]
    });
}

/// A qualified name without its `#n` collision suffix.
fn without_suffix(q: &str) -> &str {
    match q.rfind('#') {
        Some(i) if i + 1 < q.len() && q[i + 1..].chars().all(|c| c.is_ascii_digit()) => &q[..i],
        _ => q,
    }
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
    // Bug-report boilerplate: "Find the cause and fix it. Make sure the tests still pass."
    "find", "cause", "sure", "still", "pass", "passes", "whose", "contain", "contains", "one",
];

/// Words worth searching for in a task description. Inside code literals
/// (`` `*.gz` ``, `archive.tar.gz`) two-letter words count too: there they are
/// extensions and names, the most specific words a report has.
pub fn task_words(task: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let literal: HashSet<String> = backticked_spans(task)
        .into_iter()
        .chain(code_tokens(task))
        .flat_map(|t| fts::split_identifier(&t))
        .collect();
    fts::split_identifier(task)
        .into_iter()
        .filter(|w| {
            (w.len() >= 3 || (w.len() == 2 && literal.contains(w)))
                && !STOPWORDS.contains(&w.as_str())
                && !w.chars().all(|c| c.is_ascii_digit())
        })
        .filter(|w| seen.insert(w.clone()))
        .collect()
}

/// The text inside each pair of backticks.
fn backticked_spans(task: &str) -> Vec<String> {
    task.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
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
pub(crate) fn backticked(task: &str) -> Vec<String> {
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
        let by_summary = fts::query_summaries(&db.conn, &words, 30)?;
        for id in &by_summary {
            if !ids.contains(id) {
                ids.push(id.clone());
            }
        }
        let want = words.len().min(4) as f64;
        let bug = asks_about_bug(task);
        // For a bug report, the order of the summary search counts: it weighs
        // rare words (an extension, a literal value) above common ones, which
        // is what tells the place of a bug apart from everything near it.
        let summary_rank = |id: &str| -> f64 {
            if !bug {
                return 0.0;
            }
            by_summary
                .iter()
                .take(SUMMARY_RANKED)
                .position(|s| s == id)
                .map_or(0.0, |i| SUMMARY_RANK_BONUS / (1.0 + i as f64 / 2.0))
        };
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
            let details = fts::summary_details(&db.conn, &id)?;
            let have: HashSet<&str> = text.split(|c: char| !c.is_alphanumeric()).collect();
            let more: HashSet<&str> = details.split(|c: char| !c.is_alphanumeric()).collect();
            let hit = |set: &HashSet<&str>, w: &str| set.iter().any(|h| h.starts_with(w));
            let named = words.iter().filter(|w| hit(&have, w)).count();
            // Words only the note's behaviour and terms carry: how a person
            // describes what the code does, which a bug report repeats.
            let described = words
                .iter()
                .filter(|w| !hit(&have, w) && hit(&more, w))
                .count();
            let matched = (named + described) as f64;
            let relevance = (matched / want).min(1.0);
            if relevance < 0.5 && !(words.len() == 1 && matched >= 1.0) {
                continue;
            }
            let bonus = summary_rank(&id);
            let why = if n.ntype == "commit" {
                "matching commit"
            } else {
                "matches task words"
            };
            out.push(Seed {
                id,
                score: 6.0 * relevance + bonus,
                why,
            });
        }
    }
    Ok(out)
}

/// Extra seed score for the best summary-search hits of a bug report: the
/// first gets all of it, the next ones less.
const SUMMARY_RANK_BONUS: f64 = 4.0;
/// Summary hits that get the bonus.
const SUMMARY_RANKED: usize = 8;

/// A report of something behaving wrongly, as opposed to a question or a
/// request for new code.
pub fn asks_about_bug(task: &str) -> bool {
    let t = task.to_lowercase();
    [
        "instead of",
        "but not",
        "doesn't",
        "does not",
        "don't",
        "do not",
        "isn't",
        "is not",
        "fails",
        "failing",
        "broken",
        "wrong",
        "incorrect",
        "bug",
        "regression",
        "crash",
        "panic",
        "error",
        "drop",
        "ignore",
        "skip",
        "returns",
    ]
    .iter()
    .any(|p| t.contains(p))
        && !t.trim_start().starts_with("add ")
        && !(is_question(&t) && !t.contains("fix"))
}

/// The task names, in backticks, an identifier that does not exist in the code
/// yet (`takeEvery` in "Add `takeEvery` ..."): it is about code still to write.
pub fn names_new_symbol(db: &Db, task: &str) -> Result<bool> {
    for t in backticked(task) {
        if find(db, &t)?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The task names, in backticks, something that already exists in the code
/// (`GlobSet` in "Add a method `first_match` to `GlobSet`").
pub fn names_existing_symbol(db: &Db, task: &str) -> Result<bool> {
    for t in backticked(task) {
        if !find(db, &t)?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Asks for new code to be written: "Add a function ...", "Implement ...".
pub fn asks_for_new_code(task: &str) -> bool {
    let t = task.trim_start().to_lowercase();
    let first = t.split_whitespace().next().unwrap_or("");
    matches!(first, "add" | "implement" | "create" | "introduce" | "write" | "build")
}

/// Starts like a question about how the code works.
pub fn is_question(t: &str) -> bool {
    let t = t.trim_start().to_lowercase();
    let first = t.split_whitespace().next().unwrap_or("");
    matches!(
        first,
        "how"
            | "which"
            | "what"
            | "why"
            | "when"
            | "where"
            | "who"
            | "does"
            | "is"
            | "can"
            | "explain"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_overloads_are_shadowed_by_the_real_function() {
        let n = |q: &str, path: &str| Node {
            id: format!("sym:{path}#{q}"),
            qname: Some(q.into()),
            path: Some(path.into()),
            ..Default::default()
        };
        let mut v = vec![
            n("pkg.more.windowed", "pkg/more.py"),
            n("pkg.more.windowed#2", "pkg/more.pyi"),
            n("pkg.more.other#2", "pkg/more.pyi"),
        ];
        drop_shadowed_stubs(&mut v);
        let left: Vec<&str> = v.iter().map(|n| n.qname.as_deref().unwrap()).collect();
        assert_eq!(left, ["pkg.more.windowed", "pkg.more.other#2"]);
    }

    #[test]
    fn short_words_count_inside_literals() {
        let w = task_words("A set built from `*.gz` matches `archive.tar.gz`; go to it");
        assert!(w.contains(&"gz".to_string()));
        assert!(w.contains(&"tar".to_string()));
        assert!(!w.contains(&"go".to_string()));
        assert!(
            !task_words("Find the cause and fix it. Make sure the tests still pass.")
                .iter()
                .any(|w| ["find", "cause", "sure", "still", "pass"].contains(&w.as_str()))
        );
    }

    #[test]
    fn new_code_requests_are_told_apart() {
        assert!(asks_for_new_code("Add a data-first utility `takeEvery(data, n)` to remeda"));
        assert!(asks_for_new_code("  Implement a cache for lookups"));
        assert!(!asks_for_new_code("session token handling"));
        assert!(!asks_for_new_code("`chunk` drops the last, shorter chunk"));
        assert!(!asks_for_new_code("How does `windowed` work?"));
    }

    #[test]
    fn bug_reports_are_told_apart() {
        assert!(asks_about_bug("`chunk` drops the last, shorter chunk"));
        assert!(asks_about_bug(
            "A glob set built from `*.gz` matches `archive.gz` but not `archive.tar.gz`"
        ));
        assert!(!asks_about_bug(
            "How does remeda let the same utility be called both ways?"
        ));
        assert!(!asks_about_bug(
            "Add a function `take_every(iterable, n)`; it returns every nth item"
        ));
        assert!(!asks_about_bug(
            "Which module defines `unique_everseen`, and what does it do? Do not change any files."
        ));
        assert!(asks_about_bug(
            "Why does `chunk` drop the last chunk? Fix it."
        ));
    }

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
