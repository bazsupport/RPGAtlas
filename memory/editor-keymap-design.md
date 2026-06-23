---
name: editor-keymap-design
description: Editor keyboard-shortcut scheme and the design rules behind it (semantic vs positional keys; deliberately keyless dialogs).
metadata:
  type: project
---

The editor keymap (shipped via upstream PR #17, branch `feature/editor-keymap`) follows two
distinct assignment rules, on purpose:

- **Positional / muscle-memory** for high-frequency drawing: tools are a contiguous left-hand row
  `Q W E R T Y` (Pen/Eraser/Rect/Circle/Fill/Shadow); layers are `` ` `` `1 2 3 4`
  (Auto/Ground/Decor/Decor 2/Overhead); modes cycle with `Tab` / `Shift+Tab`
  (Map → Event → Passability → Height, wraps). "Start position" is **excluded from the Tab cycle**
  (reached via the Mode menu) — see `MODE_CYCLE` in `js/editor.js`.
- **Semantic, individually memorized** for app actions: `F1` Database, `F2` HD-2D Preview, `F5`
  Playtest. These are assigned by *meaning*, not toolbar order, so non-contiguous gaps (F3/F4
  unused) are fine and intentional — do **not** fill them just for contiguity.

The other dialog-openers (Plugins, Audio, Search, Resources, Character Generator) are
**deliberately left keyless** — infrequent one-shot dialogs that don't earn a prime key.

`?` opens a Keyboard Shortcuts overlay. It and the toolbar tooltips/menus are **registry-driven**:
each action's `key` field on the `ACT` registry auto-populates the tooltip suffix, the menu
`.mi-key`, and the overlay (`aKey`/`aLabel` read `ACT[id]`), so there is no separate list to keep in
sync. Keydown-handler ordering matters: Height mode consumes `0`–`9` for elevation and that branch
must stay **above** the layer gate; F-keys are global and use `preventDefault()` to suppress the
browser's Help/Reload; tools are gated to Map/Height mode, layers to Map mode.

This overhaul **changed existing defaults**, not just added keys: tools were `B E R O F S`, layers
`0 1 2 3 4`, Zoom-1:1 `Ctrl+0`, and Height had a standalone `H` (now removed). PR #17 calls this out.

**Why:** the positional-vs-semantic split and the keyless-dialog decision aren't obvious from the
code; recording them keeps future shortcut work from re-litigating them or filling F-key gaps for
the wrong reason.

**How to apply:** to add a shortcut, set the action's `key` field on `ACT` (that alone documents it
in tooltip + menu + `?` overlay). Decide positional (drawing/contiguous) vs semantic (F-key);
leave low-traffic dialogs keyless unless baz asks otherwise. See [[user-baz]] — baz validates the
feel in-browser himself.
