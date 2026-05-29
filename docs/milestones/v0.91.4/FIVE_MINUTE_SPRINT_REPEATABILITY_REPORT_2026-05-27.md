# v0.91.4 Five-Minute Sprint Repeatability Report

## Status

Tracked WP-10 metrics/proof packet.

## Scope

This report records bounded repeatability evidence for recent ADL transitions
and defines the first v0.91.4 Parallel Validation Fabric posture.

It does not claim:

- that every end-to-end ADL transition completes in five minutes
- that broad validation tails have already been eliminated
- that pre-PR evidence reuse is implemented
- that WP-10 alone produces new signed-trace or ObsMem-ingestion artifacts

## Evidence Sources

- `#3420` / `CF-PRE-03` CodeFriend infrastructure publication PR metadata
- `#3427` CodeFriend rapid website-launch demo write-up PR metadata
- `#3440` `WP-09` sprint-conductor hardening PR metadata
- `docs/milestones/v0.91.4/features/FIVE_MINUTE_SPRINT_REPEATABILITY.md`
- `docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md`
- Sprint 3 state artifact at `.adl/reviews/sprint-3357-state.json`

## Measured Transition Sample

The durable evidence currently available without inventing local stopwatch data
is GitHub PR timing plus the tracked issue proof shape.

| Issue / PR | Surface | Commit authored | PR opened | PR merged | Prep-to-open | Open-to-merge | Validation / proof shape | Takeaway |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `#3375` / `#3420` | Real infra/product-sidecar lane | `2026-05-27T18:24:07Z` | `2026-05-27T18:24:22Z` | `2026-05-27T18:31:58Z` | `15s` | `7m36s` | Terraform/apply and live HTTPS verification; real AWS substrate work | Substantive product/infrastructure work can still publish quickly when the proof surface is narrow and the substrate is already reachable. |
| `#3423` / `#3427` | Docs/process lane | `2026-05-27T18:41:17Z` | `2026-05-27T18:41:21Z` | `2026-05-27T18:43:00Z` | `4s` | `1m39s` | Docs-only issue with bounded review and focused doc-truth checks | Docs/process transitions can complete well inside a five-minute claim envelope when no broad validation tail is required. |
| `#3358` / `#3440` | Core C-SDLC tools lane | `2026-05-27T19:58:31Z` | `2026-05-27T19:58:34Z` | `2026-05-27T20:34:28Z` | `3s` | `35m54s` | Focused sprint-conductor helper proof passed quickly; default publish lane widened into monolithic Rust validation and nextest | The local issue work can be quick while the validation tail remains long. Counting the whole tail as “five-minute sprint” would be misleading. |

## What The Sample Proves

1. Bounded ADL transition coordination can be consistently fast.
   Short commit-to-PR-open intervals across different issue classes show that
   binding, review, and publication setup can stay within a narrow execution
   loop.

2. Validation tail remains the dominant source of wall-clock variance.
   `#3440` is the clearest example: the issue-local proof was a focused helper
   regression script, but the default finish path attempted a much broader Rust
   lane before publication. That validation tail was real work, but it was not
   the same thing as the issue-local transition itself.

3. The five-minute claim must be scoped.
   The truthful claim is:
   - ADL can often move a bounded issue from active work to publish-ready in a
     short interval
   - end-to-end merged proof time may be much longer when broad or remote proof
     lanes remain blocking

4. Signed trace and ObsMem remain milestone-level evidence dependencies.
   WP-10 uses tracked PR metadata, issue records, and sprint-state evidence to
   describe repeatability and validation-tail posture. It does not claim to
   replace the signed-trace and ObsMem-derived evidence expected elsewhere in
   the milestone tail.

## Parallel Validation Fabric: First Bounded Posture

WP-10 does not ship a distributed validation system. It ships the first
truthful lane model for one.

| Lane | Owner | Blocking semantics | Cache / reuse posture | Current v0.91.4 posture |
| --- | --- | --- | --- | --- |
| Focused issue-local proof | active issue branch/worktree | blocks `pr finish` for the issue | no reuse by default | landed and required |
| Docs-only truth / consistency proof | active issue branch/worktree | blocks docs-only publication | no reuse by default | landed and required |
| Broad Rust / integration lane | repo-wide quality / runtime surfaces | should block only when the touched surface genuinely requires it | reuse not yet implemented | still too monolithic in some publish paths |
| GitHub CI merge gate | remote PR checks | blocks merge, not local issue execution | exact reuse proof only, otherwise rerun | landed as merge gate |
| Pre-PR evidence reuse | future cache-aware proof lane | may satisfy CI only with exact identity proof | planned via `#3437` and future PVF implementation | planned, not landed |
| Deferred or follow-on proof | owning issue or follow-up issue | does not block the current issue when explicitly marked deferred | not applicable | must remain visible in `SRP`, `SOR`, and sprint state |
| Sprint closeout truth | sprint umbrella / closeout path | blocks sprint advancement or closure | not a cache target | landed via `#3358` hardening |

## Current PVF Rules

- Passed proof, pending proof, deferred proof, blocked proof, and failed proof
  must remain distinguishable.
- Aggregate success must not hide an unfinished or failed lane.
- Merge-time CI remains a separate blocking lane from issue-local focused proof.
- Evidence reuse is a future cache-aware lane, not a waiver.
- Sprint state and closeout truth are themselves blocking proof lanes for sprint
  advancement.

## Operational Interpretation

The current repeatability story is strong enough to support milestone proof, but
only with careful wording:

- good claim: “bounded issue coordination is repeatable and often fast”
- good claim: “validation-tail and proof-latency are now explicit tracked
  concerns”
- bad claim: “the whole process is always five minutes”

## Recommended Reporting Vocabulary

When future issues report sprint timing, separate at least:

- coordination / execution time
- focused issue-local proof time
- publication setup time
- merge-gate / remote CI time
- deferred or follow-on proof still outstanding

## Relationship To WP-14

WP-14 should explicitly check that:

- no issue treats a long blocking proof tail as invisible
- no aggregate “PASS” hides pending, deferred, or blocked proof lanes
- pre-PR evidence reuse remains fail-closed until exact identity proof exists

## Conclusion

WP-10 proves the shape of a truthful five-minute sprint claim:

- fast bounded transition work is real
- long validation tails are also real
- Parallel Validation Fabric exists first as a proof taxonomy and reporting
  discipline before it becomes richer automation
