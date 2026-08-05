# Issue 5352 Design

## Boundary

Issue #5352 publishes the final WP-21 exact-revision consumption handoff. It
does not implement v0.92 features, claim that the birthday occurred, run AWS or
Unity work, alter #5713/#5733, or perform typed closeout.

Tracked work is confined to `/Volumes/FastWork/adl-wp-5352-final` on
`codex/5352-v0918-final-handoff`. The active claim protects only issue-local
C-SDLC state/evidence and `docs/milestones/v0.91.8/handoff`.

## Exact Baseline

The handoff records `origin/main` at
`c34f0c9412495039a6374f7ce88fa39e34bb5042`. Validation fails if the live
tracking ref differs. Every accepted platform, WP-20, and WP-21 merge must be
an ancestor of that exact baseline.

## Truth Model

- Every platform and WP-21 table row binds one concern/product to an exact
  issue/PR pair, reviewed head, and accepted merge revision.
- Token presence is insufficient. The validator parses the Markdown rows and
  rejects missing, duplicated, or substituted identities.
- #5558 is closed by PR #5749 at merge
  `c34f0c9412495039a6374f7ce88fa39e34bb5042`.
- All eight WP-21 children are merged and closed at the revisions named by the
  handoff.
- Local install receipts are audit evidence only. They never release or block
  this issue.

## Validation

Four focused lanes prove the publication candidate:

1. `validate_handoff.rb --final` checks the exact baseline, row bindings,
   repository references, schemas, rollback boundaries, and non-claims.
2. `validate_dependency_ancestry.rb --final` checks every accepted merge
   against the recorded baseline and verifies the WP-20 dependency declaration.
3. `validate_implemented.rb` checks current lifecycle phase, claim, protected
   paths, cards, and absence of superseded baselines.
4. `git diff --check` rejects whitespace defects.

Exactly one GPT-5.5 pre-PR review evaluates the clean exact candidate. After
publication, a full findings-first WP-21 sprint review covers all nine issues
before #5352 may merge.

## Publication

The PR targets `main`, includes `Closes #5352`, and contains only the handoff,
focused validators/evidence, and issue-local lifecycle state. Required CI must
pass before merge. GitHub issue closure follows the merge; typed closeout is a
separate asynchronous process and is not part of this lane.

## Rollback And Non-Claims

The handoff retains the WP-14A rollback window, reversible ADL v2 selector
evidence, Runtime v3 continuity boundary, and `deletion_authorized: false`.
Launch documents, witness artifacts, and Adaptive Learning planning remain
planning/evidence surfaces; they do not prove an observed birthday, production
deployment, or learning-driven runtime behavior.
