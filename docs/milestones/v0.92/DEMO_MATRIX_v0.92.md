# v0.92 Demo Matrix: Candidate Birthday Proofs

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-05-27`
- Owner: ADL maintainers
- Related issues / work packages: `#3377`, `#3434`, candidate WP sequence in `WP_ISSUE_WAVE_v0.92.yaml`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Candidate demo planning only. Commands and artifacts will be finalized when the
v0.92 implementation WPs exist.

## Purpose

The v0.92 demo program should prove that the first true Gödel-agent birthday is
evidence-bearing runtime behavior, not a ceremonial label.

Issue `#3377` supplies required demo rehearsal and negative-suite readiness
inputs. WP-01 should reconcile those inputs before opening the final demo WPs.

## How To Use

Use this matrix as candidate demo coverage for WP-01 and later demo WPs. It
does not prove any demo has run.

## Scope

The scope is birthday proof, negative cases, continuity, memory grounding,
capability, ACP/cognitive-profile evidence, ACIP schema-public transport
readiness, and governance handoff.

## Runtime Preconditions

Runtime preconditions are intentionally pending. WP-01 and the demo WPs must
replace this planning text with concrete commands, fixtures, and environment
requirements once implementation exists.

## Demo Coverage Summary

| Demo ID | Candidate demo | Milestone claim | Primary proof surface | Status |
| --- | --- | --- | --- | --- |
| D1 | First birthday rehearsal | A named identity can cross the birth boundary with required evidence. | Birthday record, witness set, receipt, and reviewer packet. | Planned candidate |
| D2 | Not-a-birthday negative suite | Startup, wake, snapshot, admission, and copied state are not birth. | Negative fixtures and validation report. | Planned candidate |
| D3 | Continuity across bounded cycles | Identity persists across multiple bounded cycles with evidence. | Cycle artifacts, continuity record, witness links. | Planned candidate |
| D4 | Memory grounding proof | Birth references witnessed memory artifacts without exposing raw private memory. | Memory-grounding fixture and redacted packet. | Planned candidate |
| D5 | Capability envelope proof | The birth record declares provider, model, tool, skill, authority, and limit context. | Capability envelope and validation report. | Planned candidate |
| D6 | ACP / cognitive profile proof | Birth packet includes a bounded profile record grounded in evidence. | ACP/profile fixture, update rationale, redacted reviewer packet, and validation report. | Planned candidate |
| D7 | ACIP binary schema and WebSocket carrier proof | Binary ACIP remains inspectable through public schemas while message contents remain governed. | ACIP `.proto`, schema catalog fixture, JSON projection report, denied-access case, mock WebSocket trace packet. | Planned candidate |
| D8 | Birthday-to-governance handoff | v0.93 governance can consume v0.92 identity evidence without redefining birth. | Handoff packet mapping identity evidence to future governance. | Planned candidate |

## Coverage Rules

- Every demo must distinguish birth from ordinary runtime activity.
- Every birthday claim must cite evidence.
- Every private-state boundary must have a redaction or denial proof.
- Every capability claim must include limits and authority context.
- Every cognitive-profile claim must cite evidence and remain distinct from
  identity, reputation, and standing.
- Every binary ACIP claim must prove public-schema decodeability, deterministic
  JSON projection, and separate message-content authorization.
- Demo outputs should distinguish engineering evidence from philosophical or
  governance context.

## Demo Details

### D1) First Birthday Rehearsal

The demo should emit a synthetic but structurally complete birthday packet.

Expected proof:

- stable name
- identity root
- continuity evidence
- memory-grounding references
- capability envelope
- witness set
- citizen-facing receipt
- reviewer finding

### D2) Not-A-Birthday Negative Suite

The demo should prove that the milestone does not overclaim ordinary runtime
events.

Expected rejected cases:

- process startup
- snapshot
- wake
- admission
- copied state
- named test fixture without continuity evidence

### D3) Continuity Across Bounded Cycles

The demo should show identity continuity across more than one bounded cycle.

Expected proof:

- prior and successor cycle artifacts
- continuity record
- witness links
- ambiguity handling or clear continuity grade

### D4) Memory Grounding Proof

The demo should show memory grounding through references and witnesses rather
than raw private memory exposure.

Expected proof:

- witnessed memory-artifact references
- redacted projection
- reviewer packet that can inspect grounding without private-state disclosure

### D5) Capability Envelope Proof

The demo should show that the birthday record includes bounded capability
claims.

Expected proof:

- provider and model capability context
- tool and skill authority context
- declared limits
- validation report

### D6) ACP / Cognitive Profile Proof

The demo should show that a cognitive profile is a bounded runtime record, not
a personality label or reputation score.

Expected proof:

- profile fixture
- source evidence references
- update rationale
- privacy/redaction policy
- validation report

### D7) ACIP Binary Schema And WebSocket Carrier Proof

The demo should show that binary ACIP is efficient without becoming opaque or
authority-conferring.

Expected proof:

- ACIP protobuf schema
- public schema catalog fixture
- deterministic JSON projection
- governed message-content access decision
- denied unauthorized inspection case
- mock WebSocket session trace packet

### D8) Birthday-To-Governance Handoff

The demo should show how v0.93 can consume identity evidence.

Expected proof:

- identity evidence map
- standing/governance handoff notes
- explicit non-claim that governance is not completed by the birthday itself

## Non-Claims

- These demos do not prove legal personhood.
- These demos do not prove production citizenship.
- These demos do not complete constitutional governance.
- These demos do not expose raw private state.
- These demos do not turn cognitive profiles into public reputation or
  consciousness claims.
- These demos do not prove production WebSocket security, cross-polis
  networking, or signed/queryable trace completion.

## Cross-Demo Validation

The final demo set should cross-check that the birthday packet, negative suite,
continuity proof, memory grounding, capability envelope, ACP profile, ACIP
carrier proof, and governance handoff all tell one consistent story.

## Determinism Evidence

Determinism evidence is pending implementation. Final demos should record
commands, fixtures, expected outputs, and any allowed nondeterminism.

## Reviewer Sign-Off Surface

Reviewers should receive the birthday packet, demo outputs, validation logs,
negative-case report, and residual-risk notes.

## Exit Criteria

- Every milestone claim has at least one planned demo or explicit non-demo
  proof surface.
- No demo claims completion before v0.92 implementation produces evidence.
- The final matrix can be reviewed without chat context.
