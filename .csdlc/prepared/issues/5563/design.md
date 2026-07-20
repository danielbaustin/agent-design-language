# #5563 Design

Permit an explicit typed design reapproval while an issue is still `initialized` only when the existing design review is approved and the current authored design or diagram digest no longer matches the SPP/VPP projections.

The operation retains exact generation/digest CAS, active-claim validation, reviewer identity, atomic six-card rendering, and audit evidence. It does not permit lifecycle advancement or bypass readiness checks; it only refreshes the authored-artifact digests so normal doctor and `ready` gates can run.
