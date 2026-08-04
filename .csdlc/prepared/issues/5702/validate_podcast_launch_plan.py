#!/usr/bin/env python3
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
PLAN = ROOT / ".adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md"
GEMINI = ROOT / ".csdlc/evidence/5702/gemini-3.1-pro-review-summary.json"


required_patterns = [
    "Launch posture: audio and RSS are required launch gates",
    "Gemini 3.1 Pro Review Incorporation",
    "Deepgram is an investigation lane, not a preselected vendor.",
    "Critical-path rule: harden the existing known route first.",
    "public podcast feed, expected route `/podcast/feed.xml`",
    "feed content parsed and compared back to `episode.yaml`",
    "audio player actually plays, seeks, and reports duration/progress",
    "DeepSeek invited AI guest",
    "No launch until all lanes are green",
    "Implementation is not done until the follow-on issues prove:",
    "gemini-3.1-pro-preview",
    ".csdlc/evidence/5702/gemini-3.1-pro-review-summary.json",
    "Apple Podcasts approval timing",
    "URL-order dependency",
    "CDATA",
    "TTS chunking/retry",
    "ID3v2.3",
    "iOS Safari",
    "publish-ready human guest episodes require signed/approved release state",
]

forbidden_patterns = [
    "The Agent Logic site has a static `site/` tree",
    "`site/index.html`",
    "`assets/css/styles.css`",
    "`site/podcast/index.html`",
    "`site/podcast/feed.xml`",
]

source_paths = [
    "docs/milestones/v0.91.8/features/AI_AGENT_PODCAST_STUDIO_v0.91.8.md",
    "docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_LAUNCH_READINESS_5605.md",
    "docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_TOPIC_SLATE_5605.md",
    "docs/milestones/v0.91.3/review/podcast_studio_v2/PODCAST_STUDIO_V2_PACKET_v0.91.3.md",
    "demos/v0.91.3/adl_podcast_studio_v2_episode_card.html",
]


def fail(message: str) -> None:
    raise SystemExit(f"podcast launch plan validation failed: {message}")


if not PLAN.is_file():
    fail(f"missing plan: {PLAN}")

text = PLAN.read_text(encoding="utf-8")

missing = [pattern for pattern in required_patterns if pattern not in text]
if missing:
    fail(f"missing required plan content: {missing}")

forbidden = [pattern for pattern in forbidden_patterns if pattern in text]
if forbidden:
    fail(f"stale local website path claims remain: {forbidden}")

missing_sources = [path for path in source_paths if not (ROOT / path).is_file()]
if missing_sources:
    fail(f"missing source evidence paths: {missing_sources}")

if not GEMINI.is_file():
    fail(f"missing retained Gemini result: {GEMINI}")

gemini = json.loads(GEMINI.read_text(encoding="utf-8"))
if gemini.get("status") != "passed":
    fail(f"Gemini review did not pass: {gemini.get('status')!r}")
if gemini.get("model") != "gemini-3.1-pro-preview":
    fail(f"unexpected Gemini model: {gemini.get('model')!r}")
if gemini.get("required_model") != "Gemini 3.1 Pro":
    fail(f"unexpected required Gemini model: {gemini.get('required_model')!r}")
if gemini.get("required_model_api_id") != "gemini-3.1-pro-preview":
    fail(f"unexpected required Gemini API id: {gemini.get('required_model_api_id')!r}")
if gemini.get("finish_reasons") != ["STOP"]:
    fail(f"Gemini review did not finish cleanly: {gemini.get('finish_reasons')!r}")
if int(gemini.get("output_chars") or 0) < 1000:
    fail("Gemini review output was unexpectedly short")
if not gemini.get("review_sha256"):
    fail("Gemini review summary is missing review digest")

print("podcast launch plan validation passed")
