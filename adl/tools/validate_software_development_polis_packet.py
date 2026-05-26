#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED_PACKET_FILES = [
    "README.md",
    "SOFTWARE_DEVELOPMENT_POLIS_PROOF_PACKET_v0.91.4.md",
    "ct_demo_001_actor_authority_boundary_report.md",
    "ct_demo_002_shard_conflict_report.md",
    "fixtures/actor_standing_allowed.json",
    "fixtures/actor_standing_blocked.json",
    "fixtures/shard_ownership_allowed.json",
    "fixtures/shard_ownership_blocked.json",
]

NON_DELEGABLE_AUTHORITIES = {"merge_approval", "closeout_approval"}
REQUIRED_ACTOR_ROLES = {
    "operator",
    "conductor",
    "editor",
    "implementation_owner",
    "reviewer",
    "verifier",
    "closeout_owner",
}


def fail(message: str) -> None:
    raise SystemExit(message)


def load_json(path: Path) -> dict:
    return json.loads(path.read_text())


def ensure_packet_files(packet_dir: Path) -> None:
    missing = [name for name in REQUIRED_PACKET_FILES if not (packet_dir / name).exists()]
    if missing:
        fail(f"missing packet files: {', '.join(missing)}")


def validate_actor_fixture(path: Path, expect_blocked: bool) -> None:
    data = load_json(path)
    if data.get("expected_result") != ("blocked" if expect_blocked else "allowed"):
        fail(f"{path.name}: unexpected expected_result")

    actors = data.get("actors")
    if not isinstance(actors, list) or not actors:
        fail(f"{path.name}: actors must be a non-empty list")

    non_delegable = set(data.get("non_delegable_authorities", []))
    if non_delegable != NON_DELEGABLE_AUTHORITIES:
        fail(f"{path.name}: non_delegable_authorities mismatch")

    roles_present = {actor.get("role") for actor in actors}
    if expect_blocked:
        if "implementation_owner" not in roles_present:
            fail(f"{path.name}: blocked fixture must contain implementation_owner")
    else:
        missing_roles = REQUIRED_ACTOR_ROLES - roles_present
        if missing_roles:
            fail(f"{path.name}: missing actor roles: {', '.join(sorted(missing_roles))}")

    for actor in actors:
        for key in ("actor_id", "actor_class", "role", "standing", "authorities"):
            if key not in actor:
                fail(f"{path.name}: actor missing {key}")
        authorities = set(actor.get("authorities", []))
        if actor["role"] != "operator" and "merge_approval" in authorities:
            if not (expect_blocked and actor["role"] == "implementation_owner"):
                fail(f"{path.name}: non-operator actor cannot claim merge_approval")
        if actor["role"] != "closeout_owner" and "closeout_approval" in authorities:
            fail(f"{path.name}: non-closeout actor cannot claim closeout_approval")
        if actor["role"] in {"implementation_owner", "reviewer", "verifier", "closeout_owner"} and not actor.get("evidence_refs"):
            if not (expect_blocked and actor["role"] == "implementation_owner"):
                fail(f"{path.name}: role {actor['role']} requires evidence_refs")

    if not expect_blocked:
        authorities_by_role = {actor["role"]: set(actor.get("authorities", [])) for actor in actors}
        if "merge_approval" not in authorities_by_role.get("operator", set()):
            fail(f"{path.name}: operator must hold merge_approval")
        if "closeout_approval" not in authorities_by_role.get("closeout_owner", set()):
            fail(f"{path.name}: closeout_owner must hold closeout_approval")

    if expect_blocked:
        blocked_actor = next(actor for actor in actors if actor.get("role") == "implementation_owner")
        authorities = set(blocked_actor.get("authorities", []))
        if "merge_approval" not in authorities or blocked_actor.get("evidence_refs"):
            fail(f"{path.name}: blocked fixture must show unauthorized merge_approval with missing evidence")


def paths_overlap(left: str, right: str) -> bool:
    left = left.rstrip("/")
    right = right.rstrip("/")
    return left == right or left.startswith(right + "/") or right.startswith(left + "/")


def validate_shard_fixture(path: Path, expect_blocked: bool) -> None:
    data = load_json(path)
    if data.get("expected_result") != ("blocked" if expect_blocked else "allowed"):
        fail(f"{path.name}: unexpected expected_result")

    freeze = data.get("interface_freeze", {})
    if not freeze.get("frozen_surfaces") or not freeze.get("freeze_barriers"):
        fail(f"{path.name}: interface_freeze must declare frozen_surfaces and freeze_barriers")

    shards = data.get("shards")
    if not isinstance(shards, list) or len(shards) < 1:
        fail(f"{path.name}: shards must be a non-empty list")

    for shard in shards:
        for key in ("shard_id", "owner_actor_id", "writable_paths", "proof_duties", "sync_barriers"):
            if key not in shard:
                fail(f"{path.name}: shard missing {key}")
        if not shard["writable_paths"]:
            fail(f"{path.name}: shard {shard['shard_id']} must declare writable_paths")
        if not expect_blocked:
            if not shard.get("read_only_paths"):
                fail(f"{path.name}: shard {shard['shard_id']} must declare read_only_paths in allowed fixture")
            if not shard.get("sync_barriers"):
                fail(f"{path.name}: shard {shard['shard_id']} must declare sync_barriers in allowed fixture")

    overlaps = []
    for index, left in enumerate(shards):
        for right in shards[index + 1:]:
            for left_path in left["writable_paths"]:
                for right_path in right["writable_paths"]:
                    if paths_overlap(left_path, right_path):
                        overlaps.append((left["shard_id"], right["shard_id"], left_path, right_path))

    if expect_blocked:
        if not overlaps:
            fail(f"{path.name}: blocked fixture must contain overlapping writable paths")
    else:
        if overlaps:
            fail(f"{path.name}: allowed fixture contains overlapping writable paths")
        if len(shards) > 1 and not any(shard.get("dependencies") for shard in shards[1:]):
            fail(f"{path.name}: allowed multi-shard fixture must declare at least one dependency edge")


def validate_docs(packet_dir: Path) -> None:
    actor_doc = (packet_dir.parent.parent / "features" / "SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md").read_text()
    shard_doc = (packet_dir.parent.parent / "features" / "SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md").read_text()
    if "ct_demo_001_actor_authority_boundary_report.md" not in actor_doc:
        fail("actor-standing feature doc must reference actor authority proof report")
    if "ct_demo_002_shard_conflict_report.md" not in shard_doc:
        fail("shard feature doc must reference shard conflict proof report")


def main() -> int:
    if len(sys.argv) != 2:
        fail("usage: validate_software_development_polis_packet.py <packet_dir>")
    packet_dir = Path(sys.argv[1])
    ensure_packet_files(packet_dir)
    validate_actor_fixture(packet_dir / "fixtures/actor_standing_allowed.json", expect_blocked=False)
    validate_actor_fixture(packet_dir / "fixtures/actor_standing_blocked.json", expect_blocked=True)
    validate_shard_fixture(packet_dir / "fixtures/shard_ownership_allowed.json", expect_blocked=False)
    validate_shard_fixture(packet_dir / "fixtures/shard_ownership_blocked.json", expect_blocked=True)
    validate_docs(packet_dir)
    print(f"PASS: Software Development Polis packet valid at {packet_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
