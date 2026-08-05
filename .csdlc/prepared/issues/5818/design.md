# WP-01B Canonical Documentation And Version Activation Design

## Outcome And Boundary

Issue #5818 makes v0.92 the truthful active-development version across current
repository entrypoints before product implementation starts. It updates current
indexes and version declarations; it does not rewrite historical milestone,
release, review, migration, or evidence artifacts.

The canonical feature inventory remains `docs/planning/ADL_FEATURE_LIST.md`.
The active milestone package under `docs/milestones/v0.92/` supplies planned
feature ownership and must not be converted into implementation-complete truth.
The latest completed release remains v0.91.8 until separate release authority
changes that fact.

## Source-Grounded Surface Inventory

The implementation session must classify every candidate as `update`,
`already_current`, `historical_preserve`, or `not_authoritative` in a retained
machine-readable inventory. Candidate current surfaces are:

- `README.md`, `docs/README.md`, and current package README entrypoints;
- `docs/planning/ADL_FEATURE_LIST.md` and current planning indexes;
- `docs/milestones/v0.92/` feature, quality, demo, review, and execution links;
- root and workspace `Cargo.toml` files, `Cargo.lock`, and user-visible package
  metadata that declare the current ADL version;
- `AGENTS.md`, `REVIEW.md`, `csdlc-v2/operator/skills/`, and current tooling
  runbooks where active lifecycle or milestone wording is exposed.

Historical directories are scan inputs but are protected from version rewriting.
Generated or vendored files are changed only through their owning generator.

## Execution Design

1. Build the checked-surface inventory before editing and identify each
   authoritative current-version declaration.
2. Update the canonical feature inventory and current documentation indexes,
   preserving `planned`, `active`, and `implemented` distinctions.
3. Update authoritative package/version declarations to `0.92.0`; regenerate
   `Cargo.lock` only through Cargo and inspect the resulting diff.
4. Repair current links and lifecycle language only where source evidence shows
   drift from the final C-SDLC v2 authority.
5. Run focused format, link, structured-data, Cargo metadata, version-parity,
   stale-reference, and historical-preservation checks.

The issue-local validator at
`.csdlc/prepared/issues/5818/validate-activation.rb` owns the deterministic
inventory, Markdown-link, YAML/JSON parse, version-parity, and historical-diff
checks. Historical preservation excludes only `.csdlc/evidence/5818/`, which
is new proof owned by this issue; it does not exempt any pre-existing evidence.

## Invariants And Failure Policy

- No v0.92 feature becomes complete merely because its documentation is active.
- Historical evidence retains its original version, dates, and claims.
- No product behavior, repository transfer, release ceremony, or child closeout
  occurs in this issue.
- A broad stale-reference scan is classification evidence, not permission for a
  repository-wide replacement.
- Stop on ambiguous version authority, generated-file ownership, or overlap with
  another active protected path.

## Rollback

The change is a single reviewed documentation/version activation commit. Revert
that commit if parity or historical-preservation checks fail; do not hand-edit
only part of a generated metadata set.

## Proof Design

The activation inventory has a fixed minimum denominator. It must include the
root and documentation READMEs, `docs/planning/ADL_FEATURE_LIST.md`, every
owned current-version manifest and lockfile for ADL v1/v2, Runtime, Runtime
Kernel, Resilience, Characterization, and C-SDLC v2, `AGENTS.md`, `REVIEW.md`,
all eleven current C-SDLC v2 operator skills, and the current
session-coordination and rescue-sprint runbooks. Additional package READMEs and
version declarations may be added, but the executor cannot shrink this minimum
set.

Cargo metadata proof runs from the repository root with
`--manifest-path adl/Cargo.toml`; the repository root is not itself a Cargo
workspace.

Proof consists of the retained surface inventory, deterministic version-parity
and historical-preservation checks, Markdown/link and YAML/JSON validation,
Cargo metadata plus locked check, diff hygiene, and an exact-revision bounded
review. Broad runtime tests are not required unless executable version behavior
is changed.
## Owned Paths

- `README.md`
- `docs/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `adl/README.md`
- `csdlc-v2/README.md`
- `AGENTS.md`
- `REVIEW.md`
- `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`
- `docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md`
- `csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-clean/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-shepherd/SKILL.md`
- `csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `adl-v2/Cargo.toml`
- `adl-v2/Cargo.lock`
- `adl-runtime/Cargo.toml`
- `adl-runtime/Cargo.lock`
- `adl-runtime-kernel/Cargo.toml`
- `adl-runtime-kernel/Cargo.lock`
- `adl-resilience/Cargo.toml`
- `adl-resilience/Cargo.lock`
- `adl-characterization/Cargo.toml`
- `adl-characterization/Cargo.lock`
- `csdlc-v2/Cargo.toml`
- `csdlc-v2/Cargo.lock`
- `.csdlc/evidence/5818`
- `.csdlc/prepared/issues/5818/validate-activation.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-docs-activation-final-truth-v1",
    "paths": [
      "README.md",
      "docs/README.md",
      "docs/planning/ADL_FEATURE_LIST.md",
      "AGENTS.md",
      "REVIEW.md"
    ],
    "issues": [
      5818,
      5843
    ],
    "order": [
      5818,
      5843
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-runtime-manifest-sequence-v1",
    "paths": [
      "adl-runtime/Cargo.toml",
      "adl-runtime/Cargo.lock"
    ],
    "issues": [
      5818,
      5865
    ],
    "order": [
      5818,
      5865
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5818-skills-to-final-doc-truth-v1",
    "paths": [
      "csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-clean/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-shepherd/SKILL.md",
      "csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md"
    ],
    "issues": [
      5818,
      5843
    ],
    "order": [
      5818,
      5843
    ]
  }
]
```
