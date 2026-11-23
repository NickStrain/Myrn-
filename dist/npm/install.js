#!/usr/bin/env node
// Downloads the myrn binary for this platform from the GitHub release that
// matches this package's version, checks it against the release's
// sha256sums.txt, and unpacks it into ./bin-cache. Runs as `postinstall`, and
// again from bin/myrn.js on first use if install scripts were skipped.
//
// Environment:
//   MYRN_BINARY_PATH     use this binary instead of downloading one
//   MYRN_DOWNLOAD_BASE   download from here instead of the GitHub release
"use strict";

const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const zlib = require("zlib");
const { spawnSync } = require("child_process");

const pkg = require("./package.json");
const REPO = "NickStrain/Myrn-";
const CACHE = path.join(__dirname, "bin-cache");

const TARGETS = {
  "linux-x64": "x86_64-unknown-linux-musl",
  "linux-arm64": "aarch64-unknown-linux-musl",
  "darwin-x64": "x86_64-apple-darwin",
  "darwin-arm64": "aarch64-apple-darwin",
  "win32-x64": "x86_64-pc-windows-msvc",
  "win32-arm64": "aarch64-pc-windows-msvc",
};

function target(platform = process.platform, arch = process.arch) {
  const t = TARGETS[`${platform}-${arch}`];
  if (!t) {
    throw new Error(
      `myrn has no prebuilt binary for ${platform}-${arch}. ` +
        `Build it from source: cargo install --git https://github.com/${REPO} myrn`
    );
  }
  return t;
}

function archiveName(t) {
  return `myrn-${t}.${t.includes("windows") ? "zip" : "tar.gz"}`;
}

function binaryPath() {
  if (process.env.MYRN_BINARY_PATH) return process.env.MYRN_BINARY_PATH;
  return path.join(CACHE, process.platform === "win32" ? "myrn.exe" : "myrn");
}

function baseUrl() {
  return (
    process.env.MYRN_DOWNLOAD_BASE ||
    `https://github.com/${REPO}/releases/download/v${pkg.version}`
  ).replace(/\/+$/, "");
}

async function download(url) {
  const res = await fetch(url, { redirect: "follow" });
  if (!res.ok) throw new Error(`download failed: ${url} (${res.status} ${res.statusText})`);
  return Buffer.from(await res.arrayBuffer());
}

function expectedSum(sums, name) {
  for (const line of sums.split(/\r?\n/)) {
    const [hash, file] = line.trim().split(/\s+/);
    if (file && file.replace(/^\*/, "") === name) return hash.toLowerCase();
  }
  throw new Error(`${name} is not listed in sha256sums.txt`);
}

// Minimal reader for the ustar archives the release builds: regular files only.
function untar(buf) {
  const files = {};
  let off = 0;
  while (off + 512 <= buf.length) {
    const header = buf.subarray(off, off + 512);
    if (header.every((b) => b === 0)) break;
    const str = (a, b) => header.subarray(a, b).toString("utf8").replace(/\0.*$/s, "");
    let name = str(0, 100);
    const prefix = str(345, 500);
    if (prefix) name = `${prefix}/${name}`;
    const size = parseInt(str(124, 136).trim() || "0", 8);
    const type = String.fromCharCode(header[156] || 48);
    off += 512;
    if (type === "0" || type === "\0") {
      files[name.replace(/^\.\//, "")] = buf.subarray(off, off + size);
    }
    off += Math.ceil(size / 512) * 512;
  }
  return files;
}

function unzipWithSystemTools(zipPath, dest) {
  // Windows 10 and later ship bsdtar as tar.exe, which reads zip files.
  let r = spawnSync("tar", ["-xf", zipPath, "-C", dest], { stdio: "inherit" });
  if (r.status === 0) return;
  r = spawnSync(
    "powershell",
    ["-NoProfile", "-Command", `Expand-Archive -Force -LiteralPath '${zipPath}' -DestinationPath '${dest}'`],
    { stdio: "inherit" }
  );
  if (r.status !== 0) throw new Error("could not unpack the zip archive (tried tar and Expand-Archive)");
}

async function install({ quiet = false } = {}) {
  if (process.env.MYRN_BINARY_PATH) return process.env.MYRN_BINARY_PATH;
  const bin = binaryPath();
  if (fs.existsSync(bin)) return bin;
  const t = target();
  const name = archiveName(t);
  const base = baseUrl();
  if (!quiet) console.error(`myrn: downloading ${name} (${pkg.version})`);
  const [archive, sums] = await Promise.all([
    download(`${base}/${name}`),
    download(`${base}/sha256sums.txt`).then((b) => b.toString("utf8")),
  ]);
  const got = crypto.createHash("sha256").update(archive).digest("hex");
  const want = expectedSum(sums, name);
  if (got !== want) throw new Error(`checksum mismatch for ${name}: got ${got}, expected ${want}`);

  fs.mkdirSync(CACHE, { recursive: true });
  if (name.endsWith(".zip")) {
    const tmp = path.join(CACHE, name);
    fs.writeFileSync(tmp, archive);
    unzipWithSystemTools(tmp, CACHE);
    fs.rmSync(tmp, { force: true });
  } else {
    const files = untar(zlib.gunzipSync(archive));
    const body = files["myrn"];
    if (!body) throw new Error(`${name} does not contain a myrn binary`);
    const tmp = `${bin}.${process.pid}.tmp`;
    fs.writeFileSync(tmp, body, { mode: 0o755 });
    fs.renameSync(tmp, bin);
  }
  if (!fs.existsSync(bin)) throw new Error(`${name} did not contain the expected binary`);
  if (!quiet) console.error(`myrn: installed ${bin}`);
  return bin;
}

module.exports = { install, target, archiveName, untar, expectedSum, binaryPath, TARGETS };

if (require.main === module) {
  install().catch((e) => {
    // Never fail `npm install`: bin/myrn.js retries on first use and explains.
    console.error(`myrn: ${e.message}`);
    console.error("myrn: the binary will be downloaded on first use instead.");
  });
}
