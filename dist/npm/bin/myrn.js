#!/usr/bin/env node
// Runs the native myrn binary with this process's arguments and exit code.
// Agent hooks and the MCP server call the binary itself (`myrn connect`
// records its real path), so Node is only in the way for commands you type.
"use strict";

const { spawnSync } = require("child_process");
const { install } = require("../install.js");

install({ quiet: false })
  .then((bin) => {
    const r = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
    if (r.error) {
      console.error(`myrn: could not run ${bin}: ${r.error.message}`);
      process.exit(1);
    }
    process.exit(r.status === null ? 1 : r.status);
  })
  .catch((e) => {
    console.error(`myrn: ${e.message}`);
    process.exit(1);
  });
