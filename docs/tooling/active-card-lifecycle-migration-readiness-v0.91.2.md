# Active Card Lifecycle Migration Readiness: v0.91.2

## Purpose

This report classifies the active `v0.91.2` issue-card bundles against the
target Cognitive SDLC lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

It is the safety pass for issue `#3067`. Its job is to prevent stricter
lifecycle enforcement from damaging active milestone truth.

## Scope

Evidence came from the primary-checkout local `v0.91.2` control-plane
surfaces:

- `.adl/v0.91.2/tasks/`
- `.adl/v0.91.2/bodies/`
- `adl/tools/pr.sh doctor`
- `adl/tools/validate_structured_prompt.sh`
- GitHub issue state for active `v0.91.2` issues

This report does not rewrite historical milestones and does not claim that
ignored local `.adl` records are publication artifacts. It records the active
local control-plane truth that agents rely on during execution.

The bound `#3067` worktree intentionally contains only the `#3067` local bundle.
The aggregate inventory therefore uses the root local control-plane path above,
not the worktree-local `.adl` mirror.

## Inventory Result

The active local inventory contains `57` `v0.91.2` task bundles.

All `57` bundles have the five expected card files:

- `sip.md`
- `stp.md`
- `spp.md`
- `srp.md`
- `sor.md`

No missing-card repair is required before the next migration stage.

## Lifecycle Classification

| Card | State counts | Readiness interpretation |
| --- | --- | --- |
| `SIP` | `31 complete`, `26 scaffold` | Expected for a mixed set of closed, active, and not-yet-started issues. Unopened work should keep scaffold SIP truth until `pr run` binds execution. |
| `STP` | `56 complete`, `1 active` | The selected-task surface is mostly healthy. The one active STP is the sprint umbrella and should remain sprint-scoped rather than forced into child-issue shape. |
| `SPP` | `1 complete`, `9 active`, `47 scaffold` | Planning-card rollout is incomplete in root local records. Branch-bound issues may have better worktree-local SPP truth than root closeout currently preserves. |
| `SRP` | `57 legacy_compatible` | This is the primary migration blocker. Legacy `Structured Review Policy` scaffolds validate structurally, but they are not final `Structured Review Prompt` truth. |
| `SOR` | `31 complete`, `26 scaffold` | Scaffold SOR is expected for open or unopened work. The refreshed pass found no closed issue whose root local SOR remains scaffold. |

## Closed-Issue SOR Drift

The refreshed pass found no closed GitHub issue whose root local SOR remains
scaffold.

That means `#3067` does not need to create a closed-issue closeout-repair wave.
If later scans find closed/scaffold SOR drift, those repairs should route
through `sor-editor` or `pr-closeout` with evidence for the final PR/closure
state.

## SRP Migration Blocker

Every active `v0.91.2` bundle currently remains `legacy_compatible` for `SRP`.

That means the repository can keep accepting legacy SRP scaffolds as a
temporary compatibility state, but stricter enforcement must not require final
SRP review-results truth for all active bundles yet.

The correct next move is issue `#3068`, which should harden:

- `SRP` as Structured Review Prompt
- review-results capture
- `SOR` closeout truth
- `ObsMem` handoff for final `SRP` and `SOR`
- explicit exceptions for bundles that cannot be migrated mechanically

The affected legacy-compatible SRP bundle set is:

```text
#2986, #2993, #3000, #3001, #3002, #3003, #3004, #3005, #3006,
#3007, #3008, #3009, #3010, #3011, #3012, #3013, #3014, #3015,
#3016, #3017, #3018, #3019, #3020, #3021, #3022, #3023, #3024,
#3025, #3026, #3027, #3028, #3029, #3031, #3032, #3035, #3036,
#3038, #3042, #3043, #3044, #3045, #3046, #3051, #3054, #3056,
#3059, #3060, #3063, #3064, #3065, #3066, #3067, #3068, #3069,
#3076, #3079, #3080
```

## Safe Mechanical Repair Candidates

No repo-wide mechanical card rewrite is safe in `#3067`.

The only safe changes in this issue are:

- classifying active bundle state
- documenting temporary detectable exception categories
- preparing the next issue to repair SRP/SOR semantics through editor skills

This preserves the ADL rule that cards are edited only with editor skills and
that issue truth is not rewritten by bulk text replacement.

## Temporary Detectable Exceptions

Until `#3068` lands, tooling and agents should treat these states as explicit
temporary migration states rather than final lifecycle truth:

- `SRP: legacy_compatible` means the card is structurally valid legacy review
  policy scaffolding, not final Structured Review Prompt results.
- `SOR: scaffold` on an open or unopened issue means execution has not produced
  final outcome truth yet.
- `SOR: scaffold` on a closed issue means closeout truth is missing and must be
  routed to `sor-editor` or `pr-closeout`.
- `SPP: scaffold` on an unbound issue means planning is not branch-bound yet.
- `SPP: scaffold` on a closed issue means root local records did not preserve
  final planning truth and should not be treated as a live execution blocker.

## Enforcement Recommendation

Stricter lifecycle enforcement can safely proceed for these checks now:

- all new issue bundles must include `SIP`, `STP`, `SPP`, `SRP`, and `SOR`
- `pr doctor` should continue distinguishing file existence from stage truth
- `SRP: legacy_compatible` must remain detectable and non-final
- `SOR: scaffold` on closed issues must be treated as closeout drift, not
  completion truth

Stricter lifecycle enforcement must wait for `#3068` before requiring:

- final SRP review-results truth across existing active bundles
- automatic SRP-to-ObsMem readiness
- automatic SOR-to-ObsMem readiness
- repo-wide closed-issue SOR truth normalization

## Follow-Up Routing

`#3068` is the required next issue for the remaining migration work.

It should either repair the remaining SRP/SOR drift through editor skills or
record explicit follow-up issues for any bundle that cannot be migrated safely.

No additional issue is required from this classification pass unless `#3068`
finds a bundle whose truth cannot be recovered from local cards, PR state, and
GitHub closure evidence.

## Bottom Line

The active `v0.91.2` card inventory is structurally complete but not fully
semantically migrated.

The mini-sprint can continue. The next safe gate is `#3068`, not repo-wide
automatic card rewriting.
