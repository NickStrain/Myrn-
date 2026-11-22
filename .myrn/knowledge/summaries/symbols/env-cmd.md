---
id: env-cmd
type: summary
status: unverified
title: Env::cmd
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::cmd
anchors:
- sym:crates/myrn/tests/connect.rs#Env::cmd@ae0ccb282ed05437
---
# Env::cmd

Builds a Command for the myrn test binary with the given args, cwd set to the repo, MYRN_HOME set to the temp home, MYRN_NO_AGENT_CLI=1, and stdin set to null.

## Connections

- **Location:** `crates/myrn/tests/connect.rs:46-54`
- **Signature:** `fn cmd(&self, args: &[&str]) -> Command`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Called by:** [[env-myrn]], [[env-hook]]
