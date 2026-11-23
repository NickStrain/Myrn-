//! myrn ui: a local, read-only web page over this project's store: the project
//! map, the code graph, the notes graph (as Obsidian draws a vault), learned
//! records and sessions.
//!
//! Served on 127.0.0.1 only, one request at a time. Every request must carry
//! the per-run token from the printed URL, and the Host header must be the
//! loopback address, so other web pages open in the browser cannot read it.
//! The page is embedded in the binary and loads nothing from the network.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::ExitCode;
use std::time::Duration;

use anyhow::{Context, Result};
use myrn_engine::api::{Engine, Lookup};
use serde::Serialize;

use super::Ctx;

const PAGE: &str = include_str!("../ui/index.html");

pub fn run(ctx: &Ctx, port: u16, open: bool) -> Result<ExitCode> {
    let mut engine = ctx.engine()?;
    let listener = TcpListener::bind(("127.0.0.1", port))
        .with_context(|| format!("listening on 127.0.0.1:{port}"))?;
    let port = listener.local_addr()?.port();
    let token = token();
    let url = format!("http://127.0.0.1:{port}/?t={token}");
    println!("myrn ui for {}", engine.root().display());
    println!("  {url}");
    println!("Read-only, on this machine only. Ctrl-C to stop.");
    if open {
        open_browser(&url);
    }
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        if let Err(e) = serve(&mut engine, stream, port, &token) {
            eprintln!("myrn ui: {e:#}");
        }
    }
    Ok(ExitCode::SUCCESS)
}

struct Request {
    path: String,
    query: HashMap<String, String>,
    headers: HashMap<String, String>,
}

fn read_request(stream: &TcpStream) -> Result<Option<Request>> {
    let mut r = BufReader::new(stream);
    let mut line = String::new();
    if r.read_line(&mut line)? == 0 {
        return Ok(None);
    }
    let mut parts = line.split_whitespace();
    let (Some(method), Some(target)) = (parts.next(), parts.next()) else {
        return Ok(None);
    };
    if method != "GET" {
        return Ok(None);
    }
    let mut headers = HashMap::new();
    loop {
        let mut h = String::new();
        if r.read_line(&mut h)? == 0 || h.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
        if headers.len() > 100 {
            break;
        }
    }
    let (path, qs) = target.split_once('?').unwrap_or((target, ""));
    let query = qs
        .split('&')
        .filter_map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            (!k.is_empty()).then(|| (decode(k), decode(v)))
        })
        .collect();
    Ok(Some(Request {
        path: decode(path),
        query,
        headers,
    }))
}

fn serve(engine: &mut Engine, mut stream: TcpStream, port: u16, token: &str) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    let Some(req) = read_request(&stream)? else {
        return respond(&mut stream, 405, "text/plain", b"GET only\n");
    };
    // Only this machine's loopback name: defeats DNS rebinding.
    let host = req.headers.get("host").map(String::as_str).unwrap_or("");
    if host != format!("127.0.0.1:{port}") && host != format!("localhost:{port}") {
        return respond(&mut stream, 403, "text/plain", b"forbidden host\n");
    }
    let given = req
        .headers
        .get("x-myrn-token")
        .or_else(|| req.query.get("t"))
        .map(String::as_str)
        .unwrap_or("");
    if given != token {
        return respond(
            &mut stream,
            403,
            "text/plain",
            b"missing or wrong token: open the URL `myrn ui` printed\n",
        );
    }
    if req.path == "/" {
        return respond(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            PAGE.as_bytes(),
        );
    }
    let Some(api) = req.path.strip_prefix("/api/") else {
        return respond(&mut stream, 404, "text/plain", b"not found\n");
    };
    // Follow edits made while the page is open.
    if let Err(e) = engine.refresh_if_older(5) {
        eprintln!("myrn ui: refresh failed: {e:#}");
    }
    let q = |k: &str| req.query.get(k).map(String::as_str);
    let body = match api {
        "overview" => json(&engine.status()?),
        "map" => json(&engine.viz_map()?),
        "graph" => json(&engine.viz_code_graph(
            q("level").unwrap_or("files"),
            q("focus"),
            q("depth").and_then(|d| d.parse().ok()).unwrap_or(1),
        )?),
        "notes" => json(&engine.viz_notes_graph(q("code") == Some("1"))?),
        "records" => json(&engine.viz_records()?),
        "record" => json(&engine.trace(q("id").unwrap_or(""))?),
        "sessions" => json(&engine.viz_sessions(500)?),
        "session" => json(&engine.episode_log(q("id").unwrap_or(""))?),
        "node" => json(&engine.viz_details(q("id").unwrap_or(""))?),
        "search" => {
            #[derive(Serialize)]
            struct Hit {
                id: String,
                label: String,
                #[serde(rename = "type")]
                ntype: String,
                location: String,
            }
            let hits: Vec<Hit> = engine
                .find(q("q").unwrap_or(""))?
                .into_iter()
                .take(30)
                .map(|n| Hit {
                    id: n.id.clone(),
                    label: n.display_name().to_string(),
                    ntype: n.ntype.clone(),
                    location: n.location(),
                })
                .collect();
            json(&hits)
        }
        "explain" => match engine.explain(q("q").unwrap_or(""))? {
            Lookup::Found(e) => json(&serde_json::json!({"text": e.render()})),
            other => json(&serde_json::json!({"text": other.render(|_| String::new())})),
        },
        _ => return respond(&mut stream, 404, "text/plain", b"not found\n"),
    };
    match body {
        Ok(b) => respond(&mut stream, 200, "application/json", b.as_bytes()),
        Err(e) => respond(
            &mut stream,
            500,
            "text/plain",
            format!("{e:#}\n").as_bytes(),
        ),
    }
}

fn json<T: Serialize>(v: &T) -> Result<String> {
    Ok(serde_json::to_string(v)?)
}

fn respond(stream: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) -> Result<()> {
    let reason = match code {
        200 => "OK",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    };
    let head = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\n\
         Cache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\n\
         Content-Security-Policy: default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'; img-src 'self' data:\r\n\
         Connection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

/// Percent-decoding for paths and query values (`+` is a space).
fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < b.len() => {
                let hex = std::str::from_utf8(&b[i + 1..i + 3]).ok();
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(v) => {
                        out.push(v);
                        i += 2;
                    }
                    None => out.push(b'%'),
                }
            }
            c => out.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn token() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut out = String::new();
    for i in 0..2u64 {
        let mut h = RandomState::new().build_hasher();
        h.write_u64(i);
        h.write_u32(std::process::id());
        h.write_u128(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        );
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out
}

fn open_browser(url: &str) {
    let r = if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
    } else {
        std::process::Command::new("xdg-open").arg(url).spawn()
    };
    if r.is_err() {
        eprintln!("(could not open a browser; open the URL above)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_queries() {
        assert_eq!(decode("sym%3Acrates%2Fa.rs%23f"), "sym:crates/a.rs#f");
        assert_eq!(decode("a+b"), "a b");
        assert_eq!(decode("100%"), "100%");
        assert_eq!(decode("%zz"), "%zz");
    }

    #[test]
    fn tokens_differ() {
        assert_eq!(token().len(), 32);
        assert_ne!(token(), token());
    }
}
