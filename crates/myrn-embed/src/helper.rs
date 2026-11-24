//! The helper process: loads the model once, answers embedding requests on a
//! localhost port, and exits after a quiet period. Its address and a per-run
//! token are in `<data home>/embed/helper.json`; requests without the token
//! are ignored.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub port: u16,
    pub token: String,
    pub pid: u32,
    pub dim: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub token: String,
    pub texts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub vectors: Vec<Vec<f32>>,
}

pub fn info_path(data_home: &Path) -> PathBuf {
    data_home.join("embed").join("helper.json")
}

pub fn read_info(data_home: &Path) -> Option<Info> {
    let text = std::fs::read_to_string(info_path(data_home)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Serve until no request arrives for `idle`.
#[cfg(feature = "model")]
pub fn serve(data_home: &Path, idle: Duration) -> Result<()> {
    // Another helper already listening (perhaps busy): leave it to that one.
    if crate::client::running(data_home) {
        return Ok(());
    }
    let model = crate::model::Model::load(&crate::model_dir(data_home))?;
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    listener.set_nonblocking(true)?;
    let info = Info {
        port: listener.local_addr()?.port(),
        token: token(),
        pid: std::process::id(),
        dim: model.dim(),
    };
    let path = info_path(data_home);
    std::fs::create_dir_all(path.parent().expect("has a parent"))?;
    let tmp = path.with_extension(format!("json.{}.tmp", info.pid));
    std::fs::write(&tmp, serde_json::to_string(&info)?)?;
    std::fs::rename(&tmp, &path)?;

    let mut last = Instant::now();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                last = Instant::now();
                let _ = answer(stream, &info.token, &model);
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if last.elapsed() >= idle {
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => std::thread::sleep(Duration::from_millis(20)),
        }
    }
    // Remove the address only if it is still ours.
    if read_info(data_home).is_some_and(|i| i.pid == info.pid) {
        let _ = std::fs::remove_file(&path);
    }
    Ok(())
}

#[cfg(not(feature = "model"))]
pub fn serve(_data_home: &Path, _idle: Duration) -> Result<()> {
    anyhow::bail!("this build of myrn has no embedding support")
}

#[cfg(feature = "model")]
fn answer(stream: TcpStream, token: &str, model: &crate::model::Model) -> Result<()> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let req: Request = serde_json::from_str(&line)?;
    if req.token != token {
        return Ok(());
    }
    let vectors = model.encode(&req.texts)?;
    let mut w = stream;
    w.write_all(serde_json::to_string(&Response { vectors })?.as_bytes())?;
    w.write_all(b"\n")?;
    Ok(())
}

/// A per-run secret, so only processes that can read the data directory use the helper.
fn token() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut out = String::new();
    for i in 0..4u64 {
        let mut h = RandomState::new().build_hasher();
        h.write_u64(i);
        h.write_u128(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        );
        h.write_u32(std::process::id());
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out
}
