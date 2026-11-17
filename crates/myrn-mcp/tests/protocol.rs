//! The MCP server speaks JSON-RPC correctly and its tools answer from the graph.

use myrn_engine::api::Engine;
use myrn_mcp::server::handle_line;
use serde_json::{json, Value};

fn engine() -> (tempfile::TempDir, Engine) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("src/lib.rs"),
        "/// Adds one.\npub fn inc(x: u32) -> u32 { x + 1 }\npub fn two() -> u32 { inc(1) }\n",
    )
    .unwrap();
    let repo = git2::Repository::init(&root).unwrap();
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap();
    let (e, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    (tmp, e)
}

fn call(e: &mut Engine, msg: Value) -> Value {
    handle_line(e, &msg.to_string()).expect("a response")
}

#[test]
fn handshake_list_and_call() {
    let (_tmp, mut e) = engine();
    let init = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "t", "version": "0"}}}),
    );
    assert_eq!(init["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(init["result"]["serverInfo"]["name"], "myrn");

    assert!(handle_line(
        &mut e,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#
    )
    .is_none());

    let list = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
    );
    let names: Vec<&str> = list["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        ["context", "explain", "impact", "history", "recall", "record"]
    );

    let r = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
        "params": {"name": "explain", "arguments": {"symbol": "inc"}}}),
    );
    let text = r["result"]["content"][0]["text"].as_str().unwrap();
    assert_eq!(r["result"]["isError"], false);
    assert!(
        text.contains("pub fn inc(x: u32) -> u32")
            && text.contains("Adds one.")
            && text.contains("two"),
        "{text}"
    );

    let r = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 4, "method": "tools/call",
        "params": {"name": "impact", "arguments": {"symbol": "inc"}}}),
    );
    assert!(r["result"]["content"][0]["text"]
        .as_str()
        .unwrap()
        .contains("1 direct callers"));

    let r = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 5, "method": "tools/call",
        "params": {"name": "explain", "arguments": {}}}),
    );
    assert_eq!(r["result"]["isError"], true);

    let r = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "nope"}}),
    );
    assert_eq!(r["error"]["code"], -32602);
    let r = call(
        &mut e,
        json!({"jsonrpc": "2.0", "id": 7, "method": "bogus"}),
    );
    assert_eq!(r["error"]["code"], -32601);
    let r = handle_line(&mut e, "{not json").unwrap();
    assert_eq!(r["error"]["code"], -32700);
}
