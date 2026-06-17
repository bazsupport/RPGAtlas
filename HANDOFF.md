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

### Done (on `feature/gamepad-support`, committed, all headless tests green — Phases 1–5 baz-validated in-browser; not pushed)

- **Phase 1 — schema/defaults** (`js/data.js`): `RA.PAD_BUTTONS`, `RA.INPUT_ACTIONS`,
  `RA.defaultInput()`, pure `RA.mergeInputBindings(projInput, override)` and
  `RA.inputConflict(bindings, device, code, exceptAction)`; `newProject`/`migrateProject` backfill.
- **Phase 2 — Input abstraction** (`js/runtime/input.js`, classic `createInputSystem(deps)`):
  device slots (keyboard + gamepad by index), per-frame `poll()` that owns the frame boundary and
  runs **before** `update()`'s early-return, edge detection, routing precedence
  **capture > menu (UIStack) > map edge**. Engine refactored to read named actions
  (`Input.dir/pressed/consume`) instead of the old `keyName`/loose trigger vars.
- **Phase 3 — gamepad poller:** W3C Standard Gamepad mapping, stick→synthetic names past deadzone,
  menu DAS/ARR auto-repeat, lazy connect/disconnect, `activeDevice()`. (START is **unbound by
  default**; `cancel` = `face_east` (B) opens/closes the menu — START used to share `cancel` but was
  split off during the Phase-4 polish.)
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
- **Phase 5 — editor grid + procedural glyphs + platform families** (`js/data.js`, `js/assets.js`,
  `js/runtime/input.js`, `js/engine.js`, `js/editor.js`, `css/editor.css`, `css/play.css`; commits
  `750f279` then `712209d`; **baz-validated in-browser**):
  - **Editor author-defaults grid** in its own new **"Controls"** Database tab (moved out of the
    crowded System tab; System now just points to it). Per-device add (keyboard key-capture / gamepad
    glyph picker), remove, critical-action guard, reset-to-defaults. **Replaces the old localStorage
    console snippet** for editing `proj.system.input`.
  - **Procedural input glyphs** (canvas-drawn, no art): `Assets.inputGlyphCanvas/DataUrl/Html`. Used
    in the grid, the gamepad picker, and the new **`\input[action]`** message text-code (engine pushes
    a `Plugins.textProcessors` entry; emits an `<img class="msg-icon">` so the typewriter counts it as
    one char; device chosen from `Input.activeDevice()` at message-open, with device fallback).
  - **Shared label/glyph subsystem moved into `RA`** (both shells load `data.js`; `input.js` and
    `assets.js` delegate **lazily** because they load before `data.js`): `RA.codeLabel` (verbose),
    `RA.glyphText` (compact draw token), `RA.glyphShape` (`face|dpad|stick|stick_click|pill`).
  - **Distinct shapes:** d-pad cross, analog-stick ring, and L3/R3 stick-click — so d-pad ≠ stick.
  - **Controller families = a DISPLAY layer (no schema change, bindings stay POSITIONAL):**
    `RA.padFamilyFromId(id)` + `Input.padFamily()` auto-detect Xbox / PlayStation / Switch from
    `Gamepad.id`; `RA.glyphText`/`RA.codeLabel`/the glyph renderer take a `family` (default `"xbox"`,
    so old call sites are unchanged); in-game rebinder text + `\input` glyphs follow the connected
    pad; the editor Controls tab has a **display-only** brand preview (sits in the Gamepad column
    header; **never persisted** to the project). **Relabel only — NO Nintendo confirm/cancel semantic
    swap.**
- **Tests:** `tests/input.test.js` (+glyphShape / per-family labels / `padFamilyFromId`),
  `tests/input-pad.test.js`, `tests/input-capture.test.js` (all pass alongside the pre-existing
  suites; `tests/modules.test.mjs` has a known unrelated `window is not defined` failure).

### Next

The input/gamepad feature (Phases 1–5) is functionally complete and **baz-validated in-browser**.
Nothing is pushed. Remaining, roughly in priority order — **confirm the next step with baz before
coding** (he drives):

1. **In-game rebinder glyph upgrade (optional polish):** `engine.js` `controlsDevice` /
   `actionBindings` (~line 1885 / 1904) still render bindings as **text** (`Input.label` /
   `Input.codeLabel`). They auto-relabel per controller family already, but could draw the same
   procedural glyphs as the editor grid via `Assets.inputGlyphHtml(device, code, Input.padFamily())`
   for visual consistency.
2. **Standalone-export rewrite + verify** (`js/editor/project-io.js` `buildStandaloneGame`): rewrite
   to the classic-script chain (the export is **already broken on upstream** independently of this
   feature — importmap/`type=module` wiring + missing renderer/plugins/quests/journal-view scripts),
   then **confirm an exported HTML boots, takes a gamepad, and renders glyphs + `\input[...]`** (glyphs
   are procedural so they inline for free — the breakage is the module wiring, not the glyphs).
3. **Deferred design — Nintendo confirm/cancel SEMANTIC swap:** families are relabel-only today
   (confirm is always the south button, shown as "B" on Switch). A true platform swap (confirm→east
   on Switch) needs per-family binding overrides; **design with baz before building.** Also out of
   scope by decision: paddles / extra buttons beyond the W3C-standard 16.
4. **One-time `localStorage` refresh** of baz's pre-existing "Atlas Quest" project may still be
   needed (its stored gamepad bindings predate `lstick_*`); snippet below. New projects are fine.
5. **Fix the pre-existing `tests/modules.test.mjs` failure** (`window is not defined` at
   `messages.js` loaded as a `data:` ESM in Node) — unrelated to this feature.
6. **Only after baz validates everything + explicitly approves:** rebase onto `upstream/main` (keep
   BOTH sides), confirm the maintainer wants it / isn't mid-flight, then the upstream PR. **Write/
   outward `gh` needs per-action approval, every time** ([[validate-before-pr]]).

   Atlas-Quest refresh snippet (browser console on `play.html`; type `allow pasting` first if warned):
   ```js
   const p = JSON.parse(localStorage.rpgatlas_project); delete p.system.input;
   localStorage.rpgatlas_project = JSON.stringify(p); localStorage.removeItem('rpgatlas_options');
   location.reload();
   ```

The original design plan `~/.claude/plans/swift-cooking-ladybug.md` (gamepad) and a later plan for the
glyph/family pass are **machine-local** (`~/.claude/plans/…`, absent on other machines) — their
substance is folded into this file. The standalone-export section of the gamepad plan has the exact
classic-script chain to mirror.

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
