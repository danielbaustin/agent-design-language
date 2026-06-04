# v0.91.5 Helper Binary Candidate Review

Issue: #3614
Umbrella: #3592
Captured: 2026-06-04
Status: complete_no_implementation_selected

## Purpose

This packet evaluates the helper binary candidates deferred by the first CLI
ownership split: `adl-crypto`, `adl-godel`, and `adl-identity`.

The goal is not to create more binaries by default. The goal is to decide
whether any candidate is ready to leave `adl-runtime` now, or whether keeping
the three-owner split is still the safer architecture.

## Verdict

Do not implement `adl-crypto`, `adl-godel`, or `adl-identity` in v0.91.5 Sprint
1.

Keep all three surfaces under `adl-runtime` for now:

- `adl-runtime keygen`
- `adl-runtime sign`
- `adl-runtime verify`
- `adl-runtime godel ...`
- `adl-runtime identity ...`

This is the safest answer because the first CLI split has only just established
three durable owner binaries (`adl-csdlc`, `adl-runtime`, `adl-review`). Adding
helper binaries now would increase generated-card policy, packaging, CI, and
operator-documentation surface before the runtime ownership boundary has paid
down its current speed and observability debt.

## Source Evidence

Reviewed surfaces:

- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
- `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md`
- `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md`
- `docs/milestones/v0.91.5/VALIDATION_LANE_SPLIT_3610.md`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `adl/src/cli/mod.rs`
- `adl/src/cli/commands.rs`
- `adl/src/cli/godel_cmd.rs`
- `adl/src/cli/identity_cmd.rs`
- `adl/src/signing.rs`
- `adl/src/bin/adl_runtime.rs`
- `adl/src/bin/adl_csdlc.rs`
- `adl/src/bin/adl_review.rs`

## Candidate Disposition

| Candidate | Current owner | Disposition | Rationale | Revisit gate |
| --- | --- | --- | --- | --- |
| `adl-crypto` | `adl-runtime keygen/sign/verify` | Defer. Keep under `adl-runtime`. | The command surface is small, but security-sensitive. A separate binary would create new generated-card and packaging policy before threat-model and sunset policy are ready. | Revisit after `#3615` defines compatibility sunset policy and after a crypto/signing threat-model or security-review packet is requested. |
| `adl-godel` | `adl-runtime godel ...` | Defer. Keep under `adl-runtime`. | Gödel mechanics are v0.92 activation-facing and artifact-heavy. Splitting before activation ownership and proof posture are indexed would make review harder, not easier. | Revisit after `#3623` creates the runtime-v2 feature navigation registry and v0.92 activation planning names Gödel proof lanes. |
| `adl-identity` | `adl-runtime identity ...` | Defer. Keep under `adl-runtime`. | Identity has many proof hooks and existing docs/fixtures still use legacy `adl identity ...` wording. A new binary would fossilize another command family before public prompt/card migration catches up. | Revisit after Sprint 1 card rewrite/public prompt packet work and after `#3615` defines generated-card deprecation handling. |

## Candidate Analysis

### `adl-crypto`

Evidence:

- `adl/src/cli/commands.rs` implements `real_keygen`, `real_sign`, and
  `real_verify`.
- `adl/src/signing.rs` contains the Ed25519 key generation, signing, canonical
  envelope, verification profile, and stable failure-class logic.
- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` explicitly says
  `adl keygen`, `adl sign`, and `adl verify` should stay under
  `adl-runtime keygen/sign/verify` for now and that generated cards should not
  introduce `adl-crypto` yet.

Assessment:

`adl-crypto` is attractive as a future security boundary, but premature as a
v0.91.5 mini-sprint change. The signing surface is not just a helper command;
it participates in runtime execution trust, trace verification, and remote
execution security. A separate binary should come with a threat-model/security
review packet and compatibility sunset plan, not a quick alias.

Decision:

No implementation issue now. Keep `adl-runtime keygen/sign/verify` as the
owner command family.

### `adl-godel`

Evidence:

- `adl/src/cli/godel_cmd.rs` owns `run`, `evaluate`, `inspect`, and
  `affect-slice` command routing.
- `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md` includes
  `adl-runtime godel ...` in the runtime ownership contract.
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md` routes
  runtime-v2 feature ownership indexing to `#3623` before deeper runtime code
  movement.

Assessment:

Gödel command behavior is runtime-feature activation work, not an isolated
helper utility. The right next step is not a binary split. The right next step
is to make runtime-v2 and v0.92 activation ownership clear enough that later
agents know which Gödel surfaces are proving, demoing, or merely scaffolding.

Decision:

No implementation issue now. Keep `adl-runtime godel ...` as the owner command
family and revisit only after runtime feature navigation is established.

### `adl-identity`

Evidence:

- `adl/src/cli/identity_cmd/dispatch.rs` routes `init`, `show`, `now`, and many
  identity/proof-hook subcommands.
- `adl/src/cli/identity_cmd/` is already internally decomposed into dispatch,
  profile, contracts, helpers, and tests.
- Existing feature and proof docs still contain legacy `adl identity ...`
  command examples, so generated-card and docs migration policy must be settled
  before introducing another public command family.

Assessment:

Identity is a strong future boundary, but it is also deeply tied to continuity,
chronosense, v0.92 activation readiness, and public prompt/review evidence.
Splitting it now would create one more command-family truth while Sprint 1 is
still migrating card/template behavior.

Decision:

No implementation issue now. Keep `adl-runtime identity ...` as the owner
command family and revisit after card/template migration plus compatibility
sunset policy.

## Compatibility And Generated-Card Policy

Generated cards and skills must not mention these commands yet:

- `adl-crypto`
- `adl-godel`
- `adl-identity`

Allowed command truth for this milestone remains:

- C-SDLC issue work: `adl/tools/pr.sh run <issue>`
- Runtime workflow and helper surfaces: `adl-runtime ...`
- Review surfaces: `adl-review ...`

If old examples still use legacy `adl keygen`, `adl sign`, `adl verify`,
`adl godel`, or `adl identity`, they should be handled through the compatibility
sunset work in `#3615`, not by inventing helper binaries.

## Follow-On Routing

No helper-binary implementation issue is opened from this review.

Existing live routes are sufficient:

| Route | Why it covers the need |
| --- | --- |
| `#3615` | Defines shim deprecation and compatibility sunset policy before generated cards can move again. |
| `#3623` | Creates runtime-v2 feature navigation truth before Gödel or identity-like runtime surfaces split further. |
| `#3625` | Handles long-lived agent/run-loop extraction separately from helper binary naming. |
| `#3556` / `#3609` | Cover deterministic runtime/control-plane logging and OTEL-ready observability before more binaries make logging surface wider. |

If a future operator selects a helper binary after those routes land, open a
fresh implementation issue with:

- old/new command equivalence table;
- generated-card policy update;
- compatibility warning text;
- focused owner-lane validation;
- rollback plan;
- explicit sunset milestone.

## Validation

Validation for this issue:

| Command | Purpose |
| --- | --- |
| `rg -n "adl-crypto|adl-godel|adl-identity" AGENTS.md docs/templates/prompts docs/milestones/v0.91.5 adl/src` | Confirm helper binary names exist only in planning/review context, not as implemented generated-card truth. |
| `git diff --check` | Patch hygiene. |
| `bash adl/tools/validate_structured_prompt.sh --type srp --input <srp>` | SRP lifecycle-record proof. |
| `bash adl/tools/validate_structured_prompt.sh --type sor --input <sor>` | SOR lifecycle-record proof. |

No Rust tests are required because this issue does not change Rust behavior,
command dispatch, help text, or compatibility shims.

Scan result:

- `adl-crypto`, `adl-godel`, and `adl-identity` appear in v0.91.5 planning and
  this review packet as deferred candidates.
- `adl-godel-*` also appears as internal temp-directory prefixes in Gödel tests
  and helper code. Those are not public command strings.
- No root `AGENTS.md` or prompt-template guidance teaches `adl-crypto`,
  `adl-godel`, or `adl-identity` as implemented commands.

## Non-Claims

- This packet does not implement or approve `adl-crypto`.
- This packet does not implement or approve `adl-godel`.
- This packet does not implement or approve `adl-identity`.
- This packet does not remove legacy `adl` command compatibility.
- This packet does not claim runtime observability or workspace splitting is
  complete.

## Residual Risk

The main residual risk is that helper-binary names are appealing shorthand and
could appear in future cards or docs before they exist. `#3615` should treat
that as generated-card policy debt: durable workflow state should only name
implemented commands or explicitly marked future candidates.
