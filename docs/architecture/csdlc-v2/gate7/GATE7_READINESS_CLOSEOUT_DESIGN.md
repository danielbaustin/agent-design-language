# Gate 7: Readiness and Closeout

## Decision

Gate 7 completes the native v2 lifecycle with one standalone binary,
csdlc-closeout, and one deterministic policy module, readiness.rs.
Octocrab observes the PR, checks, reviews, mergeability, and terminal state.
The policy module classifies those observations and the store atomically
projects accepted truth into the index and SOR.

The scheduler and shepherd remain classifiers, not merge authorities. Gate 7
does not merge a PR. It says whether observed remote and local proof is ready,
and after a separately authorized merge or closure it verifies and records the
terminal result.

## Readiness model

Required check names are declared inputs. Every observed check is classified
as required or optional, and its conclusion is closed and typed. Missing and
unknown required checks remain pending; skipped or neutral required checks are
not green. Optional skipped or deferred work stays visible but cannot be
silently promoted into required success.

Remote review and conflict state are independent. A green check set cannot
hide a missing approval or unknown mergeability. Post-publication requested
changes are stored as routed findings in ReadinessEvidence; they do not
replace or rewrite the mandatory exact-revision pre-publication ReviewEvidence.

~~~mermaid
flowchart LR
    GH["Octocrab PR observation"] --> N["Typed normalization"]
    Policy["Required checks + review policy"] --> N
    N --> C["Required check classifier"]
    N --> R["Review classifier"]
    N --> M["Conflict classifier"]
    N --> F["Post-publication findings"]
    C --> G["Readiness conjunction"]
    R --> G
    M --> G
    F --> G
    G -->|all observed green| A["Atomic MergeReady + SOR"]
    G -->|pending/failing/unknown| W["Waiting/blocked evidence only"]
~~~

## Closeout model

Closeout accepts only one typed terminal disposition:

- merged: an actual PR, merged state, and observed head SHA are mandatory;
- closed_unmerged: an actual closed PR is mandatory;
- closed_no_pr: no PR may exist and an explicit approval reason is mandatory.

The PR identity must match publication evidence. Merged closeout is legal only
from MergeReady. Intentional non-merged termination is legal only from the
reviewed/published tail. Successful closeout updates the SOR and index in one
transaction, writes terminal evidence, releases the claim and protected paths,
and records the release in the audit stream. Repeating the exact terminal
observation returns the canonical record without another generation.

~~~mermaid
stateDiagram-v2
    Published --> MergeReady: required checks + review + clean conflict
    MergeReady --> Merged: GitHub says merged
    Merged --> ClosedOut: atomic terminal reconciliation
    Published --> ClosedOut: observed intentional closed-unmerged
    Reviewed --> ClosedOut: approved no-PR disposition
    ClosedOut --> ClosedOut: exact idempotent repeat
~~~

## Pruning boundary

Terminal truth and cleanup eligibility are separate. validate-prune requires
ClosedOut, terminal evidence, and a claim-release audit. The library's prune
surface guard additionally checks exact branch/worktree topology and a clean
Git status. Dirty, mismatched, non-terminal, or ambiguous surfaces fail
closed. Actual removal is a typed Git worktree operation owned by the operator
or the later cutover adapter, after the durable receipt has been copied to the
shared repository state.

~~~mermaid
flowchart TD
    T["Terminal GitHub observation"] --> V{"Matches publication?"}
    V -->|no| Stop["Fail closed"]
    V -->|yes| Commit["Atomic SOR/index/claim release"]
    Commit --> E["Durable terminal receipt"]
    E --> Topology{"Exact worktree + branch?"}
    Topology -->|no| Stop
    Topology --> Clean{"Git status clean?"}
    Clean -->|no| Stop
    Clean -->|yes| Eligible["Prune eligible"]
~~~

## Failure and proof posture

Remote transport failures and incomplete GitHub fields produce typed
non-terminal errors. Tokens are memory-only and unavailable sources are
reported without contents. The default test lane remains offline: the compact Gate
7 proof set covers required/optional distinctions, pending/failing/unknown
behavior, requested-change retention, false-terminal refusal, no-PR approval,
and schemas. The existing lifecycle suite supplies the atomic card/store
regression surface; live GitHub observation is bounded integration proof.
