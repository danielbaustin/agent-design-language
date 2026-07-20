# Correct #5494 retained Runtime v3 design truth

## Problem

The terminal #5494 design says its scope is Runtime v2-only and that no
Runtime v3 changes occurred. PR #5504 actually changed the independent
`adl-runtime` crate and proved a Runtime v3 supervised typed-channel soak,
while separately proving the CSM production daemon path. The retained design
and diagram therefore contradict the merged implementation.

## Repair

- Use the typed terminal design-repair operation; do not hand-edit retained
  cards or terminal state.
- Describe the two complementary proof paths: the Runtime v3 supervision and
  channel soak, and the CSM production-daemon failure/recovery integration.
- Record that Runtime v3 remains the sole host-weather implementation owner
  and CSM does not duplicate weather.
- Preserve #5494 implementation, review, CI, publication, and closeout truth.

## Boundaries

- Documentation and C-SDLC terminal records only.
- No runtime source changes.
- No Runtime v2 source changes.
- No AWS execution.
