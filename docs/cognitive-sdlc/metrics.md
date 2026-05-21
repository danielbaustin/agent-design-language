# C-SDLC Metrics

## Status

Tracked metrics summary for C-SDLC proof and adoption planning.

## Purpose

C-SDLC must prove both throughput and governance quality. Speed alone is not a
success metric.

The key question is whether C-SDLC reduces coordination latency while
preserving review, replay, merge, closeout, and memory integrity.

C-SDLC should be evaluated with an Amdahl-style scaling lens. More agents or
more hardware help only when the serial coordination fraction is reduced and
the remaining work is decomposable, bounded, inspectable, and mergeable.
Typed work packets are therefore not administrative overhead; they are part of
the mechanism for reducing coordination entropy.

## Metric Families

### Throughput

- transition elapsed time
- issue-to-PR time
- review-ready time
- merge-ready time
- closeout time
- repeated-run variance

### Coordination

- serial fraction
- parallel shard count
- blocked time
- synchronization barrier count
- replan count
- cross-shard conflict count

### Governance

- unresolved findings at PR open
- unresolved findings at merge
- validation gaps
- stale card detections
- closeout truth corrections
- local-only durable record detections

### Evidence Quality

- evidence bundle completeness
- trace/signed-trace availability
- replay verification status
- artifact path portability
- review synthesis completeness

### Memory

- SRP findings ingested
- SOR outcome truth ingested
- trace/evidence references ingested
- future-agent recovery usefulness

## Five-Minute Sprint Measurements

The five-minute sprint target should be measured as a coordination experiment,
not a stunt.

The demo should record:

- baseline sequential estimate
- actual C-SDLC transition elapsed time
- number of shards
- number of synchronization barriers
- validation and review time
- defects or findings found before PR
- defects or findings found after PR
- closeout correctness

## Non-Claims

C-SDLC metrics do not claim:

- all software work should fit in five minutes
- speed is more important than governance
- more agents always means better throughput
- a single successful run proves repeatability
