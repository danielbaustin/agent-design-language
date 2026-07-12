# Gate 8: One-Way Import and Shadow Parity

## Decision

Gate 8 is temporary migration code, not a compatibility architecture. The
standalone csdlc-import binary reads six explicitly named legacy Markdown
documents as external data and writes an independently authored v2 issue. It
does not link any v1 crate, schema, template, fixture, test, shell, Python, or
skill code.

The supported surface is intentionally narrow:

- all six card paths must be declared;
- the legacy phase must be Initialized or Ready;
- each card must contain the documented unique level-two headings;
- planning profile and bounded validation argv are explicit typed import
  inputs, so no shell text is guessed;
- every full raw card and every authored section, including preamble and
  unmapped extra sections, is retained verbatim in canonical MigrationEvidence.

markdown.rs mdast identifies semantic heading anchors and source offsets.
Raw source slices between those anchors preserve authored Markdown. Duplicate,
empty, missing, invalid UTF-8, or absent headings produce an Unsupported
migration report before canonical output is created.

## Data flow and loss boundary

~~~mermaid
flowchart LR
    Legacy["Six declared legacy Markdown files"] --> AST["markdown.rs mdast anchors"]
    AST --> Guard{"Unique supported shape?"}
    Guard -->|no| Report["Non-destructive migration diagnostics"]
    Guard -->|yes| Archive["Typed authored-section archive"]
    Guard --> Values["Independent v2 values"]
    Typed["Planning profile + typed validation argv"] --> Values
    Values --> Cards["Six generated v2 cards"]
    Archive --> Index["Canonical v2 index"]
    Cards --> Index
    Index --> View["Generated compatibility view: do not edit"]
~~~

There is no silent loss path. Mapped sections populate the operative v2
planning fields. All supported and extra authored sections remain in the
typed archive and generated view. The view is disposable and re-derived from
the archive at the single fixed .csdlc/compat/<issue>.md path; callers cannot
select a canonical card/index path. Atomic replacement means editing or
interrupting the view cannot change canonical state.

Construction is digest-bound and resumable. Initialization and migration
evidence are idempotent for the identical request/source digest, Ready
advancement is skipped when already applied, and the generated view uses an
atomic temporary-file rename. A failed final view write can therefore be
retried without duplicating generations or leaving an unrecoverable import.

~~~mermaid
flowchart TD
    Section["Authored legacy section"] --> Mapped{"Declared v2 mapping?"}
    Mapped -->|yes| Field["Typed v2 field"]
    Mapped -->|no| Archive["Typed migration archive"]
    Field --> Archive
    Archive --> Proof["Source digest + section count"]
    Ambiguous["Duplicate/missing/empty anchor"] --> Stop["Unsupported; write nothing"]
~~~

## Shadow parity

csdlc-shadow accepts one normalized legacy doctor observation and derives the
same schema from the v2 index/cards. It compares issue, lifecycle phase, six
card statuses, review result, integration/publication/merge/closeout states,
and claim liveness. Markdown bytes, directories, internal schemas, prompts,
and audit layout are deliberately absent.

~~~mermaid
flowchart LR
    V1["Black-box v1 doctor observation"] --> N1["NormalizedOutcome v1"]
    V2["v2 doctor/index/cards"] --> N2["NormalizedOutcome v1"]
    N1 --> Compare["Field comparison"]
    N2 --> Compare
    Compare --> Equal["Equivalent outcome"]
    Compare --> Diff["Exact normalized differences"]
~~~

## Sunset

Every successful or unsupported report records
default_cutover_unix_seconds + 2,592,000 seconds. That is exactly 30 days.
There is no indefinite or mutable default. A reviewed later issue may extend
the date, but the importer itself has no extension authority. Gate 10 owns
deletion after the bounded window.

## Proof posture

Eight focused Gate 8 tests prove one-way input immutability, canonical disjoint
root/path enforcement, full raw-card and authored-section retention,
six-card/index construction, generated-view derivation, duplicate-heading
no-write failure, normalized parity independent of Markdown bytes, exact
difference reporting, digest-bound retry after a compatibility write failure,
source-change refusal, unrepresentable-layout no-write diagnostics,
fixed projection non-authority, schemas, and the 30-day sunset. This is the
smallest proving surface; existing lifecycle tests continue to cover card and
transaction invariants.
