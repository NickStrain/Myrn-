---
id: ui-open-browser
type: summary
status: unverified
title: ui::open_browser
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::open_browser
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::open_browser@a5cc450392eb5bcd
map_version: '2'
---
# ui::open_browser

Launches the platform URL opener (open on macOS, cmd /C start on Windows, xdg-open elsewhere) without waiting for it. Failure to spawn prints a hint to stderr.

## Behaviour

- spawn failure -> stderr line '(could not open a browser; open the URL above)', never an Err
- child process is not waited on, so its exit status is ignored
- Linux and other non-mac, non-Windows targets use xdg-open

## Terms

launch browser, default browser, spawn process, open url, xdg, browser launch

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:255-268`
- **Signature:** `fn open_browser(url: &str)`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Called by:** [[ui-run]]
