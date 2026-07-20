# C-SDLC Identity And Ownership Residuals

Issue: #5547

Source findings:

- IR-4645-011: C-SDLC review identity accepted a review scope while revision
  hashing considered whole-tree state outside that scope.
- IR-4645-012: several large modules concentrate ownership across runtime,
  lifecycle, persistence, provider, scheduler, and review responsibilities.

## IR-4645-011 Disposition

Decision: C-SDLC review identity is scope-aware in v0.91.7.

The intended contract is that a review assignment scope is a real Git pathspec
boundary for the substantive revision used by review assignment, review guard,
doctor, and publication preparation. Dirty or untracked files outside the
declared review scope must not stale a scoped review. Dirty or untracked files
inside the declared review scope must still change the substantive revision and
block publication until re-review.

Implementation:

- `csdlc-v2/src/git.rs` now applies the supplied `scope` pathspec to both:
  - `git diff --no-ext-diff --binary HEAD -- <scope> :(exclude).csdlc/**`
  - `git ls-files --others --exclude-standard -- <scope> :(exclude).csdlc/**`
- `.csdlc/**` remains metadata-only and excluded from substantive revision
  identity.
- The clean commit revision remains the digest of `HEAD` only, so a clean
  scoped review still compares equal to `clean_commit_revision(HEAD)`.

Focused proof:

- `substantive_revision_honors_review_scope_pathspecs` proves out-of-scope
  tracked and untracked changes do not alter a `docs` scoped revision, while an
  in-scope `docs` change does.

Residual risk: none retained for IR-4645-011 if the focused test and C-SDLC v2
doctor pass. This does not change GitHub PR scope, review assignment semantics,
or publication remote validation beyond using the already-declared review
scope as the substantive revision pathspec.

## IR-4645-012 Ownership-First Split Plan

Disposition: planning-only in v0.91.7, implementation deferred to v0.91.8.

The large-module split must be behavior-first. Cosmetic file moves without
owner-specific tests are explicitly unsafe because these modules carry runtime
state, provider behavior, persistence, scheduling, and lifecycle truth.

### `adl/src/long_lived_agent.rs`

Current ownership concentration: agent runtime loop, ACIP/status handling,
provider interaction, lifecycle state, and operator-facing behavior are coupled
in one module.

v0.91.8 split sequence:

1. Extract agent state and heartbeat snapshots behind a pure data module.
2. Extract ACIP/operator message handling behind a protocol adapter.
3. Extract provider execution orchestration after provider status tests are in
   place.

Required proof before each move:

- Existing long-lived-agent runtime tests stay green.
- ACIP positive/negative packet behavior remains unchanged.
- Status snapshots are byte-for-byte compatible or versioned.

### `adl/src/csm_runtime_api.rs`

Current ownership concentration: HTTP API surface, CSM state projection,
runtime observability, and control-plane response formatting live together.

v0.91.8 split sequence:

1. Extract response DTOs and serialization contracts.
2. Extract runtime state query/read model from HTTP route glue.
3. Extract observability/CloudWatch projection only after live CSM proof lanes
   are isolated.

Required proof before each move:

- CSM API status/ready/metrics/events/contracts remain stable.
- HTML observatory live-probe compatibility is preserved.
- CORS and runtime linkages remain fail-closed and explicit.

### `adl/src/scheduler.rs`

Current ownership concentration: scheduling policy, queue mutation, retry
rules, and runtime tick behavior are hard to review independently.

v0.91.8 split sequence:

1. Extract scheduling policy decisions into deterministic pure functions.
2. Extract queue storage/mutation behind a small interface.
3. Move retry/backoff behavior only with scheduler trace fixtures.

Required proof before each move:

- Existing scheduler tests continue to pass.
- New fixtures prove identical order, retry, and tie-break behavior.
- Runtime tick evidence remains comparable across the move.

### `adl/src/provider_adapter.rs`

Current ownership concentration: provider selection, request shaping, response
normalization, failure classification, and metrics/status behavior are coupled.

v0.91.8 split sequence:

1. Extract provider request/response DTO normalization.
2. Extract failure taxonomy and status projection.
3. Split concrete provider-family adapters only after shared metrics contracts
   are proven stable.

Required proof before each move:

- Provider HTTP-family tests stay green.
- Metrics scaling and endpoint-family behavior remain unchanged.
- ACIP and runtime status surfaces continue to classify provider failures the
  same way.

### `csdlc-v2/src/store.rs`

Current ownership concentration: persistence, card projection, lifecycle
transition, review/publication projection, terminal receipt, and recovery logic
are in one large owner.

v0.91.8 split sequence:

1. Extract typed persistence/load/store helpers without changing serialized
   records.
2. Extract card projection hydration and validation.
3. Extract lifecycle transition commits from review/publication/terminal commit
   projections.
4. Extract terminal receipt reconciliation last, after closeout tests pin the
   retained receipt contract.

Required proof before each move:

- C-SDLC v2 gate tests pass for card identity, lifecycle, review, publication,
  readiness, and closeout.
- Generated card projections remain deterministic.
- Terminal receipt digests remain unchanged unless deliberately versioned.

## v0.91.7 Deferral Truth

Safe to defer to v0.91.8:

- All behavior-moving module splits from IR-4645-012.
- Any new module boundary public APIs beyond the plan above.

Not deferred:

- IR-4645-011 review identity behavior. It is implemented as scope-aware
  revision identity for v0.91.7.

Non-claims:

- This issue does not refactor large modules.
- This issue does not prove the v0.91.8 split sequence complete.
- This issue does not widen C-SDLC review scope beyond the assignment pathspecs.
