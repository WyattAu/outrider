# outrider

The rider sent ahead of the party: read-only internet access for AI agents
across login-walled platforms, one CLI, no API metering.

outrider is a fork of [Agent-Reach](https://github.com/Panniantong/Agent-Reach)
(MIT), seeded from upstream commit `14c213405669` (see
[docs/UPSTREAM.md](docs/UPSTREAM.md)).

## Why this fork exists

Upstream's core idea is right: a capability layer that installs, health-checks,
and routes per-platform readers, with a SKILL.md contract instead of a wrapper.
This fork keeps the architecture and changes the emphasis:

1. **Western platforms first-class.** Reddit, YouTube, GitHub, and the open web
   get the depth upstream gives Bilibili and Xiaohongshu.
2. **Rust hot path.** Channel fetches move to a static Rust core (crawlkit,
   fetch-kit, breaker, ratelimit from the WyattAu estate); the Python skill
   layer stays as the agent-facing contract.
3. **Consent-cleaner auth.** Cookie reuse from your browser is documented,
   opt-in per channel, and auditable. No silent session copying.
4. **Machine-readable doctor.** `doctor --json` with per-channel, per-backend
   health, exit codes CI can gate on.
5. **Estate channel.** The WyattAu aileron browser (Servo, keyboard-driven)
   becomes a first-class channel for JS-heavy pages.

## Status

Seeded, upstream-buildable. See [docs/ROADMAP.md](docs/ROADMAP.md).

## License

MIT, preserved in [LICENSE](LICENSE). Upstream copyright Agent Eyes and
contributors; this fork's changes are (c) 2026 WyattAu, same license.
