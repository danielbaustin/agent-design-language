# WP-15 Integrated Demo Convergence Preparation Design

## Purpose

Prepare issue #5354 to demonstrate the deployed ADL v2, Runtime v3, and
C-SDLC v2 stack after WP-14A #5384 has completed integrated acceptance and
deployment. This preparation creates no demo implementation and makes no
release claim.

## Authority Boundary

- #5384 owns platform acceptance, deployment, and the v0.92 handoff gate.
- #5354 may consume only exact merged revisions and retained evidence accepted
  by #5384; it cannot repair, replace, or bypass that acceptance.
- #5354 owns the future integrated demo convergence packet, the claim-boundary
  matrix, and updates to the v0.91.8 demo/proof matrices.
- Preparation protects only `.csdlc` issue-local paths for #5354. Future demo
  and documentation paths require a typed claim amendment after #5384 is
  terminal and before any edit.
- Runtime v2, AWS, provider credentials, paid services, raw `gh`, root `main`,
  autonomous publication, and autonomous merge are outside this issue.

## Integrated Scenario Contract

The future proof must start from a fresh consumer context and use the stable
installed entrypoints accepted by #5384. It must:

1. validate and compile one declared ADL v2 document into a canonical plan;
2. execute that plan through the accepted Runtime v3 adapter and canonical
   ingress without calling Runtime v2;
3. observe the execution through the accepted secure local or remote Runtime
   access surface and retain deterministic result/checkpoint identity;
4. run the applicable C-SDLC v2 lifecycle operations through installed typed
   binaries and retain exact issue, claim, review, publication, and closeout
   boundaries without granting the demo lifecycle authority;
5. bind every displayed claim to exact accepted product revisions and classify
   unsupported, blocked, deferred, and non-applicable claims explicitly.

One script, screenshot, fixture, metadata row, or product-local test cannot
stand in for this integrated path. The retained packet must separate live
execution, deterministic fixture proof, operational observation, and planning
truth.

## Evidence Model

The future packet will bind:

- the #5384 terminal receipt digest and merged SHA;
- accepted ADL v2, Runtime v3, and C-SDLC v2 revision/provenance records;
- repo-relative commands and artifacts with SHA-256 digests;
- timestamps from the accepted time authority where operational ordering is
  claimed;
- per-step outcome, duration, retry count, and failure classification;
- a public claim-boundary matrix whose citations point to retained evidence;
- negative proof that missing, stale, substituted, or non-ancestral inputs fail
  closed.

No secret, token, credential path, host-absolute path, private transcript,
hard-coded network address, or unredacted payload may enter retained evidence.

## COTS And Reuse

No new dependency is authorized. The future demo must compose the accepted
ADL v2, Runtime v3, C-SDLC v2, Git, SHA-256, JSON/YAML parsers, and existing
repository validation/demo tooling. It must not build a second workflow engine,
runtime, TLS stack, signing system, telemetry pipeline, scheduler, or evidence
store. Any new harness code must be thin orchestration around those accepted
interfaces.

## Budgets

- Authored preparation design, diagram, request, and validation orchestration:
  at most 800 nonblank lines in total, each file below 500. Generated cards,
  typed state, PVF logs, and subagent review output are reported separately.
- Future demo harness and tightly coupled fixtures: at most 1,500 nonblank
  lines, fewer than 100 focused assertions, and no new dependency.
- Preparation and dependency gates: 120 seconds each.
- Integrated live proof: 900 seconds.
- Claim-boundary/matrix proof: 300 seconds.
- Complete and post-merge proof: 1,800 seconds each.
- Any variance requires exact-revision review before publication; the variance
  does not authorize deferred acceptance.

## Validation And Review

Preparation runs the deterministic preparation validator and confirms that the
#5384 gate currently refuses execution. After #5384 is terminal, every future
lane is mandatory at the applicable lifecycle stage: dependency admission,
integrated live proof, claim-boundary/matrix proof, complete validation,
exact-revision review, required CI, authorized serialized merge, post-merge
validation, and typed closeout.

## Stop Conditions

Stop without implementation, publication, or claims if #5384 is not merged,
typed `closed_out`, claim-free, receipt-backed, and ancestral; if accepted
product revisions cannot be resolved; if an interface requires Runtime v2 or
undeclared credentials; if protected paths collide; if retained evidence is
host-bound, stale, or secret-bearing; or if any required proof would be
deferred or represented only by prose/metadata.
