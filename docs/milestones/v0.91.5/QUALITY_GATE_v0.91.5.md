# v0.91.5 Quality Gate

## Status

Planned quality gate.

## Required Validation

The milestone must run focused validation for:

- planning-template structure for canonical planning docs;
- YAML parsing for the issue wave;
- GitHub issue label/title routing for moved bridge issues;
- public prompt packet export, validation, and redaction gates;
- `.adl` archive/deletion review-before-delete disposition;
- provider/model matrix smoke and role-probe evidence;
- multi-agent workcell proof or explicit blocker;
- single-agent versus multi-agent overhead comparison;
- demo readiness and Unity Observatory routing;
- v0.92 activation map completeness;
- `#3377` first-birthday readiness packet.

## Blockers

The milestone is blocked if:

- multi-agent execution is claimed without role, shard, provider, review, and
  closeout evidence;
- OpenRouter/provider matrix work hides skipped or blocked lanes;
- public prompt packets can leak local/private state;
- `.adl` cleanup deletes historical material without review;
- v0.92 activation surfaces remain undocumented;
- v0.92 docs depend on direct v0.91.4 closeout rather than v0.91.5 closeout and
  `#3377`.

## Release Gate

Release can proceed only when every blocker is fixed, explicitly deferred with
owner/rationale, or converted into a v0.92 WP-01 prerequisite.

