# Gate 10D1 non-mutating deletion eligibility

`csdlc-eligibility` is a read-only authority boundary between reviewed cutover
evidence and any future deletion issue. It reads versioned Phase B/Phase C
evidence, the tracked generation selector, an exact-path disposition manifest,
and typed approval. It writes only JSON to stdout and has no code path for
creating, deleting, renaming, editing, staging, committing, publishing, or
closing repository paths.

The binary reconstructs the canonical 95-file, 49,979-line inventory directly
from pinned revision `020bba17deb9f172e91a2ec5c0599cf42e4defe9`, verifies the
two reviewed sorted-list hashes and pinned line total, and rejects missing,
untracked, non-regular, or symlinked current paths. The manifest supplies one
default disposition plus exact-path overrides, so every canonical path has
exactly one disposition without caller-supplied line counts or overlapping
directory claims. Every retained disposition names an owner and justification.
Ninety percent is the review target; 80-89 percent requires explicit
qualification in the approval; below 80 percent is never eligible.

Approval is not a boolean. It binds the approver and approval time to exact
BLAKE3 digests of Phase B evidence, Phase C evidence, the selector, the
manifest, and the evaluated Git code revision. The Phase C record must itself
bind the exact Phase B digest. Its rollback and importer expiry timestamps are
mandatory eligibility clocks even when no manifest override mentions them.
Missing or mismatched approval, non-green evidence, a non-v2 selector, either
active mandatory window, an entry-specific protected window, or a deficient
removal percentage produces `eligible=false`. Digests and revisions require
strict lowercase hexadecimal encoding. The decision always records
`deletion_executed=false` because execution belongs to separate issue #5306.

The tracked request intentionally contains no approval and retains the full
baseline. Its expected decision is ineligible with zero v1 mutation. Synthetic
tests cover positive eligibility only to prove decision semantics; they grant
no operational deletion authority.

`csdlc-eligibility schema` emits the versioned request, manifest, entry,
approval, and decision JSON Schemas. `csdlc-eligibility evaluate --repo <repo>
--request <file>` emits a decision to stdout and accepts no output path.
