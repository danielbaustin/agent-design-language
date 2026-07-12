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
)
hosted_results = dict(args.hosted_result)

if spot_selected and args.work_required:
    if args.spot_result != "success":
        errors.append(f"selected Spot lane did not succeed: {args.spot_result}")
elif not spot_selected:
    if args.surface == "adl-coverage":
        if hosted_results.get("coverage") != "success":
            errors.append(
                "selected hosted coverage lane did not succeed: "
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
