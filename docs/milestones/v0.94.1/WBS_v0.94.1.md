# v0.94.1 Candidate Work Breakdown Structure

## Status

Candidate allocation only. `v0.94.1` has no final issue wave yet.

## WBS Summary

`v0.94.1` should give payments, settlement, `x402`, Lightning adapters, and
economic-agency follow-on work a dedicated bounded milestone.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | Payment adapter boundary | Define the adapter interface and authority boundary. | adapter contract | `v0.94` secure execution |
| B | `x402` and Lightning adapters | Define the protocol-specific adapter surfaces. | adapter specs | A |
| C | Settlement rules | Define bounded settlement behavior and non-goals. | settlement contract | A, B |
| D | Economic accounting and trace | Define accounting, ledger, and economic trace artifacts. | accounting schema and trace surface | C |
| E | Economic agency | Define performance-position and economic-agency follow-on surfaces. | agency contract | D |
| F | Demo matrix and proofs | Build bounded payment/economic demo candidates. | demo matrix and proof candidates | A through E |
| G | Review and release tail | Align docs and review package. | release-ready planning package | All prior work |

## Acceptance Mapping

- payments work must be explicitly separate from `v0.93` and `v0.94`
- financial claims must remain bounded and non-production
- adapter, settlement, accounting, and demo surfaces must all have a tracked home
