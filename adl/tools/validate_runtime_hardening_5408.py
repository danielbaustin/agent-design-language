#!/usr/bin/env python3
"""Validate the retained, bounded WP-07 hardening contract."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def require(path: str, *needles: str) -> None:
    text = (ROOT / path).read_text()
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(f"{path}: missing required contract: {missing}")


require(
    "adl/src/long_lived_agent.rs",
    'GOVERNED_STOP_POLICY_KEY',
    'GOVERNED_STOP_POLICY_PUBLIC_KEY',
    "authorization signature verification failed",
    "operator is not present in the locked agent spec policy",
    "governed_stop_authorization_payload",
    '"authorization_verified": true',
    '"operator_identity_verified": true',
    '"os_identity_verified": true',
)
require(
    "adl/src/csm_api_gateway_bridge.rs",
    'status: "bounded_smoke"',
    '"default_route_is_not_substituted": true',
    '"malformed_request": "api_gateway_malformed_request"',
    '"upstream_failure": "deferred_to_injected_upstream_fixture"',
)
require(
    "docs/review-fixes/runtime/WP07_COHERENCE_GATE_DISPOSITION_5408.md",
    "#4906",
    "blocked_with_evidence",
    "explicit hold",
    "not a waiver",
)
print("WP-07 runtime hardening contract passed")
