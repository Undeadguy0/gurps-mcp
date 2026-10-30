//! Смоук stdio: initialize + tools/list против запущенного бинаря.
mod common;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::test]
async fn stdio_initialize_and_list_tools() {
    let Some((s, _db)) = common::start().await else {
        return;
    };
    let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_gurps-mcp"))
        .env("SURREALDB_HOST", &s.cfg.host)
        .env("SURREALDB_NAMESPACE", &s.cfg.namespace)
        .env("SURREALDB_NAME", &s.cfg.database)
        .env("SURREALDB_USER", &s.cfg.user)
        .env("SURREALDB_PASSWORD", &s.cfg.password)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut out = BufReader::new(child.stdout.take().unwrap()).lines();
    let send = |v: serde_json::Value| format!("{v}\n");

    stdin.write_all(send(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
        "protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}})).as_bytes()).await.unwrap();
    let init: serde_json::Value =
        serde_json::from_str(&out.next_line().await.unwrap().unwrap()).unwrap();
    assert_eq!(init["result"]["serverInfo"]["name"], "gurps-mcp", "{init}");

    stdin
        .write_all(
            send(serde_json::json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
                .as_bytes(),
        )
        .await
        .unwrap();
    stdin
        .write_all(
            send(serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})).as_bytes(),
        )
        .await
        .unwrap();
    let list: serde_json::Value =
        serde_json::from_str(&out.next_line().await.unwrap().unwrap()).unwrap();
    let mut names: Vec<&str> = list["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "check_prereqs",
            "compute_character",
            "get_character",
            "get_entry",
            "list_packs",
            "search"
        ],
        "{list}"
    );

    stdin
        .write_all(
            send(
                serde_json::json!({"jsonrpc":"2.0","id":3,"method":"tools/call",
        "params":{"name":"get_entry","arguments":{"table":"skill","sid":"sword_a"}}}),
            )
            .as_bytes(),
        )
        .await
        .unwrap();
    let call: serde_json::Value =
        serde_json::from_str(&out.next_line().await.unwrap().unwrap()).unwrap();
    assert!(
        call["result"]["structuredContent"]["record"]["id"]
            .to_string()
            .contains("homebrew"),
        "{call}"
    );

    stdin
        .write_all(
            send(
                serde_json::json!({"jsonrpc":"2.0","id":4,"method":"tools/call",
        "params":{"name":"compute_character","arguments":{"id":"test_hero"}}}),
            )
            .as_bytes(),
        )
        .await
        .unwrap();
    let call: serde_json::Value =
        serde_json::from_str(&out.next_line().await.unwrap().unwrap()).unwrap();
    let sc = &call["result"]["structuredContent"];
    assert!(sc["attributes"]["values"]["ST"].is_number(), "{call}");
    assert!(
        sc["attacks"].as_array().is_some_and(|a| !a.is_empty()),
        "{call}"
    );
}
