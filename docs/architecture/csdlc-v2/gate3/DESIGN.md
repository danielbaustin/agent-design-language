# C-SDLC v2 Gate 3 design

Gate 3 adds repository binding without coupling the standalone C-SDLC workspace to ADL or Runtime. `csdlc-init` accepts one typed bootstrap request and atomically creates the canonical issue record and all six card projections. `csdlc-bind` accepts one typed bind request and uses a small `std::process::Command` Git adapter whose inputs are executable argument arrays; it never evaluates a shell string.

## Invariants

- The primary checkout must be on the declared base branch, and the issue branch must differ from both `main` and the base.
- Worktree and protected paths are normalized repository-relative paths.
- An exact branch/worktree/claim match is an idempotent success. A branch at another path, a path on another branch, a different active claim, or overlapping protected paths fails closed.
- Claims carry owner, generation, acquisition, expiry, heartbeat, branch, worktree, protected paths, and purpose.
- Heartbeat and recovery are compare-and-swap operations. Recovery requires observed lease expiry plus the expected claim id and generation. A missed heartbeat alone grants no authority.
- Recovery audit evidence retains previous owner, observed expiry, recovery actor, and reason.
- Initialization serializes claim reservation through one repository binding lock. Generated design/diagram placeholders remain pending and block readiness; only an explicitly supplied reviewed design may be approved.
- Exact initialization idempotency compares a digest of the complete typed request, including card inputs and reviewer truth.

## Transaction boundary

Card and claim truth remain in the Gate 2 atomic issue record. Git worktree creation occurs only after claim reservation and all policy/collision checks pass. Successful binding records the `Ready -> Bound` transition. A failed state compare-and-swap compensates a newly created worktree and branch; a subsequent exact invocation observes and reuses both topology and bound state. Git failure returns typed failure and never fabricates record success.

## Validation budget

Focused tests create local temporary Git repositories and cover creation, idempotent reuse, unsafe-primary refusal, CAS heartbeat refusal, and premature recovery refusal. They require no network, ADL crate, Runtime crate, shell control plane, or Python. The warm focused suite target is under two minutes; init/bind planning and application target p95 is under two seconds in a temporary repository.
