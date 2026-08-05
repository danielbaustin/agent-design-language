# Structured Task Prompt

Template: 1.0.0

Issue: 5846

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver internal review report and finding register.

## Deliverables

- internal review report and finding register
- internal review packet

## Acceptance

1. AC-1: WP-23, WP-24, and WP-24A are merged, terminal, claim-free, ancestral, and the review target/manifest are pinned to one exact SHA.
2. AC-2: The packet explicitly inventories included, excluded, unknown, local-only, and redacted code, docs, tests, evidence, issues/PRs, demos, launch assets, and release surfaces.
3. AC-3: Independent specialist lanes cover correctness, architecture, tests/PVF/CI, security/privacy, dependencies, docs/claims, lifecycle/evidence, demos/integration, and release/publication.
4. AC-4: Every finding has stable ID, evidence, severity, invariant/failure mode, reproduction or proof gap, owner route, and open disposition; duplicates and disagreements remain traceable.
5. AC-5: Packet digest, revision identity, source coverage, evidence links, redaction/private-path/secret hygiene, and specialist completion validate or the review is blocked.
6. AC-6: A bounded independent meta-review finds no actionable review-quality gap and the result does not claim remediation, external approval, or release readiness.

## Dependencies

- WP-23
- WP-24
- WP-24A

## Inputs

- Passing WP-22 gate; terminal WP-23, WP-24, and WP-24A outputs
- Complete v0.92 source, docs, feature, demo, publication, evidence, issue/PR, typed lifecycle, and release corpus
- docs/milestones/v0.91.8/review/V0918_INTERNAL_REVIEW_PLAN_5356.md as format precedent only

## Non Goals

- Fixing findings, dispatching external review, or approving release
- Opening one issue per finding or suppressing specialist disagreement
- Crediting closed issues, receipts, articles, or podcasts as product acceptance without WP-22 evidence
