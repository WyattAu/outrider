# outrider baseline: upstream build on CachyOS (wyattdeskacercachy)

All claims measured on this host. Date: 2026-10-31. Seed commit:
`14c213405669` (upstream v1.6.0). Every entry carries the command that
produced it.

## Install (from this repo, not PyPI)

```
uv tool install . --force        # isolated tool env; uninstall with: uv tool uninstall agent-reach
agent-reach version              # {"ok": true, "result": {"version": "1.6.0", "install_method": "uv"}}
```

Python: system 3.14 accepted (`requires-python = ">=3.10"`); uv resolved the
env without intervention. Side effects observed:

- bili-cli 0.6.2 installed into agent-reach's private scope
- yt-dlp 2026.8.19 + yt-dlp-ejs 0.8.0 installed
- skill written to `~/.agents/skills/agent-reach/SKILL.md` (tool-managed;
  the fleet skills parity check excludes it)

## Channel health (2026-10-31)

`agent-reach setup` then `agent-reach doctor`: 5 default channels present,
zero problems. Verification runs (`agent-reach check <channel>`, JSON):

| Channel | Action | Backend | Passed | Elapsed |
|---|---|---|---|---|
| github | search | gh | yes | 1438 ms |
| github | search | github-api | yes | 373 ms |
| github | read | gh | yes | 1515 ms |
| github | read | github-api | yes | 603 ms |
| web | search | exa | yes | 672 ms |
| web | read | jina | yes | 318 ms |
| web | read | exa | yes | 271 ms |
| youtube | read | yt-dlp | yes | 10503 ms |
| youtube | search | yt-dlp | yes | 3031 ms |
| youtube | subtitle | yt-dlp | yes | 4331 ms |

Not added (login/key-gated): twitter, reddit, facebook, instagram, xiaohongshu,
linkedin, boss, xueqiu, xiaoyuzhou. bilibili + v2ex installed but unverified
at baseline time.

## Where upstream is weakest (fork priorities, measured)

1. **No machine-readable channel health.** `doctor` prints a human block and a
   huge JSON blob under `capability_summary`, but `check` results and doctor
   status are separate surfaces; there is no single `doctor --json` exit code
   a CI gate can consume. (ROADMAP R1)
2. **Cookie reuse is implicit.** Setup's login model ("reuse authorized
   session, then ask user") never states where sessions are copied from or
   how to revoke. (ROADMAP R1)
3. **Chinese-platform depth vs western gaps.** LinkedIn/Boss/Reddit declare
   thin, unverified action sets; the verification model itself admits
   `reach_status: unverified` is sample-based, not tested. (ROADMAP R2)
4. **Python-only runtime.** Every hot path (fetch, parse, route) is Python
   process spawns. (ROADMAP R3)
