# C-SDLC v2 Sprint Gap Analysis

## Baseline

The expected baseline is the combined acceptance surface of issues #5228,
#5232-#5240, #5292-#5295, and #5305-#5308; root and nested `AGENTS.md`; the
Gate 1 architecture and retained-behavior contracts; and Gate 9/10 cutover,
parity, deletion, and final-authority evidence.

## Findings

| Expected outcome | Observed implementation or evidence | Status | Review consequence |
| --- | --- | --- | --- |
| Final v2 authority is self-resolving through installed `csdlc-install resolve`. | The stable receipt has 11 binaries and omits `csdlc-install`; the final inventory accepts that omission. | contradicted | Must fix before the installed route can satisfy current operator policy. |
| Review evidence binds a reviewer's declared scope to the exact substantive revision. | Scope must be nonempty but does not constrain the Git digest or prove coverage of changed paths. | contradicted | Exact-revision review can overstate what was reviewed. |
| Review then publication has a recoverable, monotonic lifecycle. | Dirty review is accepted, clean publication is required, and Reviewed cannot be reassigned. | contradicted | Independently corroborates testing issue #5368; lifecycle can dead-end. |
| Binding and claim recovery preserve exclusive Git/path ownership. | Existing ordinary directories can be accepted as worktrees; recovery omits normal collision/topology and replacement-record guards. | contradicted | Canonical ownership can become false or corrupt. |
| Failed validation can be fixed and truthfully retried. | SOR is append-only and any historical failure permanently blocks readiness. | contradicted | Normal fail/fix/retry cannot converge without deleting evidence. |
| Merge readiness and closeout are bound to canonical publication identity and repository policy. | Repository/base/head/draft/open identity and required-check/review policy are caller-controlled or absent from committed observations. | contradicted | Unrelated or under-governed remote truth can advance lifecycle state. |
| Terminal SOR truth satisfies the same completion invariant as semantic card edits. | Terminal commit directly marks SOR complete and maps closed-unmerged integration to `closed_no_pr`. | contradicted | Closeout can persist invalid or misleading terminal truth. |
| PVF network, credential, redaction, and path policies are enforced properties. | They are mostly declaration checks; child processes inherit host environment/network and redaction is self-reported true. | contradicted | Determinism and secret-isolation claims exceed implementation proof. |
| Every current operator route and generated card names final v2 authority. | Active docs/templates still emit removed v1 commands; thin skills omit required resolver routing. | contradicted | New work can be generated into an unusable lifecycle. |
| All 18 closed issues retain truthful SRP/SOR and closeout evidence. | 108 local cards are ignored/untracked; many are stale or internally contradictory; umbrella/sunset closeout is unreconciled. | contradicted | GitHub closure is not backed by durable lifecycle truth. |
| Gate 10D2's 100% parity claim is executable and revision-bound. | Capability proof references are checked as nonempty strings, not resolved or executed mappings. | unproven | Central deletion precondition has weaker proof than its label implies. |
| Final size/test evidence is reproducible. | The suite executes 101 cases while evidence counts 100 annotations; the recorded LoC command text would count files, not lines. | contradicted | Budget values happen to match current physical LoC but their recorded method is not reproducible. |
| Public state mutation is repository-contained. | `.csdlc` ancestor symlinks are not rejected before lock, write, rename, or cleanup operations. | contradicted | Typed commands can write or remove predictable paths outside the checkout. |
| Explicit v1 selection is unavailable after `v1_sunset`. | The current resolver still accepts explicit v1 while final inventory forbids the binaries needed to execute it. | contradicted | Authority selection can produce a generation with no valid implementation. |

## Achieved Outcomes

- All 18 issues are closed and all 15 observed implementation/preparation PRs
  are merged.
- The independent Rust workspace remains separate from ADL and Runtime product
  crates at the manifest boundary.
- The current standalone suite, strict Clippy, and formatting pass.
- Gate 10D2 removed 94 of 95 pinned incumbent files and records 48,966 of
  49,979 pinned lines removed, while retaining the session ledger with an
  explicit justification.
- The current tracked selector is a regular file selecting v2, and the final
  coexistence inventory records `v1_sunset`.

## Disposition

The sprint delivered a compact, independently buildable v2 implementation and
completed the recorded cutover/deletion wave, but the review does not support
an unqualified "all acceptance criteria proven" conclusion. The final packet
must retain the findings above and distinguish implementation completion from
review acceptance. No remediation or new issue creation is authorized in
issue #5375.
