# outrider ROADMAP

## R0: Baseline honesty (this fork, week one)

- [ ] Run upstream's own install + doctor on CachyOS; record what breaks in
      docs/BASELINE.md with dates and commands
- [ ] CI: lint + install-path smoke test on the shared engineering-standards
      matrix
- [ ] SECURITY.md: cookie-handling policy stated as contract, not convention

## R1: Consent-clean auth + machine-readable doctor

- [ ] Cookie reuse becomes opt-in per channel with a written summary of what
      is copied, where it lives, and how to revoke
- [ ] `doctor --json`: per-channel, per-backend health, backend chosen,
      version, last-success; non-zero exit on degraded required channels
- [ ] Doctor output feeds a TUI status page (estate plychart)

## R2: Western platforms first-class

- [ ] Reddit: search/thread/user via public API + fallback backends, rate
      limits surfaced in doctor, not discovered as 429s
- [ ] YouTube: transcript + metadata without key; voice-to-text behind the
      existing Groq/OpenAI option, plus local whisper option
- [ ] GitHub: keep gh-cli path; add issue/PR timeline depth upstream lacks
- [ ] Open web: Jina reader stays default; add a readability fallback so
      reader outages degrade instead of failing

## R3: Rust hot path

- [ ] Channel fetch loop in Rust (crawlkit engine + fetch-kit + breaker +
      ratelimit), Python CLI shells down to the skill contract
- [ ] Single static binary distribution; Python stays for agent skill glue
- [ ] Per-channel latency + success metrics exported (estate otelkit)

## R4: Estate channel + depth

- [ ] aileron channel: Servo-driven, keyboard-driven browsing for JS-heavy
      pages, sandboxed profile, screenshots on demand
- [ ] Structured extraction profiles (search → normalized JSON per platform)
- [ ] Read-only invariant enforced by construction: no write verbs in the
      tool registry, asserted by a CI gate
