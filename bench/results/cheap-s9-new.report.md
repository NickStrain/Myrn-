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
| Task tokens | 345.8k ± 61.1k |
| Total tokens, recording included | 345.8k ± 61.1k |
| Output tokens | 8.8k ± 1.5k |
| Cost (USD), recording included | 0.0129 ± 0.0018 |
| Reads and searches before first correct edit | 5.6 ± 1.1 |
| Reads and searches, whole task | 8.6 ± 1.6 |
| Time to passing tests (s) | 51.8 ± 9.9 |
| Turns | 14.7 ± 2.0 |
| Task success | 88% |
| Timed out | 0 |

## myrn sessions

| | myrn, inject on, + use-context hint |
|---|---|
| Items injected per run | 4.2 |
| Injection use (items whose file the agent opened) | 57% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 16 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 715 ms, max 1058 ms |

## Per task

| Task | Kind | Success hint | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|
| `more-itertools/feature/gen-extract` | feature | 100% | 445.2k | – |
| `more-itertools/feature/gen-loops` | feature | 100% | 412.0k | – |
| `more-itertools/feature/gen-multinomial` | feature | 100% | 392.0k | – |
| `more-itertools/feature/gen-outer-product` | feature | 100% | 547.6k | – |
| `more-itertools/feature/gen-subfactorial` | feature | 100% | 425.8k | – |
| `more-itertools/feature/gen-takewhile-inclusive` | feature | 100% | 349.3k | – |
| `more-itertools/feature/take-every` | feature | 100% | 261.4k | – |
| `remeda/feature/gen-nth-by` | feature | 100% | 347.8k | – |
| `remeda/feature/gen-only` | feature | 100% | 530.7k | – |
| `remeda/feature/gen-sorted-index` | feature | 100% | 162.8k | – |
| `remeda/feature/gen-split-at` | feature | 0% | 380.6k | – |
| `remeda/feature/gen-take-last` | feature | 0% | 273.0k | – |
| `remeda/feature/gen-uncapitalize` | feature | 100% | 179.7k | – |
| `remeda/feature/take-every` | feature | 100% | 300.4k | – |
| `ripgrep/feature/glob-is-literal` | feature | 100% | 329.5k | – |
| `ripgrep/feature/globset-first-match` | feature | 100% | 194.9k | – |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
