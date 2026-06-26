# Unity Observatory Local Runtime Consumption Proof

## Status

Bounded proof for ADL issue `#4548`.

## Purpose

Prove that the Unity 6.5 Observatory shell can consume an explicitly local
runtime-derived Observatory contract instead of only the older checked-in seed,
without widening into live runtime/network ingestion claims.

## Bounded Contract Path

Runtime-derived packet source:

- `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`

Generated bundle artifact used by the proof:

- staged `unity_observatory_contract.json` copied into
  `Assets/Resources/observatory_contract.json` inside a disposable Unity
  project copy

Normal checked-in project seed remains:

- `demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json`

## Proof Commands

Contract baseline guardrail:

```bash
bash adl/tools/test_v0916_unity_observatory_contract.sh
```

Local runtime-derived consumption proof:

```bash
bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh
```

## What The Proof Verifies

The local-runtime proof stages a disposable Unity project copy, replaces the
staged `observatory_contract.json` with a freshly generated runtime-derived
bundle, runs the Unity batch validator, and asserts the rendered shell surfaces:

- title `Prototype CSM 01`
- packet ref `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
- artifact root `runtime_v2`
- report ref `runtime_v2/observatory/operator_report.md`
- evidence-level note `artifact_backed_fixture`

## Observed Non-Claims

- No live runtime socket, service, or network ingestion is claimed.
- No Unity player build is claimed.
- No direct governed-packet parser inside Unity is claimed.
- The checked-in project seed is not replaced by this proof; only the staged
  disposable copy is changed.
