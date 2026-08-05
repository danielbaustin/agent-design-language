# v0.92 Feature and Proof Coverage

This planning matrix connects milestone outcomes to their owning work packages
and expected proof. Status remains `planned` until exact-revision evidence is
linked by the owning issue.

| Outcome | Owner | Planned proof | Status |
| --- | --- | --- | --- |
| Canonical milestone and version truth | WP-01, WP-01B | Issue graph, six-card inventory, docs/version parity | active |
| Agent Logic repository migration | WP-02 | Transfer manifests, preserved-surface checks, integration verification | planned |
| Reliable CI and coverage | WP-02A | Lane-selection regressions, coverage aggregation, platform checks | planned |
| Evidence-based build acceleration | WP-02B | Same-SHA standard/16-core trials, proof parity, canary, cost decision, and fallback or cleanup | planned |
| Resilient local Runtime | WP-03 | Start, stop, recovery, configuration, clean-log, and failure injection proof | planned |
| Distributed Guardian/polis | WP-04 | Architecture/security review and 16-issue integrated distributed proof | planned |
| Faster C-SDLC and remote validation | WP-05, WP-06, WP-07 | Cycle-time comparison, portable runner proof, typed-card parity | planned |
| Birthday and identity | WP-08, WP-09, WP-10 | Birth negative cases, stable identity, bounded-cycle continuity | planned |
| Memory and capability | WP-11, WP-12 | Grounded/redacted memory and capability-envelope validation | planned |
| Cognitive profile and adaptation queue | WP-13, WP-13A | Evidence-grounded profile fixtures and current Runtime v3 loop qualification | planned |
| ACIP/A2A transport | WP-14 | Reconciled contracts, protobuf/JSON parity, authenticated full-duplex WSS | planned |
| Witness, receipt, and review packet | WP-15, WP-16 | Witness/receipt validation and integrated reviewer packet | planned |
| Cross-polis continuity | WP-17 | Migration semantics and explicit infrastructure non-goals | planned |
| Demonstrable birthday | WP-18 | Runnable positive and negative birthday proof | planned |
| Observatory and Unity consumers | WP-18A | Real versioned API/WSS interactions, compatibility matrix, and consumer failures | planned |
| Provider-neutral multi-agent execution | WP-18B | Real multi-provider runs, ACIP traces, negative cases, and no-substitution proof | planned |
| v0.93 governance handoff | WP-19 | Traceable downstream evidence map | planned |
| Reduction and refactoring | WP-20, WP-21, WP-21A | Deletion eligibility, net reduction, behavior-preserving Rust checks | planned |
| Quality, release, and publication | WP-22 through WP-30 | Review packets, remediation, release evidence, articles, podcasts, ceremony | planned |

## Coverage Rule

An outcome becomes covered only when its owning issue links exact-revision
implementation, validation, review, and integration evidence. An open issue or
initialized card bundle proves scheduling, not delivery.

WP-22 must fail the quality gate if any product-feature row remains `planned`,
has no accepted exact-revision evidence, or relies on fixtures, receipts,
synthetic success, or provider substitution. WP-25 internal review may begin
only after every feature row is `landed` or an explicit reviewed milestone
scope amendment removes it from v0.92.
