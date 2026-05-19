# Moderne Modernization Reversibility And Review Policy

## Purpose

Define the acceptance and rollback boundary for the `WP-10` modernization demo.

The goal is to keep modernization reviewable, reversible, and explicitly
governed rather than allowing "AI modernization" to collapse into silent mass
rewrite.

## Review Gate

No modernization change should be accepted by default.

Required review sequence:

1. bounded objective accepted
2. recipe family justified
3. dry-run output or equivalent command posture reviewed
4. resulting diff summarized
5. residuals classified
6. explicit human acceptance before any merge or broader rollout

## Reversibility Rule

Every accepted modernization slice must remain reversible at the issue/PR
boundary.

Minimum reversibility expectations:

- one bounded recipe family per proof slice
- reviewer-comprehensible diff size
- explicit record of changed files and change category
- follow-on routing for residuals instead of burying them in the same change

## Approval Posture

Dry-run may be evaluated under bounded baseline authority.

Mutation acceptance requires explicit review because:

- deterministic recipes can still be overbroad
- repository coupling can still create partial or ambiguous results
- "successful execution" is not the same as "safe modernization outcome"

## Allowed Outcome Labels

The modernization lane should classify outcomes as:

- `accepted`
- `partial`
- `blocked`
- `non_proving`

These labels matter more than raw execution success because the real ADL claim
is about governed truthfulness, not about forcing every recipe run into success
language.

## Residual Handling

Residuals must stay visible:

- skipped files
- ambiguous transformations
- hidden build coupling
- manual fixups required
- follow-on recipe candidates

Residuals should create a smaller follow-on or a clearer stop, not a hidden
scope expansion.

## Non-Claims

- This policy does not authorize autonomous repo-wide modernization.
- This policy does not claim a recipe run should merge automatically.
- This policy does not replace normal PR review.
