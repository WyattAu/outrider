# Consent model: logins, cookies, and sessions

Contract, not convention: this document states exactly what outrider does
with your platform sessions, per channel. If the code ever diverges from
this page, the code is wrong — file an issue.

## The rule

**outrider never logs in for you and never asks for your password.** When a
channel's remaining task genuinely requires an authenticated session, the
flow is:

1. The agent's own browser session is tried first (read-only public paths
   need no login at all — see each channel's `public_access` field in
   `agent-reach doctor` output).
2. If authentication is unavoidable, outrider opens a dedicated browser
   window; **you** perform the login yourself, on the platform's own page.
3. The session state (cookies/localStorage) is stored locally, chmod 600,
   under the channel's private cache dir — never synced, never transmitted,
   never shared between channels.

## What is copied, and how to revoke

`agent-reach add <channel>` with a login-gated channel copies **only that
platform's cookies** from your default browser profile into the channel's
private store. It does not read other sites' cookies, passwords, or the
keyring.

To revoke: `agent-reach logout <channel>` deletes the stored session, or
delete the channel's cache directory (path printed by `agent-reach doctor`).
Logging out on the platform itself also terminates the copied session.

## Per-channel honesty table

| Channel | Public (no login) | Login-gated additions | Session reuse |
|---|---|---|---|
| github | repos, README, issues/PRs + comments | none | none needed |
| youtube | video meta, search, subtitles | transcribe (needs Groq/OpenAI key — not a login) | none |
| web | pages, search | none | none |
| v2ex | hot, node topics, threads, profiles | none | none |
| reddit | search, post text, top comments | subreddit listings depth | only if the task demands it |
| x/twitter | search summaries, some post text | search depth, profiles, threads | only if required |
| linkedin | public job listings | job search depth | only if required |
| bilibili | search, video meta, hot | subtitles (first part) | QR login, you scan |
| xiaohongshu | note IDs from discover page | note body, loaded comments | QR login, you scan |

Rows marked "only if required" mean: the dispatcher tries public paths
first and only invokes the authenticated backend when the task cannot be
completed otherwise — and says so in the result's `access` field.

## What outrider never does

- Never posts, comments, likes, messages, or performs any write action.
  The tool registry contains no write verbs (asserted by a CI gate).
- Never accepts passwords on the command line or in config files.
- Never uploads your cookies or session state anywhere.
- Never expands a channel's declared actions without a release note.
