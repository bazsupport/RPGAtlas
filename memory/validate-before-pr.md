---
name: validate-before-pr
description: Hard rule — never open/draft a PR or push to upstream until baz validates in-browser and explicitly approves.
metadata:
  type: feedback
---

**Never open or even draft a PR, and never push to `upstream`, until baz has validated the work in
the browser and explicitly approved it.** Work phase-by-phase and stop to hand back for his
validation between phases.

**Why:** baz validates everything himself; a green headless test run is NOT proof it works in the
real DOM/canvas/WebGL. He owns the "is this actually good?" call, especially on UX/feel.

**How to apply:** Implement a slice, report precisely what changed and how to test it, then wait.
Committing/pushing to his **own fork `origin`** for sync (e.g. machine switches) is fine **when he
asks** — that is not a PR. Outward actions to Driftwood need his explicit go-ahead every time
(see [[fork-and-config-policy]]). When he says "don't run build/test tooling," don't — just make the
change and describe it.
