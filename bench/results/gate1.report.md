# Gate 1: paired benchmark

16 tasks, up to 2 runs per condition, model: default. 46 runs total, 0 excluded because the run itself failed.

## Gate 1: INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

| | Value |
|---|---|
| Paired runs | 23 |
| Token change per task (with − without) | -3.6k ± 23.7k (-2%) |
| Task success without → with | 96% → 96% |

Automatic injection becomes the default only on PASS. Values are means ± 95% confidence intervals.

## All tasks

| Metric | Without myrn | With myrn |
|---|---|---|
| Total tokens | 209.2k ± 85.9k | 205.6k ± 75.3k |
| Output tokens | 2.7k ± 0.7k | 2.5k ± 0.6k |
| Cost (USD) | 0.1888 ± 0.0549 | 0.1901 ± 0.0493 |
| Reads and searches before first correct edit | 2.5 ± 0.6 | 2.0 ± 0.8 |
| Reads and searches, whole session | 5.0 ± 1.1 | 4.4 ± 1.1 |
| Time to passing tests (s) | 46.6 ± 11.9 | 39.5 ± 8.2 |
| Turns | 9.0 ± 2.8 | 8.5 ± 2.6 |
| Task success | 96% | 96% |
| Timed out | 0 | 0 |

## Injection and hooks

| | Value |
|---|---|
| Items injected per run | 39.0 |
| Injection use (items whose file the agent then opened) | 20% |
| myrn tool calls per run | 0.0 |
| Prompt hook latency | median 26 ms, max 44 ms |

## Per task

| Task | Kind | Success without / with | Tokens without | Tokens with | Change | Exploration without / with |
|---|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 100% / 100% | 137.2k ± 371.7k | 92.2k ± 157.4k | -33% | 2.0 / 1.0 |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 69.4k ± 231.4k | 101.1k ± 0.5k | +46% | 1.0 / 1.0 |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 207.2k ± 100.4k | 249.2k ± 99.2k | +20% | 3.5 / 3.0 |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 49.6k ± 2.9k | 56.5k ± 1.8k | +14% | 2.0 / 2.0 |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 43.3k ± 98.3k | 60.6k ± 0.7k | +40% | 1.5 / 1.0 |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 546.1k ± 954.7k | 523.8k ± 584.0k | -4% | 3.0 / 5.0 |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 429.8k ± 203.0k | 382.2k ± 27.5k | -11% | 2.0 / 1.0 |
| `remeda/feature/take-every` | feature | 100% / 100% | 762.8k | 655.2k | -14% | 5.0 / 5.0 |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 83.2k | 77.3k | -7% | 2.0 / 2.0 |
| `remeda/question/purry` | question | 100% / 100% | 153.0k | 134.9k | -12% | 5.0 / 4.0 |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 201.4k | 217.0k | +8% | 9.0 / 2.0 |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 217.6k | 200.1k | -8% | 3.0 / 6.0 |
| `ripgrep/feature/glob-is-literal` | feature | 0% / 0% | 163.9k | 184.7k | +13% | 2.0 / 3.0 |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 117.2k | 167.8k | +43% | 2.0 / 1.0 |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 95.6k | 120.4k | +26% | 5.0 / 4.0 |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 50.5k | 39.6k | -22% | 2.0 / 1.0 |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user settings and MCP servers off (`--setting-sources project,local --strict-mcp-config`). *With myrn*: `myrn init`, `myrn connect` hooks, `[context] inject = true`, and myrn's MCP server. *Without*: neither. Total tokens are input + cache creation + cache reads + output from the session result, so injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
