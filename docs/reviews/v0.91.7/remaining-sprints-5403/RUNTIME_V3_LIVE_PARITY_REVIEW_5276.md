# Runtime v3 Live Parity And Observatory Sprint Review

Issue: #5276
Review issue: #5403
Status: changes required
Remediation: #5413

## Findings

### P1: Nine Runtime v3-only proof lanes are mislabeled live v2/v3 equivalence

The retained matrix classifies ten capabilities as `live_equivalent_fixture`,
but the child tests around `adl-runtime-kernel/tests/parity.rs:1075` primarily
parse proof JSON and assert that it says `proved`. The only actual v2/v3 process
comparison is ignored around lines 803 and 845 and covers only
`reasoning.graphs_and_loops`.

Impact: `cutover_eligible: true` and zero live-parity blockers at
`docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json:6` are not
supported by live equivalence evidence for nine groups.

Disposition: open. Downgrade those groups to Runtime v3 fixture proof and run
real cross-runtime comparisons, or obtain reviewed non-equivalence dispositions.

### P1: Observatory live-consumption proof uses a mocked feed

`adl/tools/validate_v0917_html_observatory.py:269` replaces `fetch` with a
map-backed mock and around line 354 calls the client against
`https://runtime-gateway-host`. This proves parsing and rendering compatibility,
not browser consumption of a running `adl-runtime-kernel` endpoint.

Impact: the packet's `runtime_v3_read_feed_consumed: true` claim at
`docs/architecture/runtime_v3_observatory_consumption_5286.v1.json:30` is an
end-to-end overclaim.

Disposition: open. Route a #5286 live browser/runtime HTTPS proof or downgrade
the packet to mocked client-contract validation.

### P1: The remotely bindable Observatory feed has no read authentication

`GET /v1/observatory` returns the complete feed without signed authorization at
`adl-runtime-kernel/src/control.rs:642` and line 673. CORS controls browser
origins, not network access. The demo README instructs external binding at
`demos/html-observatory/README.md:129`, while the release packet calls
the surface loopback-only at
`docs/architecture/runtime_v3_release_proof_gate_5220.v1.json:89`.

Impact: a remotely exposed runtime can disclose Observatory state to any
network caller, and retained release truth understates the reachable surface.

Disposition: open. Route a Runtime v3 networking/security issue requiring
authenticated read access and consistent local/remote deployment truth.

### P2: Weather is sampled once and presented as live telemetry

The server samples `SysinfoWeatherObserver` once at startup at
`adl-runtime-kernel/src/bin/adl-runtime-kernel.rs:110`. Feed construction clones
that retained report at `adl-runtime-kernel/src/control.rs:349`.

Impact: long-running Observatory clients can display stale CPU, memory, disk,
GPU, and network health as current weather.

Disposition: open. Refresh weather on a bounded cadence or on feed request,
with timestamp/staleness semantics and low overhead.

### P2: Release child results omit the entire second parity wave

`docs/architecture/runtime_v3_release_proof_gate_5220.v1.json:95` lists only
#5247-#5254 even though its zero-blocker and Observatory conclusions consume
#5277-#5286.

Impact: the release packet lacks the child/PR/check inventory needed to audit
the evidence on which its later parity conclusion depends.

Disposition: open. Reconcile the release packet with the second child wave and
the corrected parity classifications.

## Child Coverage

Reviewed #5277-#5286 and merged PRs #5291, #5297, #5299, #5302, #5310, #5313,
#5314, #5288, #5315, and #5317. All child issues are closed and PRs merged.

## Validation And Release Truth

Current Runtime v3 validation produced 151 passed, 0 failed, and 8 ignored. The
ignored set includes the only live v2/v3 process comparison and the real
100-cycle soak. The HTML Observatory validator passed through the mocked path
described above. All five findings above are review-discovered; no
test-discovered defect is counted above.

The conservative release decision remains current and supported: Runtime v3 is
explicit opt-in, Runtime v2 is the default and rollback target, and neither
Runtime v2 deletion nor decommission is authorized. The unsupported claim is
that live black-box parity is complete and cutover-eligible.

Dependency review found no repo-local path dependency or clear manifest/lock
defect. Ordinary transitive version duplication was observed; no vulnerability
audit is claimed.

## Review Result

Changes required. The live-equivalence and Observatory claims must be corrected,
and remote read authentication is required before exposing the feed outside a
trusted local boundary.
