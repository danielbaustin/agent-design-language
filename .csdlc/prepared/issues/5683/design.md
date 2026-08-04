# Issue 5683 Design: Observatory Visual Remediation

## Intent

Remove the visibly corrupted-looking cyan and green blocks, uncontrolled
emission, exposed voids, and weak architectural grounding that remain in the
flagship Observatory proof from #5662. Preserve its fixed operator dashboard,
Runtime v3 feed classification, and fail-closed communication truth.

## Ownership Boundary

The operator-provisioned FastWork Unity project is the live authoring and proof
surface. Licensed asset roots remain external and untracked. Repository-owned
changes are limited to the declared Observatory scene, staging/editor code,
shell resources, validation, and issue-local proof.

No player binary, owner binary, licensed pack, or replacement runtime is built
or published. Unity scene mutation and visual proof use Unity-MCP or the editor.

## Diagnostic Loop

1. Prove the live editor project, scene, and MCP endpoint before mutation.
2. Capture the current 1920x1080 Play Mode frame from the presentation camera.
3. Inventory visible renderers, particle systems, lights, volumes, and staging
   helpers in the camera frustum.
4. Isolate the artifact source by toggling bounded object groups and capturing
   comparison frames.
5. Disable, replace, or retune only the responsible objects and materials.
6. Rebalance camera, terrain, architecture, and lighting after artifact removal.
7. Verify the fixed dashboard and Runtime v3 truth states at both resolutions.
8. Retain fresh 1920x1080 and 2560x1440 frames and visually inspect both.

## Visual Acceptance

- The hero view contains no conspicuous cyan or green blocks, corrupted-looking
  particles, accidental debug meshes, or blown emissive surfaces.
- Architecture and terrain read as one grounded campus with continuous access,
  intentional supports, and no visible staging voids.
- Neutral key and fill lighting preserve material detail and depth.
- Cyan is a restrained systems accent, amber communicates event emphasis, and
  green is reserved for healthy runtime state.
- The fixed dashboard remains crisp and non-overlapping, with scrolling only in
  the intended event and inspector windows.

## Runtime Truth

The #5662 Runtime v3 and legacy adapter behavior remains authoritative:

- live requires an accepted current runtime document;
- degraded reports partial or unhealthy runtime truth;
- disconnected reports absence or transport failure;
- demo or fixture state stays explicitly labeled;
- unsupported governed communication fails closed with an exact reason.

Visual remediation must not manufacture live runtime, CloudWatch, governance,
evidence, or communication claims.

## Proof Boundary

Static source checks are guardrails only. Success requires direct Unity proof
from the intended FastWork project and `FlagshipObservatoryStage` in Play Mode,
plus retained and visually inspected 1920x1080 and 2560x1440 frames. The proof
index must link those images and the implementation PR, never a generated
terminal-reconciliation branch.

## Tooling Anomalies

Every Unity-MCP, editor, asset, runtime, GitHub, C-SDLC, or proof-tool anomaly
encountered during execution is recorded in #5683 or routed to a named owner.
The initial anomalies are the typed GitHub marker-readback race and
`csdlc-install resolve` rejecting the tracked symlinked generation selector.
