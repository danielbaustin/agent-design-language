#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import re


ENV_NAME_RE = re.compile(r'^[A-Z_][A-Z0-9_]*$')


def parse_bool(raw: str) -> bool:
    lowered = raw.strip().lower()
    if lowered in {'1', 'true', 'yes', 'on'}:
        return True
    if lowered in {'0', 'false', 'no', 'off'}:
        return False
    raise ValueError(f'invalid boolean: {raw}')


def present_auth_env_names(names: list[str]) -> list[str]:
    present = []
    for name in names:
        if not ENV_NAME_RE.match(name):
            raise SystemExit(f'invalid auth environment variable name: {name}')
        if os.environ.get(name, '').strip():
            present.append(name)
    return present


def require_responses_auth_context(names: list[str]) -> None:
    required = names or ['OPENAI_API_KEY']
    if present_auth_env_names(required):
        return
    joined = ', '.join(required)
    raise SystemExit(
        'missing Codex Responses API authentication context: none of the '
        f'required environment variables are set: {joined}. Configure an '
        'inherited parent-session auth context or a local environment before '
        'starting the bounded review subagent; credential values were not '
        'inspected or printed.'
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--allow-review-subagent-exception', required=True)
    parser.add_argument('--max-review-subagents', type=int, default=1)
    parser.add_argument('--review-subagent-id', action='append', default=[])
    parser.add_argument('--require-responses-auth', action='store_true')
    parser.add_argument('--auth-env', action='append', default=[])
    parser.add_argument('--subagent-model-override')
    args = parser.parse_args()

    allowed = parse_bool(args.allow_review_subagent_exception)
    count = len(args.review_subagent_id)
    model_override = (args.subagent_model_override or '').strip()

    if not allowed and count > 0:
        raise SystemExit('review subagent ids were supplied even though the exception is disabled')
    if allowed and count > args.max_review_subagents:
        raise SystemExit(f'review subagent count {count} exceeds allowed maximum {args.max_review_subagents}')
    if count > 0 and model_override:
        raise SystemExit(
            'review subagent model override is forbidden; forked reviewer '
            'subagents must inherit the parent session model and authentication context'
        )
    if count > 0 and args.require_responses_auth:
        require_responses_auth_context(args.auth_env)

    print('review_subagent_policy_ok')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
