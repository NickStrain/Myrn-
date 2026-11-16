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

    // Injection is off by default: hooks answer with nothing.
    let cwd = env.repo.join("src");
    let start = json!({"session_id": "s1", "cwd": cwd, "hook_event_name": "SessionStart", "source": "startup"}).to_string();
    let (o, _) = env.hook("claude", "SessionStart", &start, &env.repo);
    assert!(o.status.success());
    assert!(o.stdout.is_empty(), "{}", text(&o));
    let prompt = json!({"session_id": "s1", "cwd": env.repo, "hook_event_name": "UserPromptSubmit",
        "prompt": "refresh_session returns the wrong value"})
    .to_string();
    let (o, _) = env.hook("claude", "UserPromptSubmit", &prompt, &env.repo);
    assert!(o.stdout.is_empty());

    // Turn injection on: the prompt hook returns additionalContext naming the symbol.
    let cfg = env.repo.join(".myrn/project.toml");
    let toml = std::fs::read_to_string(&cfg)
        .unwrap()
        .replace("inject = false", "inject = true");
    std::fs::write(&cfg, toml).unwrap();
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
    let (o, _) = env.hook("claude", "SessionStart", &start, &env.repo);
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert!(v["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap()
        .contains("project map"));

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
