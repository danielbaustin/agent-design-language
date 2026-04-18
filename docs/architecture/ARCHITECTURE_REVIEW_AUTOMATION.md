# Architecture Review Automation

## Purpose

This document defines how ADL should generate and review architecture packets
without replacing human architecture judgment. The pipeline is designed for the
CodeBuddy-style review family and for ADL's own milestone review process.

## Review Pipeline

1. Build a bounded evidence packet.
2. Run architecture review against the packet.
3. Run docs, code, security, dependency, and test specialist reviews where
   relevant.
4. Ask the diagram planner for diagram briefs.
5. Ask the diagram author to create source-grounded diagrams.
6. Ask the architecture diagram reviewer to validate labels, edges, evidence,
   assumptions, and unsupported claims.
7. Run redaction and evidence-boundary review before publication.
8. Ask the fitness-function author to separate machine-checkable invariants
   from human judgment.
9. Synthesize findings into a product-grade or milestone-grade report.
10. Run review-quality evaluation before using the report externally.

## Machine-Checkable Invariants

These checks can be automated today or with small follow-up scripts:

- Required architecture packet files exist under `docs/architecture`.
- Diagram sources exist under `docs/architecture/diagrams`.
- `DIAGRAM_PACKET.md` lists every diagram source.
- Public architecture docs contain no host-absolute private paths or common
  secret markers.
- Architecture docs include evidence, assumptions, validation, residual risk,
  and known-gap sections.
- Demo matrix entries do not claim a command before the command exists.
- Issue work has STP, SIP, and SOR cards before `pr run`.
- Worktree-bound implementation is used for tracked issues.
- SOR closeout claims match GitHub issue and PR state.
- Provider docs keep model refs, transport ids, and policy decisions separate.
- Long-lived-agent docs describe cycle-scoped artifacts and operator controls.

## Human-Judgment Gates

These checks should remain explicitly human-reviewed:

- Whether an architecture finding severity matches user impact.
- Whether a diagram hides an important boundary or implies unsupported behavior.
- Whether a tradeoff deserves an ADR or only an implementation note.
- Whether a runtime capability is mature enough to be claimed publicly.
- Whether a demo is genuinely proving, merely illustrative, skipped, or failed.
- Whether publication is appropriate after redaction passes.

## Specialist Roles

- `repo-packet-builder`: gathers bounded evidence.
- `repo-architecture-review`: reviews layering, coupling, lifecycle, state, and
  architecture drift.
- `repo-review-security`: reviews trust boundaries, secrets, unsafe IO,
  privilege, injection, and abuse paths.
- `repo-review-tests`: reviews executable proof and missing coverage.
- `repo-review-docs`: reviews onboarding, command truth, stale docs, and
  overclaims.
- `repo-dependency-review`: reviews supply-chain and dependency drift.
- `repo-diagram-planner`: turns review evidence into diagram briefs.
- `diagram-author`: authors source-grounded diagram sources.
- `architecture-diagram-reviewer`: validates diagram truth.
- `redaction-and-evidence-auditor`: blocks unsafe publication.
- `architecture-fitness-function-author`: proposes checkable invariants and CI
  gates.
- `review-quality-evaluator`: checks quality against the third-party review bar.

## Missing Or Backlog Skills

- Documentation specialist: should maintain packet readability, navigation,
  examples, and command truth.
- Gap analysis skill: should compare architecture claims against code, tests,
  demos, issue state, and milestone docs.
- Architecture-document writer: may eventually own the synthesis step for docs
  like `ADL_ARCHITECTURE.md`, but should consume specialist outputs instead of
  inventing claims.

## Automation Boundaries

Automation may create draft packets, diagrams, issue candidates, and validation
reports. It must not silently accept ADRs, merge PRs, close issues, publish
customer reports, or claim release readiness. Those remain explicit operator
decisions.

## Suggested CI Gates

- Run `python3 adl/tools/validate_architecture_docs.py` when files under
  `docs/architecture`, `demos/README.md`, or the v0.90 demo matrix change.
- Run architecture packet validation before review publication.
- Run diagram renderer checks when renderer dependencies are available.
- Run a closeout truth gate after merge to ensure issue closure, final SOR
  truth, and worktree pruning are aligned.
