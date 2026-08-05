# Structured Task Prompt

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Issue #5757 corrective implementation and proof only.

## Deliverables

- Trusted localhost Runtime API origin guard
- Monotonic generation guard for async Observatory completions
- Real shared-certificate browser and authenticated WSS proof
- Focused validation evidence
- Ready PR with Closes #5757

## Acceptance

1. Untrusted runtime origins are rejected before any bearer token is attached or transmitted
2. Late retained/live/WSS completions cannot overwrite newer operator intent
3. Browser-visible HTTPS/WSS proof verifies shared certificate identity on ports 8765 and 20997
4. Focused Observatory tests, JavaScript syntax, Runtime HTTPS/WSS control proof, and diff hygiene pass
5. PR body includes Closes #5757
6. No /private/tmp is used

## Dependencies

- Current origin/main at 85f0aa3d1f6b442acb61ada97fb3a5a73b50a444
- Issue #5722 source context

## Inputs

- GitHub issue #5757
- GitHub issue #5722
- demos/html-observatory/app.js
- demos/html-observatory/README.md
- infra/runtime-v3/runtime-api-5665.toml
- Runtime v3 HTTPS/WSS proof tests

## Non Goals

- Observatory visual redesign
- Moving the HTML Observatory into the runtime binary
- Rewriting historical evidence
- Any #5748 lifecycle or worktree action
