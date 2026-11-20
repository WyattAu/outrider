# PATTERN_ADOPTION

Read-only internet access CLI for AI agents: Python skill shell over a
Rust fetch core (fork of Agent-Reach, upstream seed 14c213405669).

| Pattern | Status | Evidence |
|---|---|---|
| P51 | adopted | rust/outrider-fetch/src/main.rs:67 (estate CircuitBreaker wrapped around fetch retries via fetch-kit with_breaker) |
| T13 | adopted | rust/outrider-fetch/src/main.rs:3 (Rust fetch and extraction core; Python channels shell out to this binary) |
| P57 | adopted | tests/test_channel_contracts.py:4 (registry contract: every channel exposes a unique name, a level, and actions with callable backends) |
| P94 | partial | agent_reach/runtime/cli.py:255 (doctor reports per-channel health with no network and a strict exit gate for CI; no continuous heartbeat loop yet) |
| P23 | n/a | verified absent: no AbortScope usage; browser pipe fds are closed in try/finally (agent_reach/runtime/browser_watcher.py:54) |

The hermetic test fixture blocks live network and subprocess escapes
(tests/conftest.py:75) but maps to no catalog ID, so it carries no
row.
