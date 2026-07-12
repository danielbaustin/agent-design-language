# C-SDLC v2 Gate 2 State Engine Design

Issue: #5232  
Status: implementation design  
Boundary: standalone `csdlc-v2` Rust workspace

## Problem

C-SDLC v1 spread lifecycle authority across large binaries, shell/Python
helpers, mutable Markdown, and overlapping records. Gate 2 establishes the
small independent core: typed state, six automatically generated cards,
semantic editing, atomic persistence, and offline diagnosis.

## Invariants

1. The lifecycle phase is one `strum`-backed enum and advances only through the
   explicit transition table.
2. Every mutation supplies the current generation, issue digest, and live claim.
3. Each semantic field has one owning card. Cross-card writes fail before IO.
4. The six Markdown cards are projections of typed values. Direct Markdown
   drift is detectable and never becomes authority.
5. A commit replaces one complete issue directory. The previous complete
   generation remains recoverable until the replacement is durable.
6. Doctor is read-only, offline, deterministic, and returns stable JSON.
7. Design, diagram, and design-review evidence are readiness inputs, not prose
   conventions.

## Boundary And COTS Choices

The workspace has one small library plus `csdlc-edit` and `csdlc-doctor`.
Clap owns typed CLI parsing; Serde/Serde JSON own records; Schemars derives
public schemas; Thiserror owns the error taxonomy; Strum owns closed
vocabularies; Markdown.rs parses every rendered card to mdast; Blake3 provides
content identity; and fs2 provides the issue lock. No ADL or Runtime crate is a
dependency.

The repository's temporary v1 validation selector contains one narrow
`csdlc-v2/**` integration rule whose only command is
`cargo test --manifest-path csdlc-v2/Cargo.toml`. This lets publication and CI
recognize the independent product without building ADL or Runtime. The rule is
an external integration adapter, not lifecycle authority inside v2.

The state machine is an explicit match table rather than a macro framework.
Markdown serialization is deterministic rendering from typed values followed
by mdast parse/validation; the AST is never modified and re-serialized, and no
text surgery is used.

## Data And State

Each issue lives under `.csdlc/issues/<number>/`:

```text
index.json
audit.jsonl
cards/<kind>.values.json
cards/<kind>.md
```

`index.json` owns identity, phase, generation, digests, claim, design/diagram
references, design-review disposition, and the monotonic transition log. The
cards retain human-readable lifecycle intent at their declared phase.

SPP/VPP budgets are never a post-bootstrap repair step. Initialization selects
a Strum-backed planning profile (`small`, `medium`, `large`, or `migration`)
from the issue contract. The card engine expands that profile into explicit
SPP elapsed/validation seconds and token estimates plus explicit VPP
validation seconds/tokens in the same six-card transaction. VPP lane budgets
must fit inside that envelope. Doctor and cross-card validation fail closed on
zero, missing, or inconsistent estimates.

## Transaction And Failure Behavior

A writer locks the issue, verifies current digests, renders and parses all six
cards, writes a complete staged issue directory, and renames the current
generation to a backup before installing the staged generation. A crash can
leave either the old complete directory, the new complete directory, or a
complete backup with no current directory—never a mixed record. The next
writer recovers the backup deterministically; doctor reports interrupted
transaction evidence without mutating it.

The parent issue directory is fsynced after preserving the backup, after
installing the replacement, and after deleting the backup. The injected
interruption fixture exercises the recovery boundary after backup preservation;
the directory sync barriers provide the corresponding durable rename ordering.

Invalid transitions, stale generations/digests, missing/expired claims,
ownership violations, mdast/schema errors, and direct card drift fail closed
with typed codes.

## Concurrency And Security

The mutation lock is advisory and issue-scoped. Claims include owner,
generation, acquisition/expiry/heartbeat, protected paths, and purpose. Gate 2
validates claim identity and expiry; acquisition, collision policy, heartbeat,
and stale takeover are completed by Gate 3. Machine JSON is stdout-only;
diagnostics go to stderr. No command evaluation or credentials exist in this
gate.

## Validation

Focused tests cover transition legality, six-card construction, ownership and
stale-write rejection, direct drift/corruption, design readiness, and injected
transaction interruption/recovery. Tests validate owned behavior rather than
duplicating every flag. Construction, binary size, doctor p95, and focused
validation are measured separately from ADL and Runtime.

## Rollback

Gate 2 is opt-in and has no operator cutover. Reverting its tracked directory
removes the product without changing v1. At the record level, an interrupted
commit retains the prior complete backup for deterministic recovery.

## Non-goals

- Git branch/worktree creation or claim acquisition (Gate 3).
- PVF execution, scheduling, or shepherding (Gate 4).
- Review mutation, GitHub publication, closeout, legacy import, or cutover.
- Compatibility with every historical card layout.
