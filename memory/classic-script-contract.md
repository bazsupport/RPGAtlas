---
name: classic-script-contract
description: engine.js / messages.js / input.js are CLASSIC scripts (window.createX globals), NOT ES modules — CLAUDE.md's table is stale here.
metadata:
  type: reference
---

In the **player runtime**, `js/engine.js`, `js/runtime/messages.js`, and `js/runtime/input.js` are
**classic scripts**: each defines a factory and attaches a `window.createX` global (e.g.
`window.createMessageSystem`, `window.createInputSystem`) with **no `import`/`export`**. `engine.js`
loads as a plain `<script src="js/engine.js?v=N">` in `play.html` (not `type="module"`) and reads its
deps off `window.RPGAtlasDeps` / `window.createX`.

**Only `js/editor.js` is a real ES module.** Parts of `CLAUDE.md`'s module table still describe
`engine.js`/`messages.js` as ES modules — that is **stale**; upstream changed to the classic-script
contract. Verify against `play.html`'s `<script>` chain if unsure.

**Implications when adding shared runtime code:** write a classic IIFE that attaches a global, load
it via a plain `<script>` in `play.html` **and** inline it the same way in the standalone export
(`js/editor/project-io.js` `buildStandaloneGame`). Bump the `?v=N` cache-bust on any changed
`<script>`/`<link>`. This is exactly how `js/runtime/input.js` was added — see
[[gamepad-input-effort]].
