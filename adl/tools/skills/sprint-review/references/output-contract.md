# Sprint Review Output Contract

Required default sections:

## Findings
- Findings first, ordered by severity.
- Each finding should include:
  - priority
  - title
  - umbrella or child issue context
  - file or artifact reference when available
  - scenario or trigger
  - impact
  - evidence

## Scope Summary
- reviewed scope type: sprint | mini_sprint | issue_wave | release_tail
- umbrella issue
- child issue list
- PR list reviewed
- changed surfaces reviewed
- skipped surfaces and why

## Lane Coverage
- lane: gap_analysis | code | docs | tests | evidence_and_closeout | synthesis | review_quality | security | architecture | dependency | release_evidence
- status: run | skipped | blocked
- artifact path or reason

## Lifecycle And Closeout Truth
- umbrella issue state
- child issue closure truth summary
- PR state summary
- lifecycle-card truth summary
- closeout artifact summary
- local-only or ignored evidence notes

## Validation Summary
- commands or artifacts reviewed
- what each validation surface actually proved
- missing proof or residual validation gaps

## Residual Risk
- what the review did not inspect or could not prove
- what must be fixed before closure versus what can route as follow-up

## Follow-up Routing
- findings retained in packet only
- recommended follow-up issues
- must-land-before-close vs post-sprint-follow-on distinctions

## Non-Claims
- no merge approval
- no sprint closure approval
- no remediation completion claim
