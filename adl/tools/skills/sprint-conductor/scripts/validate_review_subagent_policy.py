#!/usr/bin/env python3
from __future__ import annotations

import argparse


def parse_bool(raw: str) -> bool:
    lowered = raw.strip().lower()
    if lowered in {'1', 'true', 'yes', 'on'}:
        return True
    if lowered in {'0', 'false', 'no', 'off'}:
        return False
    raise ValueError(f'invalid boolean: {raw}')


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--allow-review-subagent-exception', required=True)
    parser.add_argument('--max-review-subagents', type=int, default=1)
    parser.add_argument('--review-subagent-id', action='append', default=[])
    args = parser.parse_args()

    allowed = parse_bool(args.allow_review_subagent_exception)
    count = len(args.review_subagent_id)

    if not allowed and count > 0:
        raise SystemExit('review subagent ids were supplied even though the exception is disabled')
    if allowed and count > args.max_review_subagents:
        raise SystemExit(f'review subagent count {count} exceeds allowed maximum {args.max_review_subagents}')

    print('review_subagent_policy_ok')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
