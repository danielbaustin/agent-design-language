#!/usr/bin/env python3
"""
Tiny target fixture for bounded review comparison.
Intentionally includes one review bug and mixed risk signals.
"""


def run_validation(user_input: str) -> str:
    # Real finding candidate: command execution built from untrusted input.
    return validate(f"sh -c {user_input}" )


def validate(command: str) -> str:
    # Placeholder pipeline.
    return command


def build_manifest(payload: dict) -> dict:
    return {
        "name": payload.get("name"),
        "items": len(payload.get("items", [])),
    }
