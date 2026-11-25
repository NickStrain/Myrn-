# Gate 1: paired benchmark

16 tasks, up to 1 runs per condition, conditions: myrn, inject on, + use-context hint, model: haiku. 16 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): NO DATA

no complete without/with pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | myrn, inject on, + use-context hint |
|---|---|
| Task tokens | 437.2k ± 112.5k |
| Total tokens, recording included | 437.2k ± 112.5k |
| Output tokens | 9.7k ± 2.2k |
| Cost (USD), recording included | 0.0145 ± 0.0031 |
| Reads and searches before first correct edit | 5.2 ± 1.1 |
| Reads and searches, whole task | 9.2 ± 2.5 |
| Time to passing tests (s) | 63.2 ± 17.5 |
| Turns | 15.8 ± 2.9 |
| Task success | 88% |
| Timed out | 0 |

## myrn sessions

| | myrn, inject on, + use-context hint |
|---|---|
| Items injected per run | 2.4 |
| Injection use (items whose file the agent opened) | 58% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 16 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 530 ms, max 750 ms |

## Per task

| Task | Kind | Success hint | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|
| `more-itertools/feature/gen-extract` | feature | 100% | 426.8k | – |
| `more-itertools/feature/gen-loops` | feature | 100% | 209.0k | – |
| `more-itertools/feature/gen-multinomial` | feature | 100% | 311.4k | – |
| `more-itertools/feature/gen-outer-product` | feature | 100% | 849.3k | – |
| `more-itertools/feature/gen-subfactorial` | feature | 100% | 253.2k | – |
| `more-itertools/feature/gen-takewhile-inclusive` | feature | 100% | 628.4k | – |
| `more-itertools/feature/take-every` | feature | 100% | 385.3k | – |
| `remeda/feature/gen-nth-by` | feature | 100% | 906.1k | – |
| `remeda/feature/gen-only` | feature | 100% | 303.0k | – |
| `remeda/feature/gen-sorted-index` | feature | 100% | 179.9k | – |
| `remeda/feature/gen-split-at` | feature | 0% | 359.5k | – |
| `remeda/feature/gen-take-last` | feature | 0% | 384.7k | – |
| `remeda/feature/gen-uncapitalize` | feature | 100% | 309.9k | – |
| `remeda/feature/take-every` | feature | 100% | 576.4k | – |
| `ripgrep/feature/glob-is-literal` | feature | 100% | 522.6k | – |
| `ripgrep/feature/globset-first-match` | feature | 100% | 389.3k | – |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
