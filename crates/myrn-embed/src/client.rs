//! Asking for embeddings. Every failure is `None`: callers fall back to
//! lexical search, so a missing model or helper never breaks anything.

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::helper::{read_info, Request, Response};

/// Ask a running helper.
pub fn request(data_home: &Path, texts: &[String]) -> Option<Vec<Vec<f32>>> {
    let info = read_info(data_home)?;
    let addr = SocketAddr::from(([127, 0, 0, 1], info.port));
    let stream = TcpStream::connect_timeout(&addr, Duration::from_millis(200)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .ok()?;
    let mut w = stream.try_clone().ok()?;
    let req = Request {
        token: info.token,
        texts: texts.to_vec(),
    };
    w.write_all(serde_json::to_string(&req).ok()?.as_bytes())
        .ok()?;
    w.write_all(b"\n").ok()?;
    let mut line = String::new();
    BufReader::new(stream).read_line(&mut line).ok()?;
    let resp: Response = serde_json::from_str(&line).ok()?;
    (resp.vectors.len() == texts.len()).then_some(resp.vectors)
}

/// Embed `texts`: through the helper, starting it with `helper_exe
/// embed-helper --home <data home>` when it is not running; else by loading
/// the model in this process. `None` when the model is not installed.
pub fn embed(
    data_home: &Path,
    texts: &[String],
    helper_exe: Option<&Path>,
) -> Option<Vec<Vec<f32>>> {
    if texts.is_empty() {
        return Some(Vec::new());
    }
    if !crate::installed(data_home) || !crate::supported() {
        return None;
    }
    if let Some(v) = request(data_home, texts) {
        return Some(v);
    }
    if let Some(exe) = helper_exe {
        if start(data_home, exe) {
            if let Some(v) = request(data_home, texts) {
                return Some(v);
            }
        }
    }
    local(data_home, texts)
}

/// Start the helper in the background and wait (up to 3 s) until it answers.
pub fn start(data_home: &Path, exe: &Path) -> bool {
    let spawned = Command::new(exe)
        .arg("embed-helper")
        .arg("--home")
        .arg(data_home)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    if spawned.is_err() {
        return false;
    }
    let t = Instant::now();
    while t.elapsed() < Duration::from_secs(3) {
        if request(data_home, &[String::new()]).is_some() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    false
}

/// Load the model in this process (bulk work, or no helper binary).
#[cfg(feature = "model")]
pub fn local(data_home: &Path, texts: &[String]) -> Option<Vec<Vec<f32>>> {
    let m = crate::model::Model::load(&crate::model_dir(data_home)).ok()?;
    m.encode(texts).ok()
}

#[cfg(not(feature = "model"))]
pub fn local(_data_home: &Path, _texts: &[String]) -> Option<Vec<Vec<f32>>> {
    None
}
