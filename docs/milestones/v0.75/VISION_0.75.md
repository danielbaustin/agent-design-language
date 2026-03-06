

# ADL v0.75 — Vision (Deterministic Substrate + ObsMem v1)

**Status:** Draft

v0.75 is the interstitial release between v0.7 and v0.8 that turns ADL from “a powerful deterministic runtime” into a **deterministic execution platform with operational memory**.

This milestone is explicitly sized to be **reviewable and shippable** without ceremony overload, while establishing the substrate required for the v0.8 cohesion release.

## One‑sentence thesis

**v0.75 freezes the deterministic execution contract and adds ObsMem v1 (index + retrieval) so ADL can learn from its own runs without sacrificing replayability.**

## Product story

After v0.75, a user can:

1. Run a workflow and get stable, inspectable artifacts.
2. Replay the run deterministically.
3. Ingest runs into memory.
4. Query: “Show me similar failures and what fixed them,” with citations.

v0.8 will then add Gödel and authoring surfaces on top of this stable foundation.

## Scope (what ships)

This release includes **EPIC‑A + EPIC‑B** as defined in v0.8 planning:

- **EPIC‑A: Deterministic Substrate**
  - Execution contract freeze
  - Activation log + replay runner
  - Failure taxonomy and deterministic classification
  - Trace bundle export v2 (versioned, replay‑sufficient)

- **EPIC‑B: ObsMem v1**
  - Trace bundle ingestion
  - Structured index (runs/activations/evidence)
  - Hybrid retrieval surfaces (structured + optional semantic)
  - Deterministic ranking + tie‑break rules
  - Retrieval explanation + citations

## Non‑goals (explicitly deferred)

- Cluster / distributed execution (defer to v0.85/v0.9).
- Gödel self‑improvement layer (v0.8).
- Authoring surfaces / NL→ADL compiler (v0.8).
- Major provider/tool refactors.
- New sandbox/security model work beyond what is required to preserve existing guarantees.

## Contracts to freeze in v0.75

These are the surfaces we treat as “stable enough to build on”:

1. **Deterministic execution contract**
   - Given identical workflow version + inputs + captured tool boundary events, execution produces identical step outputs and artifact layout.

2. **Activation log schema**
   - Append‑only; sufficient to drive replay.

3. **Trace bundle export v2**
   - Versioned bundle manifest.
   - Stable paths and canonical serialization (excluding run-id/timestamps).

4. **Failure taxonomy**
   - Stable classification identifiers (machine‑readable).
   - Deterministic mapping from observed failures → classification.

5. **ObsMem determinism**
   - Given the same index state + query + retrieval config, results are returned in the same order.
   - Ties are broken deterministically.

## Demo acceptance matrix (v0.75)

v0.75 is complete when we can demonstrate:

### Demo A — Determinism + Replay
- Run a workflow.
- Produce stable artifact tree.
- Replay and confirm the same outputs/artifacts (excluding run-id/timestamps).

### Demo B — Memory ingestion + similarity query
- Ingest 2–5 trace bundles.
- Query for similar failures.
- Return ranked results with citations and an explanation of scoring.

### Demo C — Operational report
- Report basic operational metrics from the index (cost/latency/retry counts) in a deterministic way.

## CLI surfaces (implemented and used in v0.75 docs)

The current CLI and demo docs use these surfaces:

- `adl <workflow.adl.yaml> --run --trace --allow-unsigned`
- `adl demo <name> --run --no-open`
- `adl instrument replay <activation_log.json>`
- `adl instrument replay-bundle <trace_bundle_dir> <run_id>`
- `adl learn export --format <bundle-v1|trace-bundle-v2> ...`

ObsMem indexing/retrieval/report are demonstrated in v0.75 through the
runtime adapter and demo artifacts (`ADL_OBSMEM_DEMO=1`), not as a dedicated
top-level `adl obsmem ...` command surface.

## Determinism and hygiene requirements

- No secrets persisted in artifacts or trace bundles.
- No absolute host paths persisted.
- Canonical serialization everywhere relevant to hashing.
- Stable ordering rules for:
  - plan steps
  - activations
  - evidence items
  - retrieval results

## Risks and mitigations

1. **Hidden nondeterminism at tool boundaries**
   - Mitigation: boundary capture + replay gating; add regression tests.

2. **Embedding instability**
   - Mitigation: embeddings are optional; record embedding model id and retrieval config; deterministic tie‑break.

3. **Index schema churn**
   - Mitigation: version manifests; keep API stable; prefer additive changes.

4. **Ceremony overload**
   - Mitigation: keep milestone limited to EPIC‑A + EPIC‑B and measurable demos; defer everything else.

## Definition of Done (DoD)

v0.75 is DONE when:

- EPIC‑A contract surfaces are frozen and tested.
- Trace bundle v2 export is versioned and replay‑sufficient.
- ObsMem v1 can ingest bundles and answer similarity queries with citations.
- Demo A/B/C run from docs on a fresh checkout.
- CI gates include determinism + hygiene checks (no secrets/host paths).

## Roadmap note

- v0.75 establishes the substrate and memory needed for v0.8.
- v0.8 adds:
  - Gödel evaluation + mutation overlays
  - Authoring surfaces (Plain English → ADL)
- v0.85/v0.9 adds:
  - cluster / distributed execution

---

**Next file to draft:** `.adl/docs/v08planning/VISION_0.80.md`
