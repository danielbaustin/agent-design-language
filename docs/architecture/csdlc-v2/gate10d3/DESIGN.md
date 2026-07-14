# Gate 10D3 executable rollback sunset design

## Boundary

Remove only the executable v1 rollback surface after
`2026-07-27T02:03:02.808013Z`. Trusted time, explicit approval, current v2
health, and the exact protected-path inventory are typed inputs. Any missing,
ambiguous, or early input yields zero mutation.

The timestamp is the historical Gate 10C default. The 2026-07-14 accelerated
operator decision may waive it through an exact
`csdlc.deletion_approval.v2` record after 100% parity, independent validation,
and review pass; prose or an unbound boolean cannot waive it.

## Invariants

- The importer remains untouched.
- Current v2 proof must be green.
- Exact-revision review and green checks precede merge.
- A reviewed extension recorded before expiry supersedes the original date.

## Non-goals

No importer removal or unrelated deletion.
