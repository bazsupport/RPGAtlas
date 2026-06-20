---
name: gamepad-input-effort
description: SHIPPED — full gamepad + input-remapping system (merged upstream PR #14/#15). Design seam, what shipped, what's still deferred.
metadata:
  type: project
---

**Status (shipped 2026-06-17/18):** the full gamepad + input-remapping system is **merged to
`upstream/main`** (PR #14 "Add gamepad support and input remapping"; options-menu polish in PR #15)
and is in `main`. The `feature/gamepad-support` branch has been deleted. Kept as a design reference
for any future input work.

**Design seam (still the model):** the engine + menus consume **named actions** (`up/down/left/right/
ok/cancel/dash/attack`) only, never raw keys/buttons. `js/runtime/input.js` (classic — see
[[classic-script-contract]]) owns keyboard + gamepad and exposes `pressed/justPressed/consume/dir/
poll/setBindings/beginCapture/label`. Bindings: `{keyboard:{action:[e.code]}, gamepad:{action:
[padName]}, stickDeadzone}`; gamepad uses generic positional names (`face_south/east/west/north`,
`dpad_*`, `lstick_*`, `bumper_l/r`, `trigger_l/r`, `start`, `select`, `stick_l/r`). Author defaults in
`proj.system.input` (`RA.defaultInput()`); per-player override persisted at `rpgatlas_<gameId>_options`,
merged on top at boot via `RA.mergeInputBindings`.

**What shipped:** in-game **Options** rebinder (add/replace/remove per action, conflict detection,
reset, capture-ignore-held); editor author-defaults grid in its own **"Controls"** Database tab;
**procedural canvas glyphs** (`Assets.inputGlyphCanvas/DataUrl/Html`) with distinct d-pad / analog-
stick / stick-click shapes; the **`\input[action]`** message text-code; **controller families**
(`RA.padFamilyFromId` + `Input.padFamily()` auto-detect Xbox/PS/Switch) as a **pure display layer** —
bindings stay POSITIONAL, relabel only, **no Nintendo confirm/cancel semantic swap**. The in-game
rebinder renders the procedural glyphs (not text). Headless coverage: `tests/input.test.js`,
`tests/input-pad.test.js`, `tests/input-capture.test.js`.

**Still deferred (not built):** the Nintendo confirm/cancel **semantic** swap (needs per-family
binding overrides — design first); and the standalone-export verify/rewrite (`buildStandaloneGame`
still wires the entry as `type=module`, flagged broken on upstream — not verified to boot). Future
follow-on: [[custom-input-actions]].
