# C-SDLC v2 Direct Contract Review

## Findings

### P1: Claims have no typed operator route for heartbeat or recovery

- File: `csdlc-v2/src/lifecycle.rs:270-347`,
  `csdlc-v2/src/bin/csdlc-bind.rs:1-32`,
  `csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md`
- Role: lifecycle and operator contract
- Scenario: A bound issue outlives its initial claim lease or needs stale-claim
  recovery.
- Impact: The library can heartbeat and recover claims, but the final installed
  command surface and nine active skills expose no owner for either operation.
  Long-running issues cannot maintain or recover required session ownership
  through authorized v2 commands.
- Evidence: `heartbeat_claim` and `recover_claim` are exported and directly
  unit-tested, but no production binary calls either. `csdlc-bind` accepts only
  its initial bind request, while the bind skill documents only that command.
  The schema bundle publishes recovery input that no CLI consumes.
- Missing proof: Typed heartbeat and recovery entrypoints with lease, collision,
  topology, and replacement-record tests through the installed operator route.

### P2: GitHub owners do not implement the required shared token-file resolver

- File: `AGENTS.md:35-42`, `csdlc-v2/src/bin/csdlc-publish.rs:134-170`,
  `csdlc-v2/src/bin/csdlc-closeout.rs:372-405`
- Role: operator contract and code
- Scenario: An operator provides the approved token-file source through
  `ADL_GITHUB_TOKEN_FILE=<approved-token-file>` and runs publication or
  closeout after v1 sunset.
- Impact: Publication ignores that file and searches
  `<implementation-fallback-token-file>`; closeout ignores it and requires a path in the
  request. Normal documented configuration can fail or use a different local
  token source unless each caller duplicates token-path resolution in JSON.
- Evidence: Root policy requires a shared token resolver and names the approved
  environment/file source. Both binaries independently implement token lookup,
  recognize only token-value environment variables, and disagree on fallback
  file behavior. Neither reads `ADL_GITHUB_TOKEN_FILE` or calls a shared owner.
- Missing proof: One shared resolver with deterministic environment/file
  precedence and non-secret tests exercised by publish, readiness, and
  closeout.

## Discovery Boundary

This finding was derived directly during the issue #5375 orchestration pass.
Testing issue #5370 concerns PR-state collection through a shared token source,
but does not establish the inconsistent publication/closeout implementations
above. No new issue was created by this review.
