---
id: connect-run
type: summary
status: unverified
title: connect::run
source: myrn summarize
about: sym:crates/myrn/src/cmd/connect.rs#cmd::connect::run
anchors:
- sym:crates/myrn/src/cmd/connect.rs#cmd::connect::run@6c5901b5deaa8834
---
# connect::run

Opens the engine at the current directory and chooses adapters. In hooks mode without prior capture_consent it prints the disclosure and asks unless yes is set; declining returns FAILURE with nothing written. Consent sets capture_consent and consent_time. It then installs each adapter, prints changes and manual steps, merges created paths with prior ones (Vec::dedup removes only consecutive duplicates), and saves AgentState once at the end. If an install fails mid-loop, state is not saved, so files already written by earlier adapters go unrecorded.

## Connections

- **Location:** `crates/myrn/src/cmd/connect.rs:78-140`
- **Signature:** `pub fn run(ctx: &Ctx, args: Args) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-connect-rs]]
- **Calls:** [[mode-as-str]], [[engine-open]], [[engine-root]], [[engine-state]], [[engine-save-state]], [[connect-install-ctx]], [[connect-choose]], [[connect-ask]], [[connect-now]]
- **Called by:** [[main-main]]
