# Benchmark (Gate 1)

The paired benchmark from the design doc's "Evaluation" section: the same tasks
on the same repositories, run with and without myrn, using Claude Code in
headless mode. Automatic context injection becomes the default only if this
shows fewer total tokens and no drop in task success.

```sh
cargo build --release -p myrn -p myrn-bench   # checks need cargo, python3 and node 24 on PATH
B=target/release/myrn-bench

$B list                      # repositories and tasks
$B fetch                     # clone checkouts into bench/repos/checkouts (network)
$B validate                  # every task: check fails when broken, passes with the reference fix
$B run --runs 3 --dry-run    # show the plan
$B run --runs 3              # spends agent usage: tasks x runs x 2 sessions
$B report bench/runs/<dir>   # rebuild report.md from results.jsonl
```

`run` writes `results.jsonl` (one line per session), the raw `stream-json`
transcripts in `raw/`, and `report.md` with the Gate 1 verdict.

## What a run does

1. Clones the pinned commit into a scratch directory, applies the task's setup
   patch and folds it into HEAD. The remote, other branches, tags, reflog and
   unreachable objects are removed, so the unbroken code can't be recovered
   from Git. (The bug does appear in the HEAD commit's diff, as a real
   regression would.)
2. **Without myrn:** no hooks and no MCP servers.
   **With myrn:** `myrn init`, `myrn connect --agent claude --yes`,
   `[context] inject = true`, and myrn's MCP server via `--mcp-config`.
   Each run gets a fresh myrn data directory.
3. Runs `claude -p <prompt> --output-format stream-json --setting-sources project,local --strict-mcp-config`,
   so user-level settings, hooks and MCP servers don't leak into either side.
4. Scores it: bug fixes and features write hidden test files and run the
   task's check command; questions must mention every `answer_contains` term.

Conditions alternate which goes first on each run, so drift over time doesn't
favour one side. Runs are paired by task and run number; the gate uses the
mean paired token difference and its 95% confidence interval:

- **PASS**: interval entirely below zero, and task success not lower.
- **INCONCLUSIVE**: lower on average, but the interval includes zero. Run more pairs.
- **FAIL**: tokens not lower, or task success dropped.

## Metrics

| Metric | Source |
|---|---|
| Total tokens | Session result: input + cache creation + cache reads + output (injected context included) |
| Exploration before first correct edit | Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file in the task's `touches` |
| Time to passing tests | Wall-clock session time, for successful bug fixes and features |
| Task success | Hidden check, or required terms in the answer |
| Injection use | Injected items whose file the agent opened or that it named in its answer |
| Hook latency | myrn's own prompt-hook timings |

Repeated-mistake rate (`replay/` tasks) and stale-injection rate need the
learned records from Phases 2 and 3.

## Adding a task

One YAML file in `tasks/<repo>/<kind>/`. The id is `<repo>/<kind>/<name>`.

```yaml
id: ripgrep/bugfix/glob-multi-dot-extension
repo: ripgrep
kind: bugfix            # bugfix | feature | question | replay
prompt: |               # the symptom, never the location
  ...
setup:                  # bug fixes: the patch that breaks the code
  patch: |
    diff --git a/... b/...
solution:               # features: the reference implementation (bug fixes: reverse of setup)
  patch: |
    ...
check:
  files:                # hidden tests, written after the session
    crates/globset/tests/bench_check.rs: |
      ...
  run: cargo test -q -p globset
touches: [crates/globset/src/pathutil.rs]
answer_contains: []     # questions only
timeout_secs: 900
```

Make patches with `git diff` in a checkout of the pinned commit, then run
`myrn-bench validate --task <id>`.
