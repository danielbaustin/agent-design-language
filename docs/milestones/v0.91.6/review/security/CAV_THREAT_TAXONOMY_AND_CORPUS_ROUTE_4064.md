# CAV Threat Taxonomy And Corpus Route for #4064

## Scope

This packet records the bounded WP-07 security review for Continuous
Adversarial Verification on the `v0.91.6` activation path.

It is a bridge-readiness and routing artifact, not a claim that ADL already has
an always-on integrated red/blue bug-finding loop, not a license to run
unbounded adversarial probes, and not a security certification surface.

## Source evidence

- `.adl/v0.91.6/tasks/issue-4064__wp07-cav-threat-taxonomy-corpus-route/stp.md`
- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`
- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md`
- `docs/security/THREAT_MODEL_v0.7.md`
- `docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`
- `docs/milestones/v0.91.6/review/security/PROVIDER_MODEL_CAV_TRUST_BOUNDARY_REVIEW_4020.md`
- `adl/src/adversarial_runtime.rs`
- `adl/src/red_blue_agent_architecture.rs`
- `adl/src/adversarial_execution_runner.rs`
- `adl/src/exploit_artifact_replay.rs`
- `adl/src/continuous_verification_self_attack.rs`
- `adl/src/demo/adversarial_self_attack.rs`
- `adl/src/cli/identity_cmd/contracts.rs`
- `adl/src/cli/identity_cmd/tests/adversarial_contracts.rs`

## Review goal

Determine what Continuous Adversarial Verification surfaces are already real and
reviewable in the repository, what threat taxonomy and security-corpus
structure `v0.91.6` can safely consume, and what must remain routed to later
security work instead of being upgraded into an implicit "CAV works fully"
claim.

## Current runnable baseline

### Verified bounded proof hooks

The current repository already supports bounded, runnable CAV-adjacent proof
surfaces:

| Surface | Current evidence | Current disposition |
| --- | --- | --- |
| Adversarial runtime model | `adl identity adversarial-runtime --out ...` emits `adversarial_runtime_model.v1` and the focused `adversarial_contracts` tests pass. | `working_bounded_contract_surface` |
| Red/blue role architecture | `adl identity red-blue-architecture --out ...` emits `red_blue_agent_architecture.v1` and the focused `adversarial_contracts` tests pass. | `working_bounded_contract_surface` |
| Adversarial execution runner contract | `adl identity adversarial-runner --out ...` emits `adversarial_execution_runner.v1` and the focused `adversarial_contracts` tests pass. | `working_bounded_contract_surface` |
| Exploit artifact and replay manifest contract | `adl identity exploit-replay --out ...` emits `exploit_artifact_replay.v1` and the focused `adversarial_contracts` tests pass. | `working_bounded_contract_surface` |
| Continuous verification/self-attack contract | `adl identity continuous-verification --out ...` emits `continuous_verification_self_attack.v1` and the focused `adversarial_contracts` tests pass. | `working_bounded_contract_surface` |
| Flagship adversarial loop demo | `adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open` succeeds and lands exploit, replay, mitigation, promotion, review-packet, and trace artifacts. | `working_bounded_demo_surface` |

### What does not exist yet

The current repository does **not** prove the following:

- an always-on autonomous red/blue security-operations loop;
- a general live "set red and blue teams running to find bugs" scheduler;
- unbounded exploit execution against external or undeclared targets;
- a complete security corpus service with operational ingestion, indexing, and
  lifecycle ownership;
- a production security tournament or certification-grade security program.

Those remain future work and must not be smuggled into `v0.91.6` closeout.
The owning route is:

- immediate closeout owner: `#4024`;
- residual bridge owner if still open at milestone closeout:
  `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`;
- integrated implementation/provenance owner:
  `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`.

## Integrated execution route

The correct future route is to make these surfaces work inside the existing ADL
runtime, not to stand up a separate security runtime.

### Concrete runtime anchor points

The implementation route should bind to code that already owns recurring
execution, runtime status, and governed control:

- `adl/src/long_lived_agent.rs` and `adl/src/long_lived_agent/types.rs` for
  cycle scheduling, status, lease, stop, and inspection behavior;
- `adl/src/continuous_verification_self_attack.rs` for the declared
  verification lifecycle and posture rules;
- `adl/src/adversarial_execution_runner.rs`, `adl/src/adversarial_runtime.rs`,
  and `adl/src/exploit_artifact_replay.rs` for the current contract/artifact
  families that need promotion from proof hooks into runtime-owned execution;
- `adl/src/freedom_gate.rs` plus ordinary run-state and trace artifacts for
  authority, defer, and evidence visibility;
- the existing `adl agent tick|run|status|inspect|stop` CLI surface for
  operator-facing cadence control.

### Phase 1: Runtime-owned target and posture control

- promote target selection, posture declaration, and stop conditions into the
  ordinary runtime/workflow control plane;
- require every adversarial action to bind to an owned ADL target surface and
  governed policy context;
- keep Freedom Gate, authority, and audit visibility in the same runtime path
  used by other governed work.

### Phase 2: Runtime-owned red/blue orchestration

- execute red discovery, blue mitigation, and purple prioritization as declared
  roles within the main runtime lifecycle;
- make scheduler ownership, cadence, limits, and defer states explicit in the
  existing run/trace artifacts instead of inventing an external orchestration
  plane;
- preserve attributable evidence and replay linkage for every stage.

### Phase 3: Integrated artifact and corpus lifecycle

- persist exploit, replay, mitigation, validation, and promotion artifacts
  through the normal runtime artifact roots and review surfaces;
- attach adversarial results to existing trace, continuity, and evidence
  surfaces so security findings become first-class runtime knowledge;
- grow the corpus through bounded promotion and regression seeding, not through
  free-form narrative notes.

### Phase 4: Bounded bug-finding loop

- allow repeated red/blue execution only against declared local or otherwise
  governed targets;
- promote successful exploit families into reusable regression seeds;
- keep operator stop authority, posture, target ownership, and review gates in
  the same ADL lifecycle used for other high-consequence work.

### Phase 5: Scaled future work

- only after the integrated bounded loop is real should ADL consider
  higher-cadence or research-style continuous bug-finding modes;
- scaled or tournament-style operation remains future work beyond this issue.

### Concrete implementation shape

The nearest truthful end-state is not a new `adl adversarial-runtime daemon`
or parallel security subsystem. It is an ordinary ADL long-lived agent or
runtime workflow that happens to execute adversarial stages under stronger
governance:

1. an operator binds a governed target and posture through a normal ADL agent
   spec or equivalent runtime-owned workflow definition;
2. `adl agent run` or `adl agent tick` schedules bounded red/blue cycles using
   existing lease, status, stop, and inspect behavior;
3. each cycle executes the declared continuous-verification stages as ordinary
   runtime work rather than identity-only export commands;
4. exploit evidence, replay results, mitigation links, and promotion/defer
   decisions land in the normal runtime artifact and review surfaces;
5. replayable bug classes become governed regression seeds instead of staying
   demo-only knowledge.

Illustrative future entrypoint:

`adl agent run --spec .adl/long_lived_agents/cav_red_blue_agent.yaml --max-cycles <n> --json`

This issue does not claim that entrypoint exists yet. It defines the route that
later implementation work should follow.

## Threat taxonomy and current route

| Threat class | Why it matters on the activation path | Current evidence | Current disposition | Residual route |
| --- | --- | --- | --- | --- |
| Prompt injection | Provider/model and public-record consumers can be manipulated without transport failure. | `#4005`, `#4020`, and the v0.91.5 source packet keep prompt-injection on the activation path. | `reviewed_and_routed` | `#4024` closeout must keep the class open unless later work closes it explicitly; broader operational regression remains a `v0.93` consumer. |
| Retrieval poisoning | Retrieved evidence can corrupt downstream review or decision quality. | Threat-model/CAV source packet plus exploit-replay and continuous-verification contracts establish the category and replay-oriented doctrine. | `reviewed_and_partially_bounded` | Security-corpus and regression-seed follow-through belong to `v0.93` WP-S6 consumers. |
| Memory poisoning | Poisoned memory becomes behavior corruption on later steps. | v0.91.5 source packet names ObsMem/memory pressure; WP-10 and `#4022` keep privacy/memory boundaries explicit. | `reviewed_and_routed` | WP-10 remains implementation owner; operational adversarial regression remains future work. |
| Tool abuse | Tool invocation amplifies capability and can become a privilege boundary failure. | Existing adversarial runner contract requires declared posture, target scope, and evidence capture. | `working_bounded_contract_surface` | A general autonomous runner remains downstream. |
| Capability escalation | Unreviewed capability gain can bypass governance or policy. | Threat model, runner posture rules, and Freedom Gate constraints keep escalation explicit. | `working_bounded_boundary_with_residuals` | Wider security-operations proof remains routed to later security work. |
| Identity forgery | Identity corruption becomes authority corruption. | Threat-model source packet and WP-08 dependency keep identity security active. | `reviewed_and_routed` | WP-08 remains the implementation owner; WP-07 must not close identity-safe claims here. |
| Delegation abuse | Delegation can launder authority or hide accountability. | Threat-model/CAV doctrine plus WP-06/WP-07 security review packets keep delegation on-path. | `reviewed_and_routed` | WP-06/WP-08 remain active owners where delegation semantics or identity continuity are still open. |
| Constitutional bypass | Adversarial pressure against Freedom Gate or policy review is a core CAV concern. | Continuous-verification contract, runner posture rules, and the source packet make the no-bypass boundary explicit. | `working_bounded_boundary_with_residuals` | Full regression and live loop coverage remain downstream. |
| Trace tampering | Security findings without attributable replay/evidence become unauditable. | Exploit replay contract, continuous-verification artifact chain, and the demo trace/review packet keep evidence linkage explicit. | `working_bounded_contract_surface` | Signed/queryable trace convergence remains later security work. |
| Governance drift | Hidden policy weakening can convert bounded security work into theater. | Purple coordination and posture rules make governance visible in bounded proof surfaces. | `working_bounded_contract_surface` | Full operational governance and corpus promotion remain future work. |

## CAV loop and corpus disposition

| CAV loop element | What exists now | v0.91.6 disposition | Future owner |
| --- | --- | --- | --- |
| Discover | Red-role architecture, adversarial runtime assumptions, and exploit-hypothesis artifacts exist. | `bounded_contract_and_demo_present` | `#4024` must preserve discovery as an open integrated-runtime gap in `v0.91.6`; if still unresolved, route it to `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`, with scaled security-operations ownership in `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`. |
| Reproduce | Replay manifests and the flagship demo's pre-fix replay exist. | `bounded_contract_and_demo_present` | `#4024` owns the residual for general runtime-owned replay operations in `v0.91.6`; unresolved follow-through routes through `v0.91.7` residual readiness, then into `v0.93` adversarial provenance work. |
| Classify | Classification artifact family and demo review packet exist. | `bounded_contract_and_demo_present` | `#4024` must preserve the gap between current bounded classification artifacts and a richer runtime-owned security taxonomy; unresolved ownership routes through `v0.91.7`, then `v0.93` WP-S6. |
| Mitigate | Mitigation linkage artifact family and demo mitigation step exist. | `bounded_contract_and_demo_present` | `#4024` owns the residual for runtime-integrated mitigation orchestration in `v0.91.6`; if not closed, route it through `v0.91.7` residual readiness and `v0.93` WP-S6. |
| Verify | Continuous-verification contract and post-fix replay demo exist. | `bounded_contract_and_demo_present` | `#4024` owns the still-missing always-on integrated verification loop in `v0.91.6`; unresolved operation must route through `v0.91.7` and then `v0.93` WP-S6. |
| Archive | Structured exploit/replay/promotion artifact families exist. | `bounded_contract_surface_present` | `#4024` owns the gap between the current artifact family and a real runtime security-corpus service; unresolved archive/corpus ownership routes through `v0.91.7` and then `v0.93` WP-S6. |
| Regression test | Focused contract tests exist and demo replay proves one local deterministic fixture. | `bounded_fixture_proof_present` | `#4024` owns the residual for governed regression-seed promotion in `v0.91.6`; unresolved follow-through routes through `v0.91.7` residual readiness and `v0.93` WP-S6. |

## Security corpus route

The current repository is strong enough to define the security-corpus shape
without pretending the corpus is operationally complete.

The bounded corpus family already has reviewer-facing shape across:

- exploit hypothesis artifacts;
- exploit evidence artifacts;
- exploit classification artifacts;
- replay manifests;
- mitigation linkage artifacts;
- exploit promotion artifacts;
- continuous-verification lifecycle artifacts;
- the flagship local adversarial review packet.

This is sufficient for `v0.91.6` to say the corpus is no longer only narrative.
It is **not** sufficient to claim that ADL already has a live, continuously
growing security corpus with autonomous red/blue ingestion and scheduling.

## Freedom Gate and no-bypass rule

Current CAV truth must preserve these boundaries:

1. Adversarial work remains posture-governed, target-bounded, and reviewer-visible.
2. Freedom Gate is not replaced by CAV; CAV exists in part to probe attempted
   bypass of governed decision boundaries.
3. No exploit attempt is legitimate without declared target scope, posture, and
   evidence-capture expectations.
4. No later packet may treat provider/model content, exploit evidence, or demo
   outcomes as execution authority.

## Findings and dispositions

1. CAV is no longer merely a narrative route in this repository. The repo has
   working bounded proof hooks and a flagship local adversarial demo that land
   structured exploit/replay/mitigation evidence.  
   Disposition: fixed by this packet's current-baseline accounting and should be
   consumed by `#4024`.

2. The repository still does not support the stronger operator desire of
   "fire up red and blue teams and let them keep finding bugs" as a general live
   integrated runtime capability.  
   Disposition: explicit residual owned immediately by `#4024`, then by
   `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
   until it is consumed by
   `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`.
   Do not overclaim this issue as closure for the future integrated red/blue loop.

3. The issue-local task bundle still references the legacy local source-path
   strings ".adl/docs/TBD/security/ADL_THREAT_MODEL.md" and
   ".adl/docs/TBD/security/CONTINUOUS_ADVERSARIAL_VERIFICATION.md" in
   `.adl/v0.91.6/tasks/issue-4064__wp07-cav-threat-taxonomy-corpus-route/stp.md`,
   but those files are not present in the current repo. The tracked source of
   truth is the `v0.91.5` CAV packet set plus `docs/security/THREAT_MODEL_v0.7.md`.  
   Disposition: remediation residue owned immediately by `#4024`, with
   unresolved prompt/card cleanup routed into
   `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
   instead of pretending those paths still exist indefinitely.

## Consumption rule for v0.91.6 and v0.92

Current decision:

- `cav_taxonomy_and_corpus_route_reviewed_with_bounded_proof_hooks_verified`

That means later milestone work may consume:

- the ten threat classes above as explicit activation-path security vocabulary;
- the bounded exploit/replay/mitigation/promotion artifact family;
- the runnable identity proof hooks for adversarial runtime, red/blue
  architecture, adversarial runner, exploit replay, and continuous verification;
- the flagship local adversarial demo as proof of a bounded reviewer-facing
  exploit loop under local/no-network/no-live-target constraints.

It may **not** consume this issue as proof of:

- autonomous continuous red/blue bug hunting;
- a full security corpus service;
- external-target adversarial testing;
- completed integrated runtime adversarial-operations work;
- complete WP-07 closure before `#4024`.

## Residual routing

- `#4024` must consume this packet and preserve the distinction between working
  bounded proof hooks and the still-missing integrated runtime bug-finding loop.
- If `#4024` cannot close the integrated-runtime gap or the stale source-path
  cleanup inside `v0.91.6`, it must route both explicitly into
  `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`.
- `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`
  is the named integrated implementation/provenance target for bringing the
  bounded CAV surfaces into the main ADL runtime.
- Scaled always-running red/blue security tournaments remain future work beyond
  the bounded `v0.91.6` bridge.

## Reviewer takeaway

`#4064` is ready when reviewers can confirm that:

- `v0.91.6` now has an explicit CAV taxonomy/corpus route instead of a vague
  future promise;
- the current repo really does run bounded adversarial proof hooks and a local
  exploit/replay/mitigation demo;
- the packet does not confuse those bounded proof surfaces with the stronger
  future capability of autonomous red/blue teams continuously finding bugs;
- the stale source-path drift remains visible as remediation residue rather than
  hidden truth drift.
