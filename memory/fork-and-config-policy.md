---
name: fork-and-config-policy
description: Git remotes/roles, what stays out of git, the two-branch handoff model, and the gh approval policy.
metadata:
  type: project
---

**Remotes:** `origin` = `bazsupport/RPGAtlas` (baz's fork — push here); `upstream` =
`DriftwoodGaming/RPGAtlas` (pull/sync only — **never push**). Contributions go upstream as PRs from a
feature branch on the fork, after baz validates ([[validate-before-pr]]).

**Stays out of git / out of upstream PRs:**
- `CLAUDE.md` is git-excluded via `.git/info/exclude` (per-machine; doesn't travel).
- `.claude/settings.json` and `.claude/settings.local.json` are **gitignored** (per-machine config).
  `.claude/launch.json` IS tracked (shared http-server launch config).

**Two-branch cross-machine handoff (set up 2026-06-16):** to sync personal docs between machines
without leaking them into an upstream PR, they live on a dedicated **`workspace`** branch on `origin`
that is **never PR'd** — `CLAUDE.md`, `HANDOFF.md`, `memory/`. The PR branch
(`feature/gamepad-support`) carries **only code**, so there is nothing to strip before PRing.
Publish/update `workspace` via a throwaway `git worktree` so the main checkout never leaves the
feature branch. Read on a new machine with `git show origin/workspace:HANDOFF.md` etc.

**gh policy:** read-only `gh`/`git` is fine unprompted. **Write/outward `gh`**
(`pr create/comment/merge/close/edit`, anything that publishes to Driftwood) needs baz's explicit
per-action approval, **every time** — never on Claude's own initiative.
