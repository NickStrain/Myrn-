---
id: env-cmd
type: summary
status: unverified
title: Env::cmd
source: myrn summarize
about: sym:crates/myrn/tests/connect.rs#Env::cmd
anchors:
- sym:crates/myrn/tests/connect.rs#Env::cmd@ae0ccb282ed05437
map_version: '2'
---
# Env::cmd

Builds an unspawned Command for the myrn test binary with the given args, cwd set to the repo, MYRN_HOME set, and MYRN_NO_AGENT_CLI=1.

## Behaviour

- args ['init'] -> a command equal to running myrn init in the repo
- stdin is Stdio::null by default; hook() overrides it with a pipe
- cwd and stdin can still be changed by the caller before running

## Terms

spawn, process builder, command builder, environment variables, agent CLI, stdin null

## Connections

- **Location:** `crates/myrn/tests/connect.rs:46-54`
- **Signature:** `fn cmd(&self, args: &[&str]) -> Command`
- **In:** [[file-crates-myrn-tests-connect-rs]]
- **Called by:** [[env-myrn]], [[env-hook]]
