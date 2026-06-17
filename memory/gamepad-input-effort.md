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

**Status (2026-06-17):** feature-complete and **baz-validated in-browser**; entering PR-prep. All
input headless tests pass (`tests/input*.test.js`, plus heights/textcodes/traits). Phases 1–5 plus
two polish passes committed; nothing pushed. Highlights: in-game **Options** rebinder (add/replace/
remove, capture-ignore-held); editor author-defaults grid in its own **"Controls"** Database tab;
**procedural canvas glyphs** (`Assets.inputGlyphCanvas/DataUrl/Html`); the **`\input[action]`**
message text-code; **controller families** (`RA.padFamilyFromId` + `Input.padFamily()` auto-detect
Xbox/PS/Switch) as a **pure display layer** (bindings stay POSITIONAL; relabel only — no Nintendo
confirm/cancel semantic swap). Latest passes: the **in-game rebinder now renders procedural glyphs**
instead of text (`989309f`, baz-validated "looks amazing"); and **editor polish** (uncommitted) —
Controls-tab header on one line (`ACTION/KEYBOARD/GAMEPAD` aligned, Preview dropdown to the right of
Gamepad) + a collapsible **"Text codes" legend** on Show Text/Show Choices (`textCodesHelp()`, reads
`RA.INPUT_ACTIONS` so it stays in sync). A comment-cleanup pass trimmed three stale `keyName()`/
"phase 4" references; reviewers found nothing else.

**PR-prep TODO (in progress 2026-06-17):** (1) commit the uncommitted editor polish + comment cleanup;
(2) **rebase onto `upstream/main`** — currently 5 ahead / 4 behind; (3) **drop the off-topic
`.gitignore` change** added in `7d5f9c0` (it ignores `.claude/settings*`, which is already covered by
local `.git/info/exclude` — unrelated to gamepad support and a tooling tell for an upstream PR);
(4) reword the **`WIP: … (Phases 1–4 + rework)`** base commit — internal jargon for upstream;
(5) **standalone-export verification** — `buildStandaloneGame` re-inlines runtime libs; confirm an
exported HTML boots, takes a gamepad, renders glyphs/`\input` (export was already broken on upstream
independently). Known non-blocker: the pre-existing `tests/modules.test.mjs` `window is not defined`
failure is NOT from this feature (identical on `upstream/main`; `messages.js` is a classic script but
the test imports it as ESM) — flag, don't fix here. PR only after baz validates everything and
explicitly approves ([[validate-before-pr]]).

**Deferred follow-on:** [[custom-input-actions]] — author-defined custom input actions (design pinned).
