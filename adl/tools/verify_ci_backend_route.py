#!/usr/bin/env python3
import argparse
import sys


def parse_bool(value: str) -> bool:
    if value == "true":
        return True
    if value == "false":
        return False
    raise argparse.ArgumentTypeError("expected true or false")


def parse_result(value: str) -> tuple[str, str]:
    name, separator, result = value.partition("=")
    if not separator or not name or not result:
        raise argparse.ArgumentTypeError("expected NAME=RESULT")
    return name, result


parser = argparse.ArgumentParser()
parser.add_argument("--surface", choices=("adl-ci", "adl-coverage"), required=True)
parser.add_argument("--backend", required=True)
parser.add_argument("--event-name", required=True)
parser.add_argument("--same-repo-pr", type=parse_bool, required=True)
parser.add_argument("--spot-opt-in", type=parse_bool, default=False)
parser.add_argument("--work-required", type=parse_bool, required=True)
parser.add_argument("--rust-required", type=parse_bool, default=False)
parser.add_argument("--demo-required", type=parse_bool, default=False)
parser.add_argument("--path-policy-result", required=True)
parser.add_argument("--spot-result", required=True)
parser.add_argument("--hosted-result", action="append", type=parse_result, default=[])
args = parser.parse_args()

errors: list[str] = []
if args.path_policy_result != "success":
    errors.append(f"path policy did not succeed: {args.path_policy_result}")

spot_selected = (
    args.backend == "spot"
    and args.event_name == "pull_request"
    and args.same_repo_pr
    and args.spot_opt_in
)
hosted_results = dict(args.hosted_result)

if spot_selected:
    expected_spot_result = "success" if args.work_required else "skipped"
    if args.spot_result != expected_spot_result:
        errors.append(f"selected Spot lane expected {expected_spot_result}: {args.spot_result}")
    for lane, result in hosted_results.items():
        if result != "skipped":
            errors.append(f"unselected hosted {lane} lane was not skipped: {result}")
elif not spot_selected:
    if args.spot_result != "skipped":
        errors.append(f"unselected Spot lane was not skipped: {args.spot_result}")
    if args.surface == "adl-coverage":
        expected_coverage_result = "success" if args.work_required else "skipped"
        if hosted_results.get("coverage") != expected_coverage_result:
            errors.append(
                f"selected hosted coverage lane expected {expected_coverage_result}: "
                f"{hosted_results.get('coverage', 'missing')}"
            )
    else:
        expected_work_required = args.rust_required or args.demo_required
        if args.work_required != expected_work_required:
            errors.append("adl-ci work-required input disagrees with required lane categories")
        if args.rust_required:
            for lane in ("rust-fmt-clippy", "rust-tests"):
                if hosted_results.get(lane) != "success":
                    errors.append(
                        f"required hosted {lane} lane did not succeed: "
                        f"{hosted_results.get(lane, 'missing')}"
                    )
        if args.demo_required and hosted_results.get("demo-proof") != "success":
            errors.append(
                "required hosted demo-proof lane did not succeed: "
                f"{hosted_results.get('demo-proof', 'missing')}"
            )

if errors:
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)
    raise SystemExit(1)

print(
    "PASS ci_backend_route "
    f"surface={args.surface} backend={'spot' if spot_selected else 'hosted'} "
    f"work_required={str(args.work_required).lower()}"
)
