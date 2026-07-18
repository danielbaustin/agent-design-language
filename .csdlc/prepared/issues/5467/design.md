# #5467 Design

Repair the local CI contract so it reaches every backend-snapshot assertion and behaviorally proves hosted, Spot-selected, and invalid backend routing without invoking AWS.

The change is limited to the shell contract and, only if needed for testability, the backend-selection block in `.github/workflows/ci.yaml`. Local fixtures may interpret the workflow selection logic, but must not call cloud commands, credentials, or remote validation workflows.

Acceptance requires the contract to fail if any snapshot assertion is removed or bypassed, and to prove the three backend inputs have deterministic local outcomes.
