README focused validation for issue #5653

README content SHA-256: 97d1adf2cc22988934a5b58af84bd851a0cf01f709da1e6e320b77e0d5f6d038
The evidence and lifecycle metadata commits are packaging-only; this digest
pins the README content independently of those metadata revisions.

1. git diff --check
   PASS

2. README assertions
   PASS: homepage URL https://agent-logic.ai is present
   PASS: v0.91.8 status is present
   PASS: v0.91.7 GitHub release link is present
   PASS: stale v0.91.7 closeout badge is absent
   PASS: CI and coverage badges target branch=main

3. Homepage reachability
   PASS: https://agent-logic.ai returned HTTP 200
