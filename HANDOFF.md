# HANDOFF — RPGAtlas (cross-machine pickup)

> This file + `memory/` live on the **`workspace`** branch of `origin` (the fork), **never** on a
> feature/PR branch — so they can't leak into an upstream PR. The game code is on **`main`**
> (kept in sync with `upstream/main`); branch new feature work off it per §6.

## 0. Pickup clause — do this FIRST (before writing any code)

1. **Read, in order:**
   - this `HANDOFF.md` (you're here)
   - `memory/MEMORY.md` (index) → then each `memory/*.md` it points to
   - `CLAUDE.md` — architecture/conventions. NOTE: on a code working tree `CLAUDE.md`
     is git-excluded and may be **absent**; on the `workspace` branch it's committed here. Read it
     via `git show origin/workspace:CLAUDE.md` if it's not on disk.
2. **Confirm the git state:** `git fetch origin upstream`, then `git branch -a`. Code work happens on
   **`main`** (kept in sync with `upstream/main`); branch each new feature off an up-to-date `main`
   (§6). The `workspace` branch is docs/memory only. **No active feature branch right now** — the
   tree is at a clean slate between features.
3. **Summarize the current state back to baz** (what's done, what's next) and **confirm the next
   step with him BEFORE writing code.** Do not assume; he drives.

To get these docs onto a fresh machine:
```
git clone https://github.com/bazsupport/RPGAtlas.git   # or pull if already cloned
cd RPGAtlas
git fetch origin upstream
git checkout main                         # the code you'll work on (synced to upstream/main)
git show origin/workspace:HANDOFF.md      # this file
git show origin/workspace:memory/MEMORY.md
git show origin/workspace:CLAUDE.md       # architecture; copy to ./CLAUDE.md if you want it on disk
# optional: have the docs as real files alongside the repo:
git worktree add ../RPGAtlas-workspace origin/workspace
```

## 1. Working agreement / tone (how baz and Claude collaborate)

- **baz validates everything himself, in the browser, before anything goes outward.** Hard rule:
  **never open or even draft a PR, and never push to `upstream`, until baz has validated and
  explicitly approved.** Work phase-by-phase; stop and hand back for his validation between phases.
  (Committing/pushing to *his own fork* `origin` for sync is fine when he asks — that's not a PR.)
- **He does the running/testing.** When he says "do NOT run build/test tooling," respect it — don't
  run `node`, servers, or the game; just make the changes and report precisely.
- **Tone:** direct, concise, technical. Give a recommendation, not a menu. Surface real
  trade-offs and disagree when warranted, but don't bikeshed. He's a capable solo dev — skip the
  hand-holding, keep the "why" tight.
- **When the UX/feel is the point, he'll eyeball it and give specific feedback** (e.g. "window's
  off-center," "the stick isn't shown"). Treat that as the spec; iterate on it, don't argue.
- **Surface contradictions** between what a doc says and what the code/state actually is, rather
  than silently picking a side (this is how the classic-script correction below got caught).

## 2. Project snapshot

RPGAtlas is a browser-based 2D RPG-making engine (editor + player), a **fork of
`DriftwoodGaming/RPGAtlas`**. Vanilla JS/HTML/CSS, **no package.json, no deps, no build step**;
all graphics procedural; GPL v3 (every JS file carries a GPL header). Two shells: `index.html` →
editor (`js/editor.js`), `play.html` → player (`js/engine.js`).

**Architecture/conventions: see `CLAUDE.md`** (committed on this `workspace` branch). One correction
that supersedes the prose there and is easy to get wrong:

> **CLASSIC-SCRIPT CONTRACT (important):** `js/engine.js`, `js/runtime/messages.js`, and
> `js/runtime/input.js` are **classic scripts** that attach a `window.createX` global (no
> `import`/`export`) — **NOT ES modules**, despite what parts of CLAUDE.md's table imply. Only
> `js/editor.js` is a real ES module. New shared runtime code = a classic IIFE that attaches a
> global; it's loaded by a plain `<script>` in `play.html` (and inlined the same way in the
> standalone export). Cache-bust with the manual `?v=N` query on the `<script>`/`<link>` when you
> change a file.

Tests are plain Node scripts (`node:assert/strict`, run individually, e.g. `node tests/input.test.js`).
No aggregate runner. Several headless tests load the browser libs into a `vm` context.

## 3. Status — keymap PR open, awaiting upstream review (2026-06-23)

**Active feature branch `feature/editor-keymap` (on `origin`), open as upstream PR #17.** Editor
keyboard-shortcut overhaul: `Tab`/`Shift+Tab` mode cycling, tools `Q W E R T Y`, layers `` ` `` `1 2 3 4`,
`F1` Database / `F2` HD-2D Preview / `F5` Playtest, and a `?` shortcuts overlay. baz validated it
in-browser and directed the PR himself. Design rationale (semantic vs positional keys; deliberately
keyless dialogs): [[editor-keymap-design]]. **Next: watch for upstream review/merge of #17; nothing
else queued — plan the next feature with baz.**

`main` is synced to `upstream/main` at `0e7e698` (merge of PR #16 "Add Event-mode right-click menu and
Quick Events") and pushed to `origin/main`. Headless tests still pass — ran `heights` / `textcodes` /
`traits` / `modules`; the keymap touches only editor UI, which the headless suite doesn't cover (so it
was verified manually in the browser).

**Branch hygiene snapshot (updated 2026-06-23):**
- **`feature/editor-keymap`** — current open branch (PR #17). On `origin`; **do not delete until #17 merges.**
- **`feature/event-context-menu`** — merged via **PR #16** into `main`; the `origin` branch is now
  redundant and safe to delete.
- **Branch hygiene snapshot (2026-06-20 cleanup):**
- **Deleted (confirmed merged into `main`):** `feature/gamepad-support`, `feature/event-window-redesign`,
  `feature/drag-reorder-commands` (the latter two are upstream PRs #9 and #4).
- **Deleted (superseded):** `fix/grid-movement-jitter` — baz reimplemented grid-movement smoothing
  himself (fixed-timestep + frame interpolation, now in `js/engine.js`); the PR for my version was
  declined.
- **Still on `origin`, NOT a feature to resume:** `origin/feature/options-menu` — now fully contained
  in `main` via merged **PR #15**, so it's redundant. Kept only because baz said "leave it for now";
  safe to delete whenever.
- **Never delete `origin/workspace`** — this docs/memory branch.

### Shipped — gamepad + input remapping (upstream PR #14, 2026-06-17; options-menu polish PR #15, 2026-06-18)

The full input system shipped. Full design record + the still-deferred bits live in
[[gamepad-input-effort]]; the one-line version: engine + menus consume **named actions** only
(`up/down/left/right/ok/cancel/dash/attack`), `js/runtime/input.js` (classic — see
[[classic-script-contract]]) owns keyboard + gamepad, gamepad bindings are **positional**
(`face_*`, `dpad_*`, `lstick_*`, …), author defaults in `proj.system.input`, per-player override at
`rpgatlas_<gameId>_options`. Editor **"Controls"** Database tab for author-defaults, in-game
**Options** rebinder, **procedural canvas glyphs**, the **`\input[action]`** message code, and
**controller-family relabel** (Xbox/PS/Switch, display-only — no Nintendo confirm/cancel semantic
swap).

### Still open / deferred — not blocking; confirm with baz before picking one up

1. **Standalone-export verify/rewrite** (`js/editor/project-io.js` `buildStandaloneGame`): the export
   still wires the entry point as `<script type="module">` (project-io.js ~line 134) and was flagged
   broken on upstream independently of the gamepad work — **not verified to boot.** If touched:
   confirm an exported HTML boots, takes a gamepad, and renders glyphs + `\input[...]` (glyphs are
   procedural so they inline for free — the risk is the module wiring, not the glyphs).
2. **Deferred design — Nintendo confirm/cancel SEMANTIC swap:** families are relabel-only today
   (confirm is always the south button, shown as "B" on Switch). A true platform swap (confirm→east
   on Switch) needs per-family binding overrides; **design with baz before building.** Out of scope
   by decision: paddles / extra buttons beyond the W3C-standard 16.
3. **Deferred follow-on:** [[custom-input-actions]] — author-defined custom input actions (design pinned).
4. **Per-machine, browser-only:** baz's pre-existing "Atlas Quest" project may need a one-time
   `localStorage` refresh if its stored gamepad bindings predate `lstick_*` (new projects are fine):
   ```js
   // browser console on play.html; type `allow pasting` first if warned
   const p = JSON.parse(localStorage.rpgatlas_project); delete p.system.input;
   localStorage.rpgatlas_project = JSON.stringify(p); localStorage.removeItem('rpgatlas_options');
   location.reload();
   ```

## 4. Conventions / patterns to follow

- camelCase functions/locals; UPPERCASE_SNAKE consts; PascalCase/UPPER shared globals
  (`Assets`, `RA`, `GLRender`, `Music`, `Sfx`). DOM helpers `el(tag,cls,html)` and hyperscript `h`.
- Keep it dependency-free / build-free. New shared runtime code = classic IIFE global; new
  editor/runtime entry code uses the existing module style for that file.
- Menus go through `showList(items, opts)` (returns a Promise → index or -1; supports
  `it.html`/`it.label`/`it.disabled`/`it.help`, `opts.className`, `opts.start`, `opts.cancellable`).
  Input routes named actions to `UIStack[top].onKey(action)`.
- Bump `?v=N` on the changed `<script>`/`<link>` in `play.html`/`index.html`.
- Bindings model: `{ keyboard:{action:[e.code…]}, gamepad:{action:[padName…]}, stickDeadzone }`.
  Project (author) defaults live in `proj.system.input`; the player override is a localStorage
  options blob merged on top at boot. **Migration keeps stored values** — engine-default changes
  don't retroactively apply to an existing project (that's why the Atlas-Quest refresh in §3 exists).

## 5. Open notes / gotchas

- **Two-branch model (this handoff's whole point):** game code on `main` / feature branches; personal
  docs (`CLAUDE.md`, `HANDOFF.md`, `memory/`) on `workspace`. **Nothing to strip before an upstream
  PR** because the docs are never on a code branch. When you update the handoff/memory, commit it on
  `workspace` and push (don't merge it into a code/feature branch).
- **`CLAUDE.md` is git-excluded on code branches** via `.git/info/exclude` (per-machine; doesn't
  travel). On a new machine, materialize it from `git show origin/workspace:CLAUDE.md > CLAUDE.md`
  if you want Claude to auto-read it; it stays untracked there.
- **`.claude/settings.json` / `settings.local.json` are gitignored** (per-machine config). Each
  machine will have its own; don't commit them. `.claude/launch.json` IS tracked (shared
  http-server launch config).
- **localStorage is per-origin:** `file://` vs `http://localhost:8080` are separate stores — always
  serve over HTTP. Project key `rpgatlas_project`; options key `rpgatlas_<gameId>_options`
  (`rpgatlas_options` when no game id).
- **Console self-XSS guard:** pasting into DevTools needs you to type `allow pasting` once per tab.
- **Machine-specific absolute paths** (will differ / be absent on the other machine):
  - `C:\Users\baz\Documents\RPGAtlas` — repo root (different home dir elsewhere);
    `C:\Users\baz\Documents\RPGAtlas-workspace` — the `workspace` worktree (where these docs get edited).
  - `~/.claude/plans/…` — past design plans (machine-local; not in any repo; their substance is folded
    into memory/this file).
  - `~/.claude/projects/c--Users-baz-Documents-RPGAtlas/…` — Claude's per-machine session/memory
    dir; its `memory/MEMORY.md` is just a pointer to this `workspace` branch.
  - Temp recon/workflow output under `…/AppData/Local/Temp/claude/…` — ephemeral, ignore.
  - `RPGAtlas.exe` / `src-tauri/` desktop wrapper — Windows-specific build artifacts.

## 6. Repo / git policy (baz's personal fork rules)

- `origin` = `bazsupport/RPGAtlas` (push here); `upstream` = `DriftwoodGaming/RPGAtlas`
  (pull/sync only — **never push**).
- Read-only `gh`/`git` is fine unprompted; **write/outward `gh`** (`pr create/comment/merge/…`)
  needs baz's explicit per-action approval, every time.
- Branch features off an up-to-date `main` (`git fetch upstream && git merge upstream/main`); rebase
  onto `upstream/main` before a PR; keep BOTH sides' work on conflicts (never clobber Driftwood).
