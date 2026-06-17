# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Handoff & Memory (read first)

- **Current status (2026-06-16):** mid-feature — full **gamepad + input-remapping** system. Code lives on branch `feature/gamepad-support` (Phases 1–4 + a Phase-4 UX rework done; all headless tests pass; **pending baz's browser validation**; Phase 5 = editor defaults grid + standalone-export rewrite + patch notes still to do).
- **Cross-machine handoff + project memory live on the `workspace` branch on `origin` (NOT in the working tree, NOT on the PR branch — keeps them out of upstream PRs).** At session start, fetch and read them:
  - `git fetch origin`
  - `git show origin/workspace:HANDOFF.md` — full pickup doc (working agreement, what's done/next, gotchas)
  - `git show origin/workspace:memory/MEMORY.md` — memory index; then `git show origin/workspace:memory/<file>.md` per entry
  - Or check it out as files: `git worktree add ../RPGAtlas-workspace origin/workspace`
- **Record new durable facts on the `workspace` branch under `memory/`** (one file per fact, frontmatter format) — **not** in `~/.claude` (that's per-machine and is now just a pointer here).

## What this is

RPGAtlas is a browser-based 2D RPG-making engine (editor + player) in the spirit of classic RPG Makers. It has **no package.json, no dependencies, and no build step** — all code is vanilla JS/HTML/CSS and all graphics are generated procedurally in JavaScript. License is GPL v3; every JS module carries a GPL header. (Legacy "Driftwood Engine" projects auto-migrate to RPGAtlas.)

## Running

- **Edit/play locally:** serve the repo root over HTTP and open `index.html` (editor) or `play.html` (player). Any static server works:
  - `python -m http.server 8080` then open `http://localhost:8080/`
  - or `npx --yes http-server -p 8080 -c-1` (matches `.claude/launch.json`)
  - or run the built `RPGAtlas.exe` (C# launcher that serves the folder and opens the browser).
- Do **not** open the HTML files via `file://` — ES module imports and `fetch` of project assets require an HTTP origin.

## Tests

No test framework. Tests are plain Node scripts using `node:assert/strict`; run each directly:

```
node tests/heights.test.js
node tests/textcodes.test.js
node tests/traits.test.js
node tests/modules.test.mjs   # ES-module test for project-io + messages
```

There is no aggregate runner — run a single file to test one area. `tests/modules.test.mjs` loads the browser modules by reading the source and importing it as a `data:` URL, and stubs `fetch`/`FileReader`, so it can exercise editor/runtime code outside a browser. Tests passing print `"<area> tests passed."`.

## Building the Windows launchers (only when changing the C# launcher/export)

C# sources and PowerShell build scripts live in `tools/`. They use the in-box .NET Framework `csc.exe`:

```
pwsh -File tools/build-engine-launcher.ps1   # -> RPGAtlas.exe (editor launcher, from RPGAtlasEngine.cs)
pwsh -File tools/build-launcher.ps1          # -> bin/RPGAtlasLauncher.exe (game export stub, from RPGAtlasLauncher.cs)
pwsh -File tools/update-assets.ps1           # -> img/assets.json (asset manifest for hosts without dir listings)
```

## Architecture

Two HTML shells load the same shared libraries, then a module entry point:

- `index.html` -> editor entry `js/editor.js`
- `play.html` -> runtime entry `js/engine.js`

**Script-load contract (important).** In each shell, the shared singletons load first as *classic scripts* that each expose an IIFE global, those globals are bundled into `window.RPGAtlasDeps`, and only then does the *ES-module* entry point load and consume them:

```html
<script src="js/assets.js"></script>   <!-- Assets -->
<script src="js/gl.js"></script>        <!-- GLRender -->
<script src="js/sfx.js"></script>       <!-- Sfx, Music -->
<script src="js/plugins.js"></script>   <!-- AtlasBuiltins -->
<script src="js/data.js"></script>      <!-- RA, DataDefaults -->
<script>window.RPGAtlasDeps = { Assets, AtlasBuiltins, DataDefaults, GLRender, Music, RA, Sfx };</script>
<script type="module" src="js/editor.js"></script>
```
So: `assets.js`, `gl.js`, `sfx.js`, `plugins.js`, `data.js` are **classic scripts** (no imports/exports — they attach a global). `js/editor.js`, `js/engine.js`, `js/editor/project-io.js`, `js/runtime/messages.js` are **ES modules** (use `import`/`export`). When editing a shared lib, keep it a global-attaching IIFE; when editing an entry/module, use ES imports. The cache-busting `?v=N` query on each `<script>` is bumped manually when a file changes.

### Modules

| File | Type | Responsibility |
|------|------|------|
| `js/data.js` | global `RA`, `DataDefaults` | Project schema + defaults (`newProject`, `newMap`, `newEvent`) and the game-rules namespace `RA` (`byId`, `typeList`, `traitsOf`, `traitRate/traitSum`, `canEquip`, `migrateProject`). Combat math and the Database "Types" system live here. |
| `js/assets.js` | global `Assets` | Procedural generation of tiles/characters/battlers/icons/faces; discovery and loading of custom images from `img/`; canvas caching for GPU reuse. |
| `js/gl.js` | global `GLRender` | WebGL2 HD-2D renderer. `setMap(lower, upper, map)` builds chunked 3D geometry (ground tiles extruded by height); `renderFrame(...)` draws terrain + upright sprite billboards + point lights + optional post-FX (bloom / DoF / fog). Silently falls back to Canvas 2D if WebGL2 is unavailable. |
| `js/sfx.js` | globals `Sfx`, `Music` | Procedural sound effects and music. |
| `js/plugins.js` | global `AtlasBuiltins` | Plugin system + built-ins (Atlas_Core, Atlas_TextCodes, Atlas_Transitions, Atlas_Weather). Plugins expose hooks (boot / map-load / per-frame) and text processors. |
| `js/editor.js` | module | Editor app: map/event/passability/height tools, Database tabs, undo/redo, and the live HD-2D preview panel (debounced rebuild via `GLRender.setMap`). |
| `js/engine.js` | module | Game interpreter/runtime: scene flow (boot→title→map→battle), central game state `G`, event command interpreter, turn-based battle, 60fps loop rendering via Canvas 2D or `GLRender.renderFrame`. Imports `createMessageSystem`. |
| `js/editor/project-io.js` | module | Persistence + export: `loadStoredProject`/`saveProject` (localStorage key `rpgatlas_project`, migrating from legacy `driftwood_project`), `exportProjectFile`, `buildStandaloneGame`, `exportStandaloneHtml`, `exportWindowsExecutable`. |
| `js/runtime/messages.js` | module | `createMessageSystem(deps)` -> message window API: `convertText` (control codes `\v[n]` var, `\n[n]` actor name, `\g` gold), `richText` (escape + plugin text processors), typewriter reveal, `showMessage` with face/icon. |

### Data model

A project is a single JSON document (maps, actors, classes, skills, items, weapons, armors, enemies, troops, states, system, plugins). It is mutated in-place by the editor and saved to `localStorage`. The **Database "Types"** live under `proj.system.types`: `elements` and `skillTypes` use stable **string keys** (referenced by skills/traits/combat formulas); `weaponTypes`/`armorTypes`/`equipTypes` use **numeric IDs**. Read types via `RA.typeList(project, kind)` (falls back to defaults for backward compatibility) rather than reading the array directly.

### Export pipeline

`buildStandaloneGame` inlines the runtime libs + `play.css`, embeds the project JSON and any used external assets into the HTML, and wires `runtime/messages.js` via an importmap/`data:` module URL. A standalone `.exe` export is that HTML appended to `bin/RPGAtlasLauncher.exe` after a payload marker; on launch the stub extracts the HTML to `%LOCALAPPDATA%/RPGAtlas Games/<name>/` and opens it.

## Custom assets (`img/`)

Subfolders: `characters/` (walk sheets, 3×4), `facesets/` (portraits, matched to character by filename), `enemies/`, `tilesets/`, `system/`. Tileset filename suffixes are semantic: `name.png` = blocked, `name.pass.png` = passable, `name.terrain.png` = passable Auto-Layer terrain. Replacing `img/system/icon_set.png` (256×256, 8×8 grid of 32×32) reskins all icons. The user manual is the 16-page `wiki/` (Markdown).

## Conventions

- camelCase functions/locals; UPPERCASE_SNAKE constants (`TILE`, `ICON_SIZE`, `MAX_SWITCHES`); shared singletons are PascalCase/UPPER globals (`Assets`, `RA`, `GLRender`). Tiny DOM helpers: `el(tag, cls, html)` and the hyperscript `h(tag, attrs, ...kids)`.
- Keep it dependency-free and build-free: no npm packages, no bundler, no transpile step. New shared code is a classic-script IIFE global; new editor/runtime code is an ES module.

## Git & PR workflow (personal policy — not upstream's)

This repo is a fork. `origin` = my fork (`bazsupport/RPGAtlas`, push here); `upstream` =
`DriftwoodGaming/RPGAtlas` (pull/sync only, **never push**). Contributions go upstream as PRs
from a feature branch on my fork. `CLAUDE.md` and `.claude/settings.json` are personal and stay
local via `.git/info/exclude` — never commit or push them.

### `gh` permission policy
- **Read-only `gh` is fine to run unprompted** — `gh pr view`, `gh pr list`, `gh pr checks`,
  `gh pr diff`. These only report status back to me.
- **Write/outward `gh` requires my explicit per-action approval, every time** — `gh pr create`,
  `gh pr comment`, `gh pr merge`, `gh pr close`, `gh pr edit`, and anything else that publishes to
  Driftwood's repo. Never run these on your own initiative; ask first and wait for my go-ahead.

### Rebase before merge — avoid conflicts proactively
- Branch each new feature off an **up-to-date `main`** (`git fetch upstream && git merge upstream/main`),
  not off another open feature branch, unless the new work genuinely depends on it.
- **Before opening a PR, and again whenever upstream moves while a PR is open, rebase the feature
  branch onto `upstream/main`** so conflicts are caught and resolved locally instead of blocking the
  merge. When resolving, keep **both** sides' work unless told otherwise; never clobber Driftwood's
  changes.
- After a rebase, re-run the tests and `git push --force-with-lease` (the PR updates automatically —
  do not open a new one).

## Verifying changes

- **A green test run is not proof it works in the browser.** `tests/modules.test.mjs` runs the modules under Node by stubbing `fetch` and `FileReader`; it exercises logic, not the real DOM/canvas/WebGL. After an editor or runtime change, also serve the repo and exercise the actual path in `index.html`/`play.html` (the failing screen, the real input) — don't let "tests pass" stand in for "the feature works."
- **Export re-inlines the runtime libs, so check both sides.** `buildStandaloneGame` inlines copies of the runtime libraries and `play.css` into the exported HTML/EXE. A change to a runtime lib (`engine.js`, `assets.js`, `gl.js`, `sfx.js`, `runtime/messages.js`, `data.js`) must be verified in *both* the live player and a fresh export, since the export carries its own embedded copy.
- **Bump the `?v=N` query** on a `<script>`/`<link>` in `index.html`/`play.html` when you change the file it points to, or the browser may serve a stale cached version.
