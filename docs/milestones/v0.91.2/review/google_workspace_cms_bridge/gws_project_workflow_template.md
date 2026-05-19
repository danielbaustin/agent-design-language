# GWS Project Workflow Template

## Intended Workflow

Use this bounded project workflow when Workspace adds value to CodeFriend/ADL
planning and content-card management.

1. Select one bounded project folder/doc/sheet scope.
2. Run live-safety verification.
3. Run bounded live capability execution verification.
4. Inventory or read the current content-card state.
5. Prepare a promotion-packet candidate from the bounded Workspace state.
6. Run the bounded content-card roundtrip path in the project's selected
   posture:
   - dry-run is acceptable for initial adoption and stop-boundary proof
   - execute mode is appropriate only after the project intentionally enables
     live mutation
7. Route actual canonical doc or repo changes through GitHub issue/PR flow.

## Suggested Stage Map

### Stage A: Safety

Goal:
- prove the auth/scope/redaction posture is truthful

Primary surface:
- `gws_live_safety_package_report.json`

### Stage B: Bounded Capability

Goal:
- prove the project can perform one bounded live `gws` slice or truthfully skip

Primary surfaces:
- `gws_live_capability_execution_report.json`
- `gws_live_capability_execution_snapshot.json`

### Stage C: Content-Card Contract

Goal:
- prove the bounded preview/apply roundtrip contract and promotion handoff

Primary surface:
- `gws_live_content_card_roundtrip_report.json`

### Stage D: GitHub-Controlled Promotion

Goal:
- move canonical tracked changes through normal issue/PR review

Primary surface:
- the project's GitHub issue and PR chain

## Expected Stop Boundaries

Stop and review before:

- widening folder/doc/sheet scope
- turning on execute mode for the first time
- treating Workspace as canonical source
- allowing Workspace-originated tracked file edits without GitHub review

## Acceptable Outcomes

- proving dry-run packet
- proving bounded read packet
- skipped live mutation with explicit reason
- execute-mode proof only when auth, scope, and revision checks all align
