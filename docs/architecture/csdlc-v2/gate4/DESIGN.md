# C-SDLC v2 Gate 4 design

Gate 4 retains the Parallel Validation Fabric, scheduler, and shepherd ideas while separating their authority. `csdlc-validate` validates and executes a typed DAG. `csdlc-schedule` only reports eligible operations and blockers. `csdlc-shepherd` only classifies observed state. None depends on ADL or Runtime.

## PVF manifest

Every lane declares an id, proof role, purpose, determinism posture, CPU/memory/token cost, credential names, network policy, dependency ids, parallel group, release-gate status, local/deferred-CI execution mode, timeout, executable, argv, and bounded evidence policy. Validation rejects duplicate or missing ids, unknown dependencies, self-dependencies, cycles, undeclared network/credential authority, and waves exceeding the execution budget before commands run.

Commands are passed directly to `std::process::Command`; the control plane never interprets a shell string. Independent lanes are deterministically packed by parallel group and CPU/memory limits after whole-DAG token/resource preflight. Dependent waves wait for successful predecessors; deferred and non-goal truth propagates without executing an invalid dependent. Local results converge through an explicit disposition lattice.

Lane identifiers are restricted to safe filename characters before execution. Logs are drained concurrently to prevent pipe deadlock, retained only to the byte limit, stored behind relative references, and scrub declared sensitive values. Structured argv evidence is scrubbed by the same policy. Each command runs in a process group; timeout, an external cancellation marker, or a failed peer terminates the group so descendants do not survive.

## Authority

- Validator: may execute only manifest-declared commands and write local evidence.
- Scheduler: pure eligibility report; cannot claim, execute, publish, merge, or close.
- Shepherd: pure ready/waiting/retryable/repair/operator classification; cannot edit or acquire workflow authority.

## Performance

The focused Gate 4 tests use local deterministic executables and complete in well under one second warm. The complete standalone workspace remains the only build/test surface. Live network and cross-product proof are explicit optional lanes, not default validation.
