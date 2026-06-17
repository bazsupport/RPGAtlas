---
name: custom-input-actions
description: Future goal (post-gamepad-PR) — let authors define/name custom input actions with default bindings; pinned design.
metadata:
  type: project
---

**Future goal (deferred — build as its own effort after the gamepad PR lands; baz said "plan and
I'll say when").** Let an author **add and name their own input actions** in the editor, each with
default keyboard + gamepad bindings, on top of the built-ins. Part of [[gamepad-input-effort]]; safe
to add because the input layer is already generic — `buildActionList()` in `js/runtime/input.js`
derives per-action state from whatever keys exist in the bindings map, not from a fixed list.

**Pinned design (agreed with baz 2026-06-17):**

- **Define UI** in the **Controls** Database tab: add/name a custom action + set default keyboard &
  gamepad bindings. Store on the project (e.g. `proj.system.inputActions`); fold its defaults into
  `RA.defaultInput()` / `RA.mergeInputBindings`. Built-ins (`RA.INPUT_ACTIONS`, with
  `RA.INPUT_CRITICAL` = ok/cancel) stay **non-removable / non-renamable**. The editor grid and the
  in-game rebinder (`engine.js` `controlsDevice`/`actionBindings`) iterate **built-ins + custom**.
- **Consumers — baz's call, ONLY these two:**
  1. A new **Conditional Branch "Input Pressed"** condition: add to the editor's condition dropdown
     (`editor.js`, near the other Conditional Branch kinds) + a `testCond()` case in `engine.js`
     calling `Input.pressed(action)`. (Events currently have **no** way to query input — kinds are
     switch/var/selfsw/quest/item/gold/actor; triggers are action/touch/auto/parallel.)
  2. Usable in the **`\input[name]` message text-code** — already resolves once the action exists in
     bindings (`engine.js` `inputPromptGlyph`). NOTE: the in-editor "Text codes" legend
     (`textCodesHelp()` in `editor.js`) currently lists only `RA.INPUT_ACTIONS`; to auto-include
     custom actions it must also read `proj.system.inputActions`.
- **Explicitly OUT:** no "press → fire a Common Event" trigger.
