# outrider ROADMAP

## R0: Baseline honesty (this fork, week one)

- [ ] Run upstream's own install + doctor on CachyOS; record what breaks in
      docs/BASELINE.md with dates and commands
- [ ] CI: lint + install-path smoke test on the shared engineering-standards
      matrix
- [ ] SECURITY.md: cookie-handling policy stated as contract, not convention

## R1: Consent-clean auth + machine-readable doctor

- [x] Cookie reuse contract written: docs/CONSENT.md (what is copied per
      channel, where it lives, how to revoke; never-posts / never-passwords
      guarantees stated as contract)
- [x] `doctor --strict` (2026-11-01): exit 1 when any default-level channel
      has problems or regressed — the CI-consumable gate. JSON health was
      already machine-readable; what was missing was the exit contract.
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

- [x] `outrider-fetch` (rust/outrider-fetch): resilient fetch + text
      extraction on fetch-kit 0.2.2, live-verified; wired as the web
      channel's first read backend (jina/exa fallback intact). Measured
      2026-10-31: read/outrider 670 ms vs read/jina 1713 ms on example.com;
      `check web` passes all backends.
- [ ] Channel fetch loop fully in Rust (crawlkit/scanner-class engines stay
      out of scope by design; per-channel parity vs Python backends)
- [ ] Single static binary distribution; Python stays for agent skill glue
- [ ] Per-channel latency + success metrics exported (estate otelkit)

## R4: Estate channel + depth

- [ ] aileron channel: Servo-driven, keyboard-driven browsing for JS-heavy
      pages, sandboxed profile, screenshots on demand
- [ ] Structured extraction profiles (search → normalized JSON per platform)
- [ ] Read-only invariant enforced by construction: no write verbs in the
      tool registry, asserted by a CI gate
