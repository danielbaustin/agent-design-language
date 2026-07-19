# v0.91.7 WP-17 Documentation Alignment

Issues: `#4644`, post-merge truth repair `#5542`

Status: closed and merged through PR `#5539`

Verified: 2026-07-18

## Result

The v0.91.7 documentation entry points now describe the live closeout tail:

- WP-01 through WP-17 are closed;
- WP-17 closed through issue `#4644` and merged PR `#5539`;
- WP-18 through WP-20 and WP-23 remain open;
- WP-21 and WP-22 are closed retained planning/review work;
- remediation `#5408` remains open and is not hidden by this alignment pass.

The post-merge repair distinguishes issue and integration truth: issue `#4644`
closed after PR `#5539` merged. It also makes the reviewed v0.91.8 bridge the
required predecessor to v0.92 and labels milestone-baseline creation dates
separately from live verification dates.

The update preserves evidence boundaries. Closed issues establish only the
bounded claims in their retained packets; they do not imply v0.92 activation,
production federation, broad release security, subjective affect, completed
adaptive learning, or general product readiness.

## Audited Scope

| Surface | Audited result |
| --- | --- |
| Repository READMEs | All 208 tracked README Markdown files inventoried case-insensitively, including lowercase template READMEs; 415 unique README/v0.91.7 Markdown entry points scanned together; 830 local links checked with zero unresolved targets after repair. |
| v0.91.7 tree | 1,328 tracked files inventoried, including 231 Markdown files, 808 JSON artifacts, and 55 YAML artifacts. The issue-owned alignment artifacts and merged WP-16 quality-gate packet are included. |
| Feature docs | All ten files under `docs/milestones/v0.91.7/features/` plus the canonical `FEATURE_DOCS_v0.91.7.md` index reviewed. |
| Cargo manifests | All six tracked manifests parsed with `cargo metadata --no-deps --locked`; package versions remain package-local truth rather than being forced to one value. |
| ADRs | 47 accepted-index paths and all nine v0.91.7 ADR-index paths exist; no duplicate accepted ADR identifiers were found. ADR 0051 remains explicitly deferred. |
| Root entry points | `README.md`, `REVIEW.md`, `docs/README.md`, and `docs/adr/README.md` aligned to the v0.91.7 closeout posture. |

Inventory-list SHA-256 digests are retained in
`review/wp17_docs_alignment_4644/audit.json`. The digest covers path lists, not
file contents, and provides a stable statement of the audited population.

## Repairs

- Replaced milestone-start language in the v0.91.7 README, WBS, sprint plan,
  release docs, checklist, handoff, planning-source ledger, feature index, and
  roadmap with current evidence-bounded closeout truth.
- Reconciled Curiosity, Constructability, reasoning/loop/skill, security,
  ACIP/A2A, affect, Godel, economics, guild, observability, and Soak #2 rows
  with their closed issue and retained-proof state.
- Added the historical remote-validation feature to the canonical feature
  index and recorded the current operator direction that AWS execution is not
  authorized. Historical proof remains immutable evidence, not an active lane.
- Verified the accepted/deferred ADR split and retained candidate ADRs 0030,
  0031, 0034, and 0040 without silently promoting them.
- Corrected seven broken relative links in the v0.8 and v0.91 feature README
  entry points found by the complete README scan.
- Reconciled WP-17 from pending integration to closed/merged truth after PR
  `#5539`, removed it from the open WP set, and made the v0.91.8 reviewed
  exact-revision handoff explicit in the repository entry points.
- Replaced ambiguous milestone-baseline `Date` labels with distinct `Created`
  and `Last verified` fields in the canonical live documents.

## Structured Artifact Validation

All 55 YAML files parse. Of 808 JSON-named artifacts, 803 parse as JSON. The
five expected non-parsing evidence files are:

- three zero-byte `csm_stdout.json` log captures under the retained `#4998`
  governed-notice evidence;
- `csm_polis_storage_4913/restore/polis_state_snapshot.corrupt.json`, a named
  corruption fixture;
- `csm_restore_fire_drill_4919/drill/negative/corrupted_manifest/bundle/continuity_capsule_manifest.json`, a negative restore fixture.

Twenty zero-byte files exist in the v0.91.7 tree. They are retained stdout,
stderr, formatting, or daemon-log captures. This pass does not rewrite evidence
to make empty output look populated. The heading-free `#4784`
`reviewer_walkthrough.md` is a two-paragraph retained reviewer instruction,
not an empty or malformed document, and remains unchanged.

## Cargo Manifest Truth

| Manifest | Package version | Disposition |
| --- | --- | --- |
| `adl/Cargo.toml` | `0.91.7` | canonical ADL package |
| `adl-runtime/Cargo.toml` | `0.91.7` | Runtime v2 package |
| `adl-runtime-kernel/Cargo.toml` | `0.92.0` | independent Runtime v3 kernel package; not normalized by WP-17 |
| `csdlc-v2/Cargo.toml` | `0.1.0` | independent C-SDLC v2 tool package |
| `demos/transpiler_demo/Cargo.toml` | `0.8.0` | historical demo package |
| `tools/aws_remote_validation/Cargo.toml` | `0.91.7` | retained historical tooling package; no AWS run performed |

## Validation Boundary

This is a documentation and retained-artifact alignment pass. It uses local
parsers, link checks, Cargo metadata, typed C-SDLC validation, and bounded
review. It does not rerun historical runtime, remote, cloud, or corruption
proofs. No AWS command or service was used.

## Remaining Release Tail

WP-17 is closed and integrated through PR `#5539`. WP-18, WP-19, WP-20, open
remediation `#5408`, and WP-23 retain their own gates. The sprint-review
register is owned by active WP-18 issue `#4645`; its closeout row must be
reconciled by that lane or after its claim is released, without cross-lane
editing. This packet does not close those gates and does not authorize v0.92
activation. v0.92 consumes only the reviewed v0.91.8 exact-revision handoff.
