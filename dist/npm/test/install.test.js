// cd dist/npm && npm test
"use strict";

const test = require("node:test");
const assert = require("node:assert");
const zlib = require("zlib");
const { target, archiveName, untar, expectedSum } = require("../install.js");

test("every supported platform maps to a release archive", () => {
  assert.strictEqual(target("linux", "x64"), "x86_64-unknown-linux-musl");
  assert.strictEqual(target("darwin", "arm64"), "aarch64-apple-darwin");
  assert.strictEqual(archiveName(target("win32", "arm64")), "myrn-aarch64-pc-windows-msvc.zip");
  assert.strictEqual(archiveName(target("linux", "arm64")), "myrn-aarch64-unknown-linux-musl.tar.gz");
  assert.throws(() => target("freebsd", "x64"), /no prebuilt binary/);
});

test("checksums are looked up by file name", () => {
  const sums = "aa11  myrn-x.tar.gz\nBB22 *myrn-y.zip\n";
  assert.strictEqual(expectedSum(sums, "myrn-y.zip"), "bb22");
  assert.throws(() => expectedSum(sums, "myrn-z.zip"), /not listed/);
});

test("reads a ustar archive", () => {
  const header = (name, size) => {
    const h = Buffer.alloc(512);
    h.write(name, 0);
    h.write("0000755\0", 100);
    h.write(size.toString(8).padStart(11, "0") + "\0", 124);
    h.write("0", 156);
    h.write("ustar\0", 257);
    return h;
  };
  const body = Buffer.from("#!/bin/sh\necho hi\n");
  const pad = Buffer.alloc(512 - (body.length % 512));
  const tar = Buffer.concat([header("./myrn", body.length), body, pad, Buffer.alloc(1024)]);
  const files = untar(zlib.gunzipSync(zlib.gzipSync(tar)));
  assert.deepStrictEqual(Object.keys(files), ["myrn"]);
  assert.strictEqual(files.myrn.toString(), body.toString());
});
