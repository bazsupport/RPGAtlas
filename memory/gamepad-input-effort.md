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

**Status (2026-06-16):** Phases 1–4 + a Phase-4 UX rework are committed; all headless tests pass
(`tests/input*.test.js`). **Pending baz's in-browser validation.** Notable: directions bind D-Pad +
left stick (`lstick_*`) and both are editable; in-game **Options** menu (Music folded in) with an
**editable add/replace/remove** rebinder; centered windows; capture uses ignore-held-until-release.

**Next:** (1) baz validates the rework; (2) one-time `localStorage` refresh of his existing "Atlas
Quest" project (stored gamepad bindings predate `lstick_*` — see HANDOFF §3); (3) **Phase 5** — editor
author-defaults grid, the `buildStandaloneGame` classic-script export rewrite (**export is already
broken on upstream independently** of this feature) + verify an exported HTML boots with a gamepad,
patch-notes entry, and fix the pre-existing `tests/modules.test.mjs` failure. PR only after baz
approves ([[validate-before-pr]]).
