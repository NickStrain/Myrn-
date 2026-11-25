# Gate 1: paired benchmark

16 tasks, up to 2 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 64 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 32 | -10.7k ± 37.5k (-5%) | 17 of 32 | 0.860 | 100% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 32 | -10.7k ± 37.5k (-5%) | 17 of 32 | 0.860 | 100% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 214.2k ± 52.8k | 203.5k ± 61.6k |
| Total tokens, recording included | 214.2k ± 52.8k | 203.5k ± 61.6k |
| Output tokens | 4.0k ± 0.8k | 4.3k ± 1.1k |
| Cost (USD), recording included | 0.0067 ± 0.0014 | 0.0065 ± 0.0016 |
| Reads and searches before first correct edit | 5.2 ± 1.0 | 4.1 ± 1.1 |
| Reads and searches, whole task | 6.6 ± 1.3 | 5.6 ± 1.7 |
| Time to passing tests (s) | 31.0 ± 6.0 | 36.0 ± 8.7 |
| Turns | 9.0 ± 1.7 | 8.6 ± 2.3 |
| Task success | 100% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 4.8 |
| Injection use (items whose file the agent opened) | 48% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 32 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 38 ms, max 65 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 100% / 100% | 168.3k ± 167.5k | 189.0k ± 239.5k | +12% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 97.9k ± 66.3k | 93.3k ± 403.8k | -5% |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 273.8k ± 336.5k | 397.5k ± 112.7k | +45% |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 54.7k ± 8.2k | 19.6k ± 0.3k | -64% |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 39.3k ± 0.9k | 23.5k ± 13.7k | -40% |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 151.3k ± 638.1k | 96.8k ± 360.5k | -36% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 158.0k ± 304.7k | 154.5k ± 181.6k | -2% |
| `remeda/feature/take-every` | feature | 100% / 100% | 330.0k ± 1305.4k | 466.1k ± 1835.6k | +41% |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 143.6k ± 161.8k | 45.9k ± 8.9k | -68% |
| `remeda/question/purry` | question | 100% / 100% | 128.7k ± 151.5k | 66.1k ± 5.7k | -49% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 407.4k ± 8.9k | 434.1k ± 72.1k | +7% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 529.3k ± 519.8k | 456.2k ± 1330.1k | -14% |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 434.6k ± 927.5k | 417.3k ± 462.5k | -4% |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 264.4k ± 585.4k | 216.7k ± 451.9k | -18% |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 190.7k ± 65.1k | 116.7k ± 212.3k | -39% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 54.7k ± 3.9k | 62.6k ± 0.0k | +15% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.
