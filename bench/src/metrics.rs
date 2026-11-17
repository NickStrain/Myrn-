//! Metrics from one headless Claude Code session (`--output-format
//! stream-json`): tokens, tool calls, exploration before the first correct
//! edit, and the final answer. Plus the small statistics the report needs.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    pub input: u64,
    pub cache_creation: u64,
    pub cache_read: u64,
    pub output: u64,
}

impl Tokens {
    /// Everything the model processed or produced, injected context included.
    pub fn total(&self) -> u64 {
        self.input + self.cache_creation + self.cache_read + self.output
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub session_id: Option<String>,
    pub tokens: Tokens,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub num_turns: Option<u64>,
    pub is_error: bool,
    /// The final message text.
    pub answer: String,
    pub tool_calls: usize,
    /// Reads and searches (including myrn's own tools) before the first edit
    /// to a file the task expects to change; `None` if no such edit happened.
    pub exploration_before_edit: Option<usize>,
    /// Reads and searches in the whole session.
    pub exploration_total: usize,
    pub edits: usize,
    /// Edits made through shell commands (`sed -i`, redirects, `patch`).
    #[serde(default)]
    pub shell_edits: usize,
    /// Repo-relative files the agent read or edited.
    pub files_seen: BTreeSet<String>,
    pub myrn_tool_calls: usize,
}

const EDIT_TOOLS: &[&str] = &["Edit", "MultiEdit", "Write", "NotebookEdit"];
const READ_TOOLS: &[&str] = &["Read", "Grep", "Glob", "LS", "NotebookRead"];
/// Shell commands that only look around.
const LOOK_COMMANDS: &[&str] = &[
    "cat",
    "head",
    "tail",
    "less",
    "grep",
    "rg",
    "ag",
    "find",
    "fd",
    "ls",
    "tree",
    "wc",
    "sed -n",
    "awk",
    "git log",
    "git show",
    "git grep",
    "git blame",
    "git diff",
];

/// Parse a stream-json transcript. `root` makes absolute paths repo-relative;
/// `touches` are the files where the correct edit lands (empty: any edit).
pub fn parse(stream: &str, root: &str, touches: &[String]) -> Session {
    let mut s = Session::default();
    let mut explored = 0usize;
    let mut last_text = String::new();
    let rel = |p: &str| -> String {
        let p = p.strip_prefix(root).unwrap_or(p);
        p.trim_start_matches('/').to_string()
    };
    for line in stream.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if s.session_id.is_none() {
            s.session_id = v["session_id"].as_str().map(String::from);
        }
        match v["type"].as_str() {
            Some("assistant") => {
                for block in v["message"]["content"].as_array().into_iter().flatten() {
                    match block["type"].as_str() {
                        Some("text") => {
                            last_text = block["text"].as_str().unwrap_or("").to_string();
                        }
                        Some("tool_use") => {
                            s.tool_calls += 1;
                            let name = block["name"].as_str().unwrap_or("");
                            let input = &block["input"];
                            let path = input["file_path"]
                                .as_str()
                                .or_else(|| input["notebook_path"].as_str())
                                .or_else(|| input["path"].as_str())
                                .map(rel);
                            if let Some(p) = &path {
                                if !p.is_empty() {
                                    s.files_seen.insert(p.clone());
                                }
                            }
                            if name.starts_with("mcp__myrn__") {
                                s.myrn_tool_calls += 1;
                            }
                            let shell =
                                (name == "Bash").then(|| input["command"].as_str().unwrap_or(""));
                            let shell_edit = shell.is_some_and(is_shell_write);
                            if EDIT_TOOLS.contains(&name) || shell_edit {
                                s.edits += 1;
                                if shell_edit {
                                    s.shell_edits += 1;
                                }
                                let correct = touches.is_empty()
                                    || path
                                        .as_deref()
                                        .is_some_and(|p| touches.iter().any(|t| t == p))
                                    || shell.is_some_and(|c| {
                                        touches.iter().any(|t| c.contains(t.as_str()))
                                    });
                                if correct && s.exploration_before_edit.is_none() {
                                    s.exploration_before_edit = Some(explored);
                                }
                            }
                            // A shell command can look and write at once
                            // (`grep … && sed -i …`); the look counts too.
                            if is_exploration(name, input) {
                                explored += 1;
                                s.exploration_total += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Some("result") => {
                let u = &v["usage"];
                let n = |k: &str| u[k].as_u64().unwrap_or(0);
                s.tokens = Tokens {
                    input: n("input_tokens"),
                    cache_creation: n("cache_creation_input_tokens"),
                    cache_read: n("cache_read_input_tokens"),
                    output: n("output_tokens"),
                };
                s.cost_usd = v["total_cost_usd"].as_f64();
                s.duration_ms = v["duration_ms"].as_u64();
                s.num_turns = v["num_turns"].as_u64();
                s.is_error = v["is_error"].as_bool().unwrap_or(false);
                if let Some(r) = v["result"].as_str() {
                    last_text = r.to_string();
                }
            }
            _ => {}
        }
    }
    s.answer = last_text;
    s
}

fn is_exploration(name: &str, input: &Value) -> bool {
    if READ_TOOLS.contains(&name) || name.starts_with("mcp__myrn__") {
        return true;
    }
    if name == "Bash" {
        let cmd = input["command"].as_str().unwrap_or("");
        return segments(cmd).any(|seg| {
            !is_write_segment(seg)
                && LOOK_COMMANDS
                    .iter()
                    .any(|c| seg == *c || seg.starts_with(&format!("{c} ")))
        });
    }
    false
}

/// The parts of a shell command line, without `cd …` steps.
fn segments(cmd: &str) -> impl Iterator<Item = &str> {
    cmd.split(['&', ';', '|', '\n'])
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with("cd "))
}

/// Does this shell command change files? In-place editors, redirects into a
/// file, `tee`, and patch tools.
pub fn is_shell_write(cmd: &str) -> bool {
    segments(cmd).any(is_write_segment)
}

fn is_write_segment(seg: &str) -> bool {
    let words: Vec<&str> = seg.split_whitespace().collect();
    let Some(&first) = words.first() else {
        return false;
    };
    let in_place = |flag: &str| words[1..].iter().any(|w| w.starts_with(flag));
    match first {
        "sed" | "gsed" => in_place("-i") || words.contains(&"--in-place"),
        "perl" => words[1..]
            .iter()
            .any(|w| w.starts_with("-") && w.contains('i')),
        "tee" | "patch" => true,
        "git" => words.get(1) == Some(&"apply"),
        _ => redirects_to_file(seg),
    }
}

/// `cmd > file` or `cmd >> file`, but not `2>&1` or `> /dev/null`.
fn redirects_to_file(seg: &str) -> bool {
    let b = seg.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'>' {
            let prev = if i > 0 { b[i - 1] } else { b' ' };
            let mut j = i + 1;
            if j < b.len() && b[j] == b'>' {
                j += 1;
            }
            let target = seg[j..].trim_start();
            let fd_dup = prev.is_ascii_digit() || target.starts_with('&');
            if !fd_dup && !target.is_empty() && !target.starts_with("/dev/") {
                return true;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    false
}

/// Do all expected strings appear in the answer (ignoring case)?
pub fn answer_ok(answer: &str, expected: &[String]) -> bool {
    let a = answer.to_lowercase();
    expected.iter().all(|e| a.contains(&e.to_lowercase()))
}

// ------------------------------------------------------------ statistics

pub fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

/// Half-width of the 95% confidence interval of the mean (Student's t).
pub fn ci95(xs: &[f64]) -> Option<f64> {
    let n = xs.len();
    if n < 2 {
        return None;
    }
    let m = mean(xs)?;
    let var = xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1) as f64;
    Some(t95(n - 1) * (var / n as f64).sqrt())
}

/// Two-sided 95% critical value of Student's t.
fn t95(df: usize) -> f64 {
    const T: [f64; 30] = [
        12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179, 2.160,
        2.145, 2.131, 2.120, 2.110, 2.101, 2.093, 2.086, 2.080, 2.074, 2.069, 2.064, 2.060, 2.056,
        2.052, 2.048, 2.045, 2.042,
    ];
    match df {
        0 => f64::INFINITY,
        1..=30 => T[df - 1],
        31..=60 => 2.000,
        61..=120 => 1.980,
        _ => 1.960,
    }
}

pub fn median(xs: &mut [u64]) -> Option<u64> {
    if xs.is_empty() {
        return None;
    }
    xs.sort_unstable();
    Some(xs[xs.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const STREAM: &str = r#"{"type":"system","subtype":"init","session_id":"s-1","tools":["Read"]}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"text","text":"Looking."},{"type":"tool_use","id":"1","name":"Grep","input":{"pattern":"ext","path":"/w/rg/crates"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"2","name":"Read","input":{"file_path":"/w/rg/crates/globset/src/lib.rs"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"3","name":"Edit","input":{"file_path":"/w/rg/README.md"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"4","name":"mcp__myrn__explain","input":{"query":"file_name_ext"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"5","name":"Bash","input":{"command":"rg -n rfind crates/globset"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"6","name":"Edit","input":{"file_path":"/w/rg/crates/globset/src/pathutil.rs"}}]}}
{"type":"assistant","session_id":"s-1","message":{"content":[{"type":"tool_use","id":"7","name":"Bash","input":{"command":"cargo test -p globset"}}]}}
{"type":"result","subtype":"success","is_error":false,"duration_ms":61000,"num_turns":8,"result":"Fixed file_name_ext.","total_cost_usd":0.42,"usage":{"input_tokens":120,"cache_creation_input_tokens":5000,"cache_read_input_tokens":40000,"output_tokens":900}}
"#;

    #[test]
    fn parses_a_transcript() {
        let s = parse(STREAM, "/w/rg", &["crates/globset/src/pathutil.rs".into()]);
        assert_eq!(s.session_id.as_deref(), Some("s-1"));
        assert_eq!(s.tokens.total(), 46_020);
        assert_eq!(s.tool_calls, 7);
        assert_eq!(s.edits, 2);
        // Grep, Read, explain, rg: the README edit doesn't count as correct.
        assert_eq!(s.exploration_before_edit, Some(4));
        assert_eq!(s.exploration_total, 4);
        assert_eq!(s.myrn_tool_calls, 1);
        assert_eq!(s.answer, "Fixed file_name_ext.");
        assert_eq!(s.duration_ms, Some(61_000));
        assert!(s.files_seen.contains("crates/globset/src/pathutil.rs"));
        assert!(s.files_seen.contains("crates"));
    }

    #[test]
    fn shell_commands() {
        for c in [
            "sed -i 's/a/b/' src/a.py",
            "cd x && sed -i.bak 's/a/b/' a.py",
            "perl -pi -e 's/a/b/' a.py",
            "cat > src/new.py <<'EOF'",
            "echo x >> notes.txt",
            "git apply fix.patch",
            "printf x | tee a.txt",
        ] {
            assert!(is_shell_write(c), "{c}");
        }
        for c in [
            "sed -n 1,20p a.py",
            "cargo test 2>&1 | tail",
            "grep -rn foo . > /dev/null",
            "git log -p",
            "python3 -m unittest 2>&1",
        ] {
            assert!(!is_shell_write(c), "{c}");
        }
    }

    /// The first run's real pattern: read with grep/sed -n, fix with sed -i.
    #[test]
    fn shell_reads_and_edits_count() {
        let stream = r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"grep -n \"def collapse\" -A45 more_itertools/more.py"}}]}}
{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"sed -n 1466,1485p more_itertools/more.py; git log -L1447,1480:more_itertools/more.py"}}]}}
{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"sed -i '1456s/>=/>/' more_itertools/more.py && sed -n 1456p more_itertools/more.py"}}]}}
"#;
        let s = parse(stream, "/w", &["more_itertools/more.py".into()]);
        assert_eq!(s.exploration_before_edit, Some(2));
        assert_eq!((s.edits, s.shell_edits), (1, 1));
        assert_eq!(s.exploration_total, 3);
    }

    #[test]
    fn answers_and_stats() {
        assert!(answer_ok(
            "It is `BinaryDetection::quit`",
            &["binarydetection".into(), "QUIT".into()]
        ));
        assert!(!answer_ok("quit", &["convert".into()]));
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
        let ci = ci95(&[1.0, 2.0, 3.0]).unwrap();
        assert!((ci - 4.303 * (1.0f64 / 3.0).sqrt()).abs() < 1e-9);
        assert_eq!(ci95(&[1.0]), None);
        assert_eq!(median(&mut [5, 1, 3]), Some(3));
    }
}
