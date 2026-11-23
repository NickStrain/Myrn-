---
id: ui-token
type: summary
status: unverified
title: ui::token
source: myrn summarize
about: sym:crates/myrn/src/cmd/ui.rs#cmd::ui::token
anchors:
- sym:crates/myrn/src/cmd/ui.rs#cmd::ui::token@14dde1db31af242b
map_version: '2'
---
# ui::token

Returns a 32-character hex string for this run, built by hashing std RandomState output, the process id and the current time in nanoseconds.

## Behaviour

- each call returns a different 32-hex-character string
- clock before 1970 -> time term is 0 and a token is still returned
- no crypto library is used; the value is only compared for equality with the request token

## Terms

access token, secret, nonce, random string, session key, auth token, run id

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs:236-253`
- **Signature:** `fn token() -> String`
- **In:** [[file-crates-myrn-src-cmd-ui-rs]]
- **Called by:** [[ui-run]]
- **Tests:** `tokens_differ`
