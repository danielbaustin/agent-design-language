# C-SDLC Role-Provider Profiles Proof Note for #4053

## Scope

This note records the bounded WP-05 contract surface for role-provider
profiles.

It defines a stable C-SDLC policy layer above the low-level provider substrate
and above the provider/capability split from `#4007`. It does not claim that
external providers become autonomous ADL actors, that every evidenced lane is
already backed by a tracked in-code provider profile, or that raw provider
output becomes final review truth.

## Source evidence

- `docs/adr/0004-provider-profiles.md`
- `adl/src/provider/profiles.rs`
- `docs/milestones/v0.91.5/review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`
- `docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`
- `docs/milestones/v0.91.6/review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md`

Bootstrap/design input that informed issue creation but is not relied on as
tracked local proof in this worktree:

- `.adl/docs/TBD/cognitive-sdlc/C_SDLC_ROLE_PROVIDER_PROFILES_AND_REVIEW_PROVIDER_PLAN.md`

Adjacent sprint context only, not required local proof for this packet:

- `#4007`
- `#4008`
- `#4009`
- `#4010`
- `#4011`

## Contract layering established here

`#4053` fixes the layering boundary for role-provider work:

1. provider profiles remain the low-level substrate authority
2. capability profiles remain provider-independent behavioral descriptors
3. role-provider profiles become a higher-level C-SDLC policy layer

The role-provider layer may select, constrain, and record low-level routes for
specific lifecycle roles, but it does not replace provider profiles and does
not bypass the ADL/Codex control plane.

## Stable role profiles established here

The stable role-profile surface is:

- `conductor_provider`
- `architect_provider`
- `implementer_provider`
- `reviewer_provider`
- `tester_provider`

These are lifecycle abstractions, not vendor or model names.

Each role profile owns:

- ordered route policy for that role
- required and forbidden capability expectations
- output-contract references
- evidence and failure-routing policy
- advisory-authority limits for the resulting output

Each role profile does not own:

- raw provider transport
- credential lookup semantics
- direct repo mutation authority
- merge, closeout, or release authority
- hidden runtime heuristics outside recorded policy

## `ProviderRouteV1` boundary

The low-level route record for a selected role-provider lane should preserve the
existing deterministic substrate rather than introducing a new ambiguous
provider identity surface.

Required route fields for the WP-05 contract are:

- `provider_profile_ref` when the selected route is backed by a tracked
  substrate profile id
- `provider_spec_kind` from the low-level substrate vocabulary
- optional `provider_family` metadata for human-readable grouping only
- `model_ref`
- `model_identity`
- `runtime_surface`
- `provider_selection_reason`
- `route_resolution_trace`
- `output_contract_ref`

Important boundary:

- `provider_family` is descriptive metadata only
- `provider_profile_ref` remains the deterministic selection anchor when a
  tracked substrate profile exists
- role-provider policy must not upgrade `provider_family` into the authoritative
  selector

## Deterministic resolution policy

Role-provider resolution is deterministic by default.

This issue establishes the following policy:

1. each role profile declares ordered candidate routes
2. resolver eligibility checks must be explicit and policy-named
3. eliminated routes must be recorded with reasons
4. missing dynamic data fails closed
5. randomized load balancing is out of scope for v1
6. fallback is allowed only through recorded ordered policy, never through
   hidden provider substitution

This keeps role routing reviewable and compatible with the fail-closed posture
already established for provider errors and resilience routing.

## Advisory authority boundary

External role-provider outputs remain advisory unless mediated by the normal ADL
control plane.

That means:

- conductor outputs may recommend workflow transitions, but may not perform
  them
- architect outputs may propose boundaries or ADR material, but may not claim
  accepted design truth by themselves
- implementer outputs may propose patches, but may not mutate repo state
  outside Codex/workflow execution
- reviewer outputs may produce findings packets, but may not become final
  review truth without synthesis or operator review
- tester outputs may propose tests or PVF plans, but may not silently widen
  validation scope or alter release-gate policy

No external provider route gains authority to close issues, merge PRs, publish
release artifacts, or bypass lifecycle-card mediation.

## Role-to-route contract

The role-provider contract for WP-05 is:

| Role profile | Stable role purpose | Required route posture | Current strongest bounded evidence | Authority limit |
| --- | --- | --- | --- | --- |
| `conductor_provider` | workflow sequencing, blocked/ready judgments, dependency routing | conservative, deterministic, low-hallucination routing | the tracked multi-agent matrix packet records bounded planning-route evidence, with OpenRouter DeepSeek V4 Flash as the strongest currently documented planning lane | may recommend next steps only |
| `architect_provider` | boundaries, ADR candidates, tradeoffs, migration risk | long-context reasoning with explicit claim boundaries | the tracked multi-agent matrix packet records bounded review/planning-quality hosted lanes; no separate architecture-only proof is claimed here | may propose design findings only |
| `implementer_provider` | bounded patch proposals and focused execution assistance | code-editing lane with explicit write-set discipline and failure stop conditions | the tracked multi-agent matrix packet records bounded coding support through OpenRouter GPT-4o-mini and Qwen 3.6 Flash as `supported_with_limits` | may propose patches only |
| `reviewer_provider` | findings-first review packets with evidence references | severity-aware, evidence-grounded, synthesis-ready output | the tracked multi-agent matrix and OpenRouter proof packets provide the strongest bounded reviewer-lane evidence; the runtime/logging proof adds the bounded diagnostic floor | may produce advisory findings only |
| `tester_provider` | focused tests, PVF routing, and coverage-gap proposals | deterministic test-planning and edge-case discipline | current strongest evidence is still indirect through bounded coding/review lanes; no universal tester-lane proof is claimed yet | may propose tests and PVF plans only |

## Mapping to the low-level provider substrate

Current tracked substrate evidence from `adl/src/provider/profiles.rs` and ADR
0004 proves deterministic low-level profile handling for static profile ids such
as:

- `chatgpt:gpt-5.3-codex`
- `chatgpt:gpt-5.4`
- `claude:claude-3-5-haiku`
- `claude:claude-3-7-sonnet`
- `http:gemini-2.0-flash`
- `http:deepseek-chat`
- `ollama:qwen2.5-7b`
- `ollama:phi4-mini`
- `mock:echo-v1`

This matters because role-provider profiles must reference the substrate when a
tracked profile id exists instead of inventing a parallel identity scheme.

Important current limit:

- the strongest bounded OpenRouter and remote Gemma route evidence from the
  tracked multi-agent, OpenRouter, and remote Gemma proof packets is expressed
  today as provider/model lane truth, not as a
  tracked in-code provider-profile registry entry shown in `profiles.rs`
- therefore `#4053` does not claim every currently useful lane already has a
  first-class profile id in the static registry
- the role-provider contract consumes those lanes only through explicit
  deterministic policy records until the low-level substrate grows equivalent
  tracked profile coverage where needed

## Review-provider v1 consumption boundary

This issue also fixes the review-provider consumption posture for CodeFriend and
related review lanes.

Review-provider outputs should become typed review artifacts such as:

- findings packets
- review notes
- confidence and uncertainty fields
- evidence references
- provider/model/run metadata
- synthesis inputs

They should not become:

- raw provider transcript as final review truth
- auto-merged findings without synthesis
- hidden no-findings results when a provider failed, timed out, or returned
  malformed output

The bounded v1 review-provider posture is therefore:

- structured request in
- structured advisory result out
- synthesis or operator review before durable review truth

## Failure and evidence posture

Role-provider routing must consume the shared provider failure vocabulary rather
than flattening all route failures into generic model errors.

The current bounded integration visible from the tracked runtime/provider
logging proof plus the shared provider/profile substrate means role-provider
selection and execution notes may safely depend on:

- explicit provider auth failures as operator-gated
- timeout and rate-limit failures as retryable classes
- model unavailable and empty-output classes as terminal or invalid-output
  states
- adapter-backed logging and redaction floors already established by the
  tracked runtime/provider logging proof

This issue does not claim that every candidate role route already has identical
telemetry depth or resilience execution proof.

## What this issue documents for `#4012`

`#4012` may consume `#4053` as the documented/specification packet showing how
WP-05 should account for provider routing in addition to provider reliability.
This issue does not claim to implement the routing layer in code. It documents
that the provider closeout should verify:

- the stable role-provider abstraction above provider and capability profiles
- the deterministic route-resolution policy this milestone intends to preserve
- the advisory-authority limits required for external provider lanes
- the review-provider ingestion boundary for CodeFriend-style review lanes
- the truthful limit that some evidenced lanes are still policy-recorded bounded
  routes rather than static substrate profiles

## Non-Claims

- This note does not claim every useful provider lane is implemented as a
  tracked static profile id today.
- This note does not grant external providers autonomous lifecycle authority.
- This note does not replace native Codex subagents.
- This note does not prove a complete multi-provider execution harness for all
  five roles.
- This note does not claim OpenRouter, remote Gemma, hosted direct-provider,
  and local Ollama lanes all have identical quality or failure-handling depth.
