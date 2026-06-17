---
name: gamepad-input-effort
description: Current effort — full gamepad + input-remapping system on feature/gamepad-support; status, key design, next steps.
metadata:
  type: project
---

**Effort:** a full gamepad + input-remapping system for RPGAtlas (wanted upstream — the maintainer
"hopes to see full gamepad support"). Branch: **`feature/gamepad-support`**. Full design in
`HANDOFF.md` §3 and (machine-local) `~/.claude/plans/swift-cooking-ladybug.md`.

**Design seam:** the engine + menus consume **named actions** (`up/down/left/right/ok/cancel/dash/
attack`) only, never raw keys/buttons. `js/runtime/input.js` (classic — see
[[classic-script-contract]]) owns keyboard + gamepad, exposes `pressed/justPressed/consume/dir/
poll/setBindings/beginCapture/label`. Bindings: `{keyboard:{action:[e.code]}, gamepad:{action:
[padName]}, stickDeadzone}`; gamepad uses generic names (`face_south/east/west/north`, `dpad_*`,
`lstick_*`, `bumper_l/r`, `trigger_l/r`, `start`, `select`, `stick_l/r`). Author defaults in
`proj.system.input` (`RA.defaultInput()`); per-player override persisted at
`rpgatlas_<gameId>_options`, merged on top at boot via `RA.mergeInputBindings`.

**Status (2026-06-17):** Phases 1–5 committed and **baz-validated in-browser**; all headless tests
pass (`tests/input*.test.js`). Nothing pushed. Phase-4 = in-game **Options** menu (Music folded in)
with an editable add/replace/remove rebinder, centered windows, capture-ignore-held-until-release;
directions bind D-Pad + left stick (`lstick_*`), both editable. **Phase 5** (commits `750f279`,
`712209d`) added: editor author-defaults grid in its own **"Controls"** Database tab; **procedural
canvas glyphs** (`Assets.inputGlyphCanvas/DataUrl/Html`); the **`\input[action]`** message text-code;
shared label/glyph helpers moved into `RA` (`codeLabel`/`glyphText`/`glyphShape`, family-aware) with
`input.js`/`assets.js` delegating lazily; **distinct d-pad/stick/L3 shapes**; and **controller
families** (`RA.padFamilyFromId` + `Input.padFamily()` auto-detect Xbox/PS/Switch from `Gamepad.id`)
as a **pure display layer** — bindings stay POSITIONAL, editor brand-preview is never persisted,
**relabel only (no Nintendo confirm/cancel semantic swap)**. Also: START is unbound by default now
(`cancel` = `face_east`).

**Next:** (1) optional — upgrade the in-game rebinder rows from text to procedural glyphs (engine
`controlsDevice`/`actionBindings`, via `Assets.inputGlyphHtml(...,Input.padFamily())`); (2)
`buildStandaloneGame` classic-script export rewrite (**export already broken on upstream
independently**) + verify an exported HTML boots, takes a gamepad, renders glyphs/`\input`; (3)
deferred design — Nintendo confirm/cancel **semantic** swap (per-family binding overrides; design with
baz first); (4) one-time `localStorage` refresh of his "Atlas Quest" project if its bindings predate
`lstick_*`; (5) fix the pre-existing `tests/modules.test.mjs` `window` failure. PR only after baz
validates everything and explicitly approves ([[validate-before-pr]]).
