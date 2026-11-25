# Gate 1: paired benchmark

16 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 96 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 48 | -2.1k ± 30.7k (-1%) | 25 of 48 | 0.885 | 98% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 48 | -2.1k ± 30.7k (-1%) | 25 of 48 | 0.885 | 98% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 198.3k ± 41.2k | 196.2k ± 50.6k |
| Total tokens, recording included | 198.3k ± 41.2k | 196.2k ± 50.6k |
| Output tokens | 4.0k ± 0.7k | 4.1k ± 0.8k |
| Cost (USD), recording included | 0.0064 ± 0.0011 | 0.0063 ± 0.0014 |
| Reads and searches before first correct edit | 4.5 ± 0.7 | 4.0 ± 0.9 |
| Reads and searches, whole task | 6.1 ± 1.1 | 5.1 ± 1.2 |
| Time to passing tests (s) | 33.6 ± 6.8 | 35.3 ± 7.0 |
| Turns | 9.0 ± 1.6 | 8.1 ± 1.7 |
| Task success | 98% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 4.2 |
| Injection use (items whose file the agent opened) | 49% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 48 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 52 ms, max 78 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 67% / 100% | 130.4k ± 47.4k | 134.7k ± 70.3k | +3% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 58.2k ± 0.9k | 74.1k ± 61.8k | +27% |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 343.8k ± 59.9k | 412.0k ± 367.6k | +20% |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 54.8k ± 0.1k | 25.7k ± 27.7k | -53% |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 39.9k ± 3.0k | 48.4k ± 33.1k | +21% |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 156.8k ± 225.4k | 123.4k ± 116.9k | -21% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 145.1k ± 68.9k | 101.0k ± 37.0k | -30% |
| `remeda/feature/take-every` | feature | 100% / 100% | 340.5k ± 196.5k | 560.8k ± 58.5k | +65% |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 147.8k ± 93.1k | 55.8k ± 42.2k | -62% |
| `remeda/question/purry` | question | 100% / 100% | 95.1k ± 56.4k | 66.1k ± 0.3k | -30% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 383.1k ± 244.3k | 523.4k ± 157.4k | +37% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 307.3k ± 92.3k | 237.5k ± 88.7k | -23% |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 427.9k ± 297.4k | 326.5k ± 150.2k | -24% |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 355.2k ± 316.0k | 255.8k ± 200.6k | -28% |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 131.9k ± 134.0k | 134.0k ± 115.5k | +2% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 55.0k ± 1.6k | 60.2k ± 0.7k | +10% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
