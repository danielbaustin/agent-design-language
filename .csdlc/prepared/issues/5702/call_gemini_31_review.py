#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
import hashlib
from pathlib import Path


ROOT = Path.cwd()
PLAN = ROOT / ".adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md"
OUT = ROOT / ".adl/local-artifacts/5702-podcast-launch-plan/gemini-review-result.json"
SUMMARY = ROOT / ".csdlc/evidence/5702/gemini-3.1-pro-review-summary.json"
MODEL = os.environ.get("ADL_GEMINI_REVIEW_MODEL", "gemini-3.1-pro-preview")
KEY_FILE = Path(os.environ.get("ADL_GEMINI_API_KEY_FILE", str(Path.home() / "keys/gcp-ace-2023.key")))
TIMEOUT_SECONDS = int(os.environ.get("ADL_GEMINI_REVIEW_TIMEOUT_SECONDS", "600"))
MAX_ATTEMPTS = int(os.environ.get("ADL_GEMINI_REVIEW_ATTEMPTS", "3"))


def read_key() -> str:
    if os.environ.get("GEMINI_API_KEY"):
        return os.environ["GEMINI_API_KEY"].strip()
    return KEY_FILE.read_text(encoding="utf-8").strip()


def main() -> int:
    plan = PLAN.read_text(encoding="utf-8")
    prompt = f"""You are Gemini acting as an external planning reviewer.

Review this internal launch plan for the Agent Logic AI Agent Podcast.
The operator requires audio and RSS to work flawlessly for a next-week launch.

Return:
1. Top 10 concrete suggestions, ordered by launch risk.
2. Missing tests or proof gates.
3. Anything that seems overbuilt or risky for today.
4. A short recommended execution order.

Stay practical. Do not claim you executed the plan.

PLAN:
{plan}
"""
    payload = {
        "contents": [
            {
                "role": "user",
                "parts": [{"text": prompt}],
            }
        ],
        "generationConfig": {
            "maxOutputTokens": 4096,
            "temperature": 0.2,
        },
    }
    url = "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent".format(
        urllib.parse.quote(MODEL, safe="")
    )
    result: dict[str, object] = {}
    attempts: list[dict[str, object]] = []
    api_key = read_key()
    for attempt in range(1, MAX_ATTEMPTS + 1):
        request = urllib.request.Request(
            url,
            data=json.dumps(payload).encode("utf-8"),
            headers={
                "Content-Type": "application/json",
                "x-goog-api-key": api_key,
            },
            method="POST",
        )
        started = time.time()
        try:
            with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
                body = response.read().decode("utf-8")
                parsed = json.loads(body)
                chunks: list[str] = []
                for candidate in parsed.get("candidates", []):
                    content = candidate.get("content", {}) if isinstance(candidate, dict) else {}
                    for part in content.get("parts", []) if isinstance(content, dict) else []:
                        text = part.get("text") if isinstance(part, dict) else None
                        if isinstance(text, str):
                            chunks.append(text)
                review_text = "\n".join(chunks).strip()
                result = {
                    "schema": "adl.podcast_launch.gemini_review.v1",
                    "status": "passed" if review_text else "failed",
                    "model": MODEL,
                    "http_status": response.status,
                    "attempt": attempt,
                    "latency_ms": int((time.time() - started) * 1000),
                    "plan_path": str(PLAN.relative_to(ROOT)),
                    "prompt_chars": len(prompt),
                    "output_chars": len(review_text),
                    "finish_reasons": [
                        candidate.get("finishReason")
                        for candidate in parsed.get("candidates", [])
                        if isinstance(candidate, dict)
                    ],
                    "review_sha256": hashlib.sha256(review_text.encode("utf-8")).hexdigest(),
                    "review_text": review_text,
                }
        except urllib.error.HTTPError as exc:
            body = exc.read().decode("utf-8", errors="replace")
            result = {
                "schema": "adl.podcast_launch.gemini_review.v1",
                "status": "failed",
                "model": MODEL,
                "http_status": exc.code,
                "attempt": attempt,
                "latency_ms": int((time.time() - started) * 1000),
                "plan_path": str(PLAN.relative_to(ROOT)),
                "error": body[:2000],
            }
        except Exception as exc:
            result = {
                "schema": "adl.podcast_launch.gemini_review.v1",
                "status": "failed",
                "model": MODEL,
                "attempt": attempt,
                "latency_ms": int((time.time() - started) * 1000),
                "plan_path": str(PLAN.relative_to(ROOT)),
                "error": str(exc),
            }
        attempts.append({k: v for k, v in result.items() if k != "review_text"})
        if result.get("status") == "passed":
            break
        time.sleep(min(30, 2 * attempt))
    result["attempts"] = attempts
    OUT.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if result.get("status") == "passed":
        SUMMARY.write_text(
            json.dumps(
                {
                    "schema": "adl.podcast_launch.gemini_review_summary.v1",
                    "status": "passed",
                    "model": MODEL,
                    "required_model": "Gemini 3.1 Pro",
                    "required_model_api_id": "gemini-3.1-pro-preview",
                    "attempt": result.get("attempt"),
                    "finish_reasons": result.get("finish_reasons"),
                    "output_chars": result.get("output_chars"),
                    "review_sha256": result.get("review_sha256"),
                    "raw_result_ref": ".adl/local-artifacts/5702-podcast-launch-plan/gemini-review-result.json",
                    "plan_path": str(PLAN.relative_to(ROOT)),
                    "review_text": result.get("review_text", ""),
                },
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )
    print(json.dumps({k: v for k, v in result.items() if k != "review_text"}, indent=2))
    return 0 if result.get("status") == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
