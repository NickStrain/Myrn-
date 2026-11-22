//! End to end: init, connect (consent), hooks, doctor, pull-only, disconnect,
//! driving the real `myrn` binary the way an agent would.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use serde_json::{json, Value};

struct Env {
    _tmp: tempfile::TempDir,
    repo: PathBuf,
    home: PathBuf,
}

fn setup() -> Env {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(
        repo.join("src/lib.rs"),
        "/// Issues a new session when the token is still valid.\npub fn refresh_session(t: u32) -> u32 { validate_token(t) }\npub fn validate_token(t: u32) -> u32 { t }\n",
    )
    .unwrap();
    let r = git2::Repository::init(&repo).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    r.commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap();
    let home = tmp.path().join("home");
    let env = Env {
        _tmp: tmp,
        repo,
        home,
    };
    assert!(env.myrn(&["init"]).status.success());
    env
}

impl Env {
    fn cmd(&self, args: &[&str]) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_myrn"));
        c.args(args)
            .current_dir(&self.repo)
            .env("MYRN_HOME", &self.home)
            .env("MYRN_NO_AGENT_CLI", "1")
            .stdin(Stdio::null());
        c
    }

    fn myrn(&self, args: &[&str]) -> Output {
        self.cmd(args).output().unwrap()
    }

    /// Run a hook the way an agent does: payload on stdin, from some directory.
    fn hook(&self, agent: &str, event: &str, payload: &str, dir: &Path) -> (Output, u128) {
        let start = Instant::now();
        let mut child = self
            .cmd(&["hook", agent, event])
            .current_dir(dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(payload.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        (out, start.elapsed().as_millis())
    }

    fn settings(&self) -> Value {
        serde_json::from_str(
            &std::fs::read_to_string(self.repo.join(".claude/settings.local.json")).unwrap(),
        )
        .unwrap()
    }
}

fn text(o: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}

#[test]
fn connect_hooks_doctor_disconnect() {
    let env = setup();

    // No terminal and no --yes: consent defaults to no, nothing is written.
    let o = env.myrn(&["connect", "--agent", "claude"]);
    assert!(!o.status.success());
    assert!(
        text(&o).contains("Session hooks let myrn see"),
        "{}",
        text(&o)
    );
    assert!(!env.repo.join(".claude").exists());

    let o = env.myrn(&["connect", "--agent", "claude", "--yes"]);
    assert!(o.status.success(), "{}", text(&o));
    assert!(
        text(&o).contains("claude mcp add --scope local myrn"),
        "manual MCP step shown: {}",
        text(&o)
    );
    let s = env.settings();
    let h = &s["hooks"]["UserPromptSubmit"][0]["hooks"][0];
    assert_eq!(h["args"], json!(["hook", "claude", "UserPromptSubmit"]));
    assert!(Path::new(h["command"].as_str().unwrap()).is_absolute());

    // Injection is on by default: the session starts with the project map...
    let cwd = env.repo.join("src");
    let start = json!({"session_id": "s1", "cwd": cwd, "hook_event_name": "SessionStart", "source": "startup"}).to_string();
    let (o, _) = env.hook("claude", "SessionStart", &start, &env.repo);
    assert!(o.status.success());
    let v: Value =
        serde_json::from_slice(&o.stdout).unwrap_or_else(|e| panic!("{e}: {}", text(&o)));
    assert!(v["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap()
        .contains("project map"));

    // ...and a prompt gets additionalContext naming the symbol.
    let prompt = json!({"session_id": "s1", "cwd": env.repo, "hook_event_name": "UserPromptSubmit",
        "prompt": "refresh_session returns the wrong value"})
    .to_string();
    let (o, ms) = env.hook("claude", "UserPromptSubmit", &prompt, &env.repo);
    assert!(o.status.success());
    let v: Value =
        serde_json::from_slice(&o.stdout).unwrap_or_else(|e| panic!("{e}: {}", text(&o)));
    assert_eq!(v["hookSpecificOutput"]["hookEventName"], "UserPromptSubmit");
    let ctx = v["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert!(
        ctx.contains("refresh_session") && ctx.contains("validate_token"),
        "{ctx}"
    );
    eprintln!("prompt hook with injection: {ms} ms end to end (debug build)");

    // The same session is never sent the same code twice.
    let (o, _) = env.hook("claude", "UserPromptSubmit", &prompt, &env.repo);
    assert!(o.status.success() && o.stdout.is_empty(), "{}", text(&o));
    // A new session gets it again.
    let other = prompt.replace("\"s1\"", "\"s2\"");
    let (o, _) = env.hook("claude", "UserPromptSubmit", &other, &env.repo);
    assert!(text(&o).contains("refresh_session"), "{}", text(&o));

    // Turn injection off: hooks answer with nothing.
    let cfg = env.repo.join(".myrn/project.toml");
    let toml = std::fs::read_to_string(&cfg)
        .unwrap()
        .replace("inject = true", "inject = false");
    std::fs::write(&cfg, toml).unwrap();
    let (o, _) = env.hook("claude", "SessionStart", &start, &env.repo);
    assert!(o.status.success() && o.stdout.is_empty(), "{}", text(&o));
    let (o, _) = env.hook("claude", "UserPromptSubmit", &other, &env.repo);
    assert!(o.stdout.is_empty(), "{}", text(&o));

    // Never in the way: junk payloads, unknown events, unconnected agents.
    for (agent, event, payload) in [
        ("claude", "UserPromptSubmit", "not json"),
        ("claude", "Notification", "{}"),
        ("codex", "UserPromptSubmit", prompt.as_str()),
        ("nope", "SessionStart", "{}"),
    ] {
        let (o, _) = env.hook(agent, event, payload, &env.repo);
        assert!(
            o.status.success() && o.stdout.is_empty(),
            "{agent} {event}: {}",
            text(&o)
        );
    }
    // A hook from a directory that is not set up does nothing.
    let other = tempfile::tempdir().unwrap();
    let (o, _) = env.hook("claude", "SessionStart", "{}", other.path());
    assert!(o.status.success() && o.stdout.is_empty());

    let o = env.myrn(&["doctor"]);
    let t = text(&o);
    assert!(t.contains("UserPromptSubmit hook ->"), "{t}");
    assert!(t.contains("hook latency") && t.contains("median"), "{t}");
    assert!(!t.contains("FAIL"), "{t}");

    // Drift: someone deletes a hook by hand.
    let mut s = env.settings();
    s["hooks"].as_object_mut().unwrap().remove("Stop");
    std::fs::write(env.repo.join(".claude/settings.local.json"), s.to_string()).unwrap();
    let t = text(&env.myrn(&["doctor"]));
    assert!(t.contains("FAIL") && t.contains("Stop hook missing"), "{t}");

    // Codex, pull-only: MCP config and an AGENTS.md line, no hooks, no prompt.
    let o = env.myrn(&["connect", "--agent", "codex", "--pull-only"]);
    assert!(o.status.success(), "{}", text(&o));
    assert!(std::fs::read_to_string(env.repo.join("AGENTS.md"))
        .unwrap()
        .contains("myrn:start"));
    assert!(std::fs::read_to_string(env.repo.join(".codex/config.toml"))
        .unwrap()
        .contains("[mcp_servers.myrn]"));
    assert!(!env.repo.join(".codex/hooks.json").exists());
    let status = text(&env.myrn(&["status"]));
    assert!(status.contains("claude, codex"), "{status}");

    // Disconnect everything: config back to how it was.
    let o = env.myrn(&["disconnect"]);
    assert!(o.status.success(), "{}", text(&o));
    assert!(env.settings().get("hooks").is_none());
    assert!(!env.repo.join("AGENTS.md").exists());
    assert!(!env.repo.join(".codex").exists());
    assert!(text(&env.myrn(&["status"])).contains("none connected"));
}

#[test]
fn sessions_are_recorded_redacted_and_listed() {
    let env = setup();
    std::fs::write(env.repo.join(".myrn/ignore"), "secrets/\n").unwrap();
    // This test is about recording; keep injected context out of the hook output.
    let cfg = env.repo.join(".myrn/project.toml");
    let toml = std::fs::read_to_string(&cfg)
        .unwrap()
        .replace("inject = true", "inject = false");
    std::fs::write(&cfg, toml).unwrap();

    // Before consent: hooks record nothing.
    let o = env.myrn(&["connect", "--agent", "codex", "--pull-only"]);
    assert!(o.status.success(), "{}", text(&o));
    let ev = |name: &str, extra: Value| {
        let mut v = json!({"session_id": "sess-42", "cwd": env.repo, "hook_event_name": name});
        v.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        v.to_string()
    };
    env.hook(
        "codex",
        "SessionStart",
        &ev("SessionStart", json!({"source": "startup"})),
        &env.repo,
    );
    assert!(text(&env.myrn(&["log"])).contains("Recording is off"));

    let o = env.myrn(&["connect", "--agent", "claude", "--yes"]);
    assert!(o.status.success(), "{}", text(&o));
    let lib = env.repo.join("src/lib.rs");
    let session = [
        ("SessionStart", json!({"source": "startup"})),
        (
            "UserPromptSubmit",
            json!({"prompt": "fix refresh_session; the key is ghp_1234567890abcdefghijABCDEFGHIJ123456"}),
        ),
        (
            "PostToolUse",
            json!({"tool_name": "Read", "tool_input": {"file_path": lib}, "tool_response": {}}),
        ),
        (
            "PostToolUse",
            json!({"tool_name": "Read", "tool_input": {"file_path": env.repo.join("secrets/prod.env")}}),
        ),
    ];
    for (name, extra) in session {
        let (o, _) = env.hook("claude", name, &ev(name, extra), &env.repo);
        assert!(
            o.status.success() && o.stdout.is_empty(),
            "{name}: {}",
            text(&o)
        );
    }
    // The agent edits one function.
    let src = std::fs::read_to_string(&lib)
        .unwrap()
        .replace("{ validate_token(t) }", "{ validate_token(t + 1) }");
    std::fs::write(&lib, src).unwrap();
    for (name, extra) in [
        (
            "PostToolUse",
            json!({"tool_name": "Edit", "tool_input": {"file_path": lib, "old_string": "a", "new_string": "b"}}),
        ),
        (
            "PostToolUseFailure",
            json!({"tool_name": "Bash", "tool_input": {"command": "DB_PASSWORD=hunter2 cargo test"},
                   "error": "test refresh ... FAILED"}),
        ),
        (
            "PostToolUse",
            json!({"tool_name": "Bash", "tool_input": {"command": "cargo test"},
                   "tool_response": {"stdout": "ok"}}),
        ),
    ] {
        env.hook("claude", name, &ev(name, extra), &env.repo);
    }
    // A commit lands during the session.
    let r = git2::Repository::open(&env.repo).unwrap();
    let mut index = r.index().unwrap();
    index.add_path(Path::new("src/lib.rs")).unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    let parent = r.head().unwrap().peel_to_commit().unwrap();
    r.commit(Some("HEAD"), &sig, &sig, "fix refresh", &tree, &[&parent])
        .unwrap();
    env.hook("claude", "Stop", &ev("Stop", json!({})), &env.repo);

    // The raw log: redacted, no ignored path, nothing from before consent.
    let logs: Vec<_> = std::fs::read_dir(env.home.join("projects"))
        .unwrap()
        .flat_map(|p| std::fs::read_dir(p.unwrap().path().join("episodes")).unwrap())
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(logs.len(), 1, "{logs:?}");
    let raw = std::fs::read_to_string(&logs[0]).unwrap();
    assert!(!raw.contains("ghp_") && !raw.contains("hunter2"), "{raw}");
    assert!(!raw.contains("secrets/"), "{raw}");
    assert!(raw.contains("[REDACTED:github-token]") && raw.contains("[REDACTED:secret]"));
    assert_eq!(
        raw.lines().filter(|l| l.contains("session_start")).count(),
        1
    );

    let o = env.myrn(&["log", "--json"]);
    let v: Value =
        serde_json::from_slice(&o.stdout).unwrap_or_else(|e| panic!("{e}: {}", text(&o)));
    let ep = &v[0];
    assert_eq!(ep["session"], "sess-42");
    assert_eq!(ep["files_edited"], json!(["src/lib.rs"]));
    assert_eq!(
        ep["symbols_touched"],
        json!(["sym:src/lib.rs#refresh_session"])
    );
    assert_eq!(
        (ep["tests_failed"].as_i64(), ep["tests_passed"].as_i64()),
        (Some(1), Some(1))
    );
    assert!(
        ep["commits"][0].as_str().unwrap().ends_with("fix refresh"),
        "{ep}"
    );

    let t = text(&env.myrn(&["log"]));
    assert!(
        t.contains("tests_passed") && t.contains("fix refresh_session"),
        "{t}"
    );
    let t = text(&env.myrn(&["log", "sess"]));
    assert!(
        t.contains("[test fail]") && t.contains("sym:src/lib.rs#refresh_session"),
        "{t}"
    );

    // The Episode node is in the graph with a touches edge, and survives rebuild.
    for _ in 0..2 {
        let t = text(&env.myrn(&["show", "episode:sess-42"]));
        assert!(
            t.contains("touches") && t.contains("refresh_session"),
            "{t}"
        );
        assert!(env.myrn(&["rebuild"]).status.success());
    }

    // Disconnect withdraws consent: later events are not recorded.
    assert!(env.myrn(&["disconnect"]).status.success());
    let before = std::fs::read_to_string(&logs[0]).unwrap();
    env.hook("claude", "Stop", &ev("Stop", json!({})), &env.repo);
    assert_eq!(std::fs::read_to_string(&logs[0]).unwrap(), before);
}
