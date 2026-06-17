# HANDOFF — RPGAtlas (cross-machine pickup)

> This file + `memory/` live on the **`workspace`** branch of `origin` (the fork), **never** on a
> feature/PR branch — so they can't leak into an upstream PR. The actual game code is on
> `feature/gamepad-support`.

## 0. Pickup clause — do this FIRST (before writing any code)

1. **Read, in order:**
   - this `HANDOFF.md` (you're here)
   - `memory/MEMORY.md` (index) → then each `memory/*.md` it points to
   - `CLAUDE.md` — architecture/conventions. NOTE: on a feature-branch working tree `CLAUDE.md`
     is git-excluded and may be **absent**; on the `workspace` branch it's committed here. Read it
     via `git show origin/workspace:CLAUDE.md` if it's not on disk.
2. **Confirm the git state:** `git fetch origin`, then `git branch -a`. Code work happens on
   **`feature/gamepad-support`**. The `workspace` branch is docs/memory only.
3. **Summarize the current state back to baz** (what's done, what's next) and **confirm the next
   step with him BEFORE writing code.** Do not assume; he drives.

To get these docs onto a fresh machine:
```
git clone https://github.com/bazsupport/RPGAtlas.git   # or pull if already cloned
cd RPGAtlas
git fetch origin
git checkout feature/gamepad-support      # the code you'll work on
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

> **CLASSIC-SCRIPT CONTRACT (important):** `js/engine.js`, `js/runtime/messages.js`, and the new
> `js/runtime/input.js` are **classic scripts** that attach a `window.createX` global (no
> `import`/`export`) — **NOT ES modules**, despite what parts of CLAUDE.md's table imply. Only
> `js/editor.js` is a real ES module. New shared runtime code = a classic IIFE that attaches a
> global; it's loaded by a plain `<script>` in `play.html` (and inlined the same way in the
> standalone export). Cache-bust with the manual `?v=N` query on the `<script>`/`<link>` when you
> change a file.

Tests are plain Node scripts (`node:assert/strict`, run individually, e.g. `node tests/input.test.js`).
No aggregate runner. Several headless tests load the browser libs into a `vm` context.

## 3. Current effort — gamepad + input remapping

**Goal:** a full, do-it-right input system — a proper Input abstraction (named actions:
`up/down/left/right/ok/cancel/dash/attack`), separate keyboard vs gamepad bindings (gamepad uses
generic positional names: `face_south/east/west/north`, `dpad_*`, `lstick_*`, `bumper_l/r`,
`trigger_l/r`, `start`, `select`, `stick_l/r`), an in-game rebinder (per-device, conflict detection,
reset, persisted per-player), editor author-defaults, generic text prompts (no glyph art), grid
movement so the stick reads 4/8-way.

### Done (on `feature/gamepad-support`, committed, all headless tests green — pending browser validation)

- **Phase 1 — schema/defaults** (`js/data.js`): `RA.PAD_BUTTONS`, `RA.INPUT_ACTIONS`,
  `RA.defaultInput()`, pure `RA.mergeInputBindings(projInput, override)` and
  `RA.inputConflict(bindings, device, code, exceptAction)`; `newProject`/`migrateProject` backfill.
- **Phase 2 — Input abstraction** (`js/runtime/input.js`, classic `createInputSystem(deps)`):
  device slots (keyboard + gamepad by index), per-frame `poll()` that owns the frame boundary and
  runs **before** `update()`'s early-return, edge detection, routing precedence
  **capture > menu (UIStack) > map edge**. Engine refactored to read named actions
  (`Input.dir/pressed/consume`) instead of the old `keyName`/loose trigger vars.
- **Phase 3 — gamepad poller:** W3C Standard Gamepad mapping, stick→synthetic names past deadzone,
  menu DAS/ARR auto-repeat, lazy connect/disconnect, `activeDevice()`. (START shares the `cancel`
  action so it opens/closes the menu.)
- **Phase 4 + REWORK — options store + rebinder + Music** (`js/engine.js`, `css/play.css`):
  per-player options store `rpgatlas_<gameId>_options` (mirrors `saveKey()`), boot merge over
  author defaults, Music-toggle persistence. The **first Phase-4 pass was rejected on validation**
  and reworked into:
  - In-game **Options** menu (renamed from "Controls"): Keyboard / Gamepad / Music On·Off /
    Reset to Defaults / Back — on both the title and pause menus; the title's standalone "Music"
    line was removed (Music now lives only inside Options).
  - **Editable-list rebinder** (`controlsDevice` → `actionBindings`): each action shows ALL its
    bindings + "Add binding" + Back; pick an existing one → Replace / Remove; Add → capture →
    append. Conflict → centered Replace/Cancel (removes the code from the clashing action); dedupes
    within an action; removing the last leaves it unbound.
  - **Capture API** in input.js: `beginCapture(device, cb)` / `cancelCapture()` /
    `isCapturing()`, with **ignore-held-until-release** (the Confirm press that opened "rebind"
    can't bind itself), device isolation, Esc-to-cancel, and a seed so a still-held pad button can't
    auto-confirm the conflict dialog. Plus `label(device,action)` / `codeLabel(device,code)` text
    helpers.
  - **Left stick is now a real, editable binding:** the poller synthesizes `lstick_*` (the D-Pad
    still emits `dpad_*`); default directions bind **both** (`up:["dpad_up","lstick_up"]`, …), shown
    as "D-Pad Up / L-Stick Up".
  - **Centering fix:** Options/conflict/reset windows no longer use the bottom-right `choicewin`
    class — they're centered (new `.optionswin`, `css/play.css`).
- **Tests:** `tests/input.test.js`, `tests/input-pad.test.js`, `tests/input-capture.test.js`
  (all pass alongside the pre-existing suites).

### Next

1. **baz to validate the Phase-4 rework in the browser** (`play.html`, hard-reload): centered
   Options window; gamepad direction rows show "D-Pad Up / L-Stick Up" and the stick is
   add/replace/removable; add/replace/remove a binding; conflict→Replace; Reset to Defaults; Music
   toggle persists and shows once.
2. **One-time refresh of his existing "Atlas Quest" project** (its stored gamepad bindings predate
   `lstick_*`, so the stick would be dead until refreshed). In the browser console on `play.html`
   (type `allow pasting` first if warned):
   ```js
   const p = JSON.parse(localStorage.rpgatlas_project); delete p.system.input;
   localStorage.rpgatlas_project = JSON.stringify(p); localStorage.removeItem('rpgatlas_options');
   location.reload();
   ```
   (New projects are correct automatically; only his pre-existing local project needs this.)
3. **Phase 5** (not started): editor System-tab **author-defaults grid** (full binding arrays, no
   truncation); finish/extend `Input.label` prompts; **`js/editor/project-io.js` `buildStandaloneGame`
   export rewrite** to the classic-script chain (the standalone export is **already broken on
   upstream** independent of this feature — importmap/`type=module` wiring + missing
   renderer/plugins/quests/journal-view scripts) and **verify an exported HTML actually boots and
   takes a gamepad**; a `js/patch-notes.js` entry. Also repair the pre-existing
   `tests/modules.test.mjs` failure.
4. **Only after baz validates everything:** consider the upstream PR (confirm the maintainer wants
   it / isn't mid-flight first). Rebase onto `upstream/main`, keep both sides' work.

The full design lives in baz's local plan `~/.claude/plans/swift-cooking-ladybug.md` (machine-local;
**won't exist on the other machine** — its substance is folded into this file). The standalone-export
section there has the exact script chain to mirror.

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
  don't retroactively apply to an existing project (that's why #2 above is needed).

## 5. Open notes / gotchas

- **Two-branch model (this handoff's whole point):** code on `feature/gamepad-support`; personal
  docs (`CLAUDE.md`, `HANDOFF.md`, `memory/`) on `workspace`. **Nothing to strip before an upstream
  PR** because the docs were never on the feature branch. When you update the handoff/memory, commit
  it on `workspace` and push (don't merge into the feature branch).
- **`CLAUDE.md` is git-excluded on feature branches** via `.git/info/exclude` (per-machine; doesn't
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
  - `C:\Users\baz\Documents\RPGAtlas` — repo root (different home dir elsewhere).
  - `~/.claude/plans/swift-cooking-ladybug.md` — the design plan (machine-local; not in any repo).
  - `~/.claude/projects/c--Users-baz-Documents-RPGAtlas/…` — Claude's per-machine session/memory
    dir; its `memory/MEMORY.md` is now just a pointer to this `workspace` branch.
  - Temp recon/workflow output under `…/AppData/Local/Temp/claude/…` — ephemeral, ignore.
  - `RPGAtlas.exe` / `src-tauri/` desktop wrapper — Windows-specific build artifacts.

## 6. Repo / git policy (baz's personal fork rules)

- `origin` = `bazsupport/RPGAtlas` (push here); `upstream` = `DriftwoodGaming/RPGAtlas`
  (pull/sync only — **never push**).
- Read-only `gh`/`git` is fine unprompted; **write/outward `gh`** (`pr create/comment/merge/…`)
  needs baz's explicit per-action approval, every time.
- Branch features off an up-to-date `main` (`git fetch upstream && git merge upstream/main`); rebase
  onto `upstream/main` before a PR; keep BOTH sides' work on conflicts (never clobber Driftwood).
