# Gate 1: paired benchmark

8 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 44 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 22 | -33.1k ± 34.5k (-17%) | 12 of 22 | 0.832 | 100% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 22 | -33.1k ± 34.5k (-17%) | 12 of 22 | 0.832 | 100% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 199.2k ± 54.3k | 166.1k ± 48.2k |
| Total tokens, recording included | 199.2k ± 54.3k | 166.1k ± 48.2k |
| Output tokens | 3.9k ± 0.9k | 3.5k ± 0.9k |
| Cost (USD), recording included | 0.0062 ± 0.0014 | 0.0053 ± 0.0012 |
| Reads and searches before first correct edit | 5.2 ± 1.1 | 2.9 ± 1.6 |
| Reads and searches, whole task | 7.0 ± 1.8 | 5.3 ± 1.6 |
| Time to passing tests (s) | 26.7 ± 6.3 | 25.3 ± 6.6 |
| Turns | 9.7 ± 2.2 | 7.8 ± 1.9 |
| Task success | 100% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 3.9 |
| Injection use (items whose file the agent opened) | 38% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 22 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 52 ms, max 71 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 100% / 100% | 139.1k ± 47.5k | 129.3k ± 117.1k | -7% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 79.9k ± 50.0k | 108.7k ± 60.5k | +36% |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 193.0k ± 166.6k | 130.1k ± 138.5k | -33% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 178.4k ± 37.9k | 117.4k ± 39.2k | -34% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 418.1k ± 148.2k | 388.1k ± 145.9k | -7% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 307.3k ± 232.4k | 216.1k ± 264.8k | -30% |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 164.3k ± 231.8k | 133.8k ± 173.5k | -19% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 53.5k ± 0.1k | 58.9k ± 1.8k | +10% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
