# Gate 1: paired benchmark

4 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 24 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 12 | -14.5k ± 21.6k (-16%) | 8 of 12 | 0.388 | 100% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 12 | -14.5k ± 21.6k (-16%) | 8 of 12 | 0.388 | 100% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 93.2k ± 29.7k | 78.7k ± 20.9k |
| Total tokens, recording included | 93.2k ± 29.7k | 78.7k ± 20.9k |
| Output tokens | 2.3k ± 0.6k | 2.5k ± 1.0k |
| Cost (USD), recording included | 0.0032 ± 0.0007 | 0.0031 ± 0.0009 |
| Reads and searches before first correct edit | 2.7 ± 0.9 | 1.0 ± 0.0 |
| Reads and searches, whole task | 3.4 ± 1.4 | 2.2 ± 0.8 |
| Time to passing tests (s) | 18.4 ± 5.3 | 21.0 ± 7.8 |
| Turns | 5.3 ± 1.8 | 4.2 ± 1.1 |
| Task success | 100% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 4.0 |
| Injection use (items whose file the agent opened) | 50% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 12 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 47 ms, max 62 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 101.1k ± 99.8k | 79.8k ± 50.5k | -21% |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 47.2k ± 33.6k | 56.6k ± 31.4k | +20% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 149.9k ± 86.2k | 118.7k ± 101.9k | -21% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 74.5k ± 51.3k | 59.6k ± 2.3k | -20% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
