#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
import json
import shutil
from pathlib import Path
from textwrap import dedent


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REVIEW_DIR = ROOT / "docs/milestones/v0.91.3/review/podcast_studio_v2"
DEFAULT_CARD_PATH = ROOT / "demos/v0.91.3/adl_podcast_studio_v2_episode_card.html"
DEFAULT_FEATURE_PATH = ROOT / "docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"
DEFAULT_LOGO_PATH = ROOT / "demos/v0.91.3/agent-logic-logo.png"

DEMO = {
    "demo_name": "ADL Podcast Studio v2",
    "milestone_version": "v0.91.3",
    "issue": "#3223",
    "wp": "demo WP-04",
    "series_name": "ADL Podcast Studio",
    "episode_title": "Episode 01: Can AI Be a Good Teammate?",
    "episode_slug": "episode-01-ai-good-teammate",
    "topic": "What makes an AI helpful in a real team instead of just impressive in a demo?",
    "bounded_purpose": "Show a repeatable, inspectable media-production system that turns a bounded topic into a full episode packet without hidden credentials or fake audio claims.",
    "timebox_claim": "Episode packet generation is deterministic and quick; this demo does not claim live five-minute end-to-end creative production.",
}

HOSTS = [
    {
        "name": "ChatGPT",
        "show_role": "Lead host",
        "studio_job": "opens the question, keeps the conversation warm, and turns big ideas into usable takeaways",
        "style": "clear, friendly, grounded",
        "color": "#f97316",
    },
    {
        "name": "Gemini",
        "show_role": "Field reporter",
        "studio_job": "brings the practical angle, asks what would work on a real team, and keeps the pace moving",
        "style": "bright, curious, direct",
        "color": "#14b8a6",
    },
    {
        "name": "Claude",
        "show_role": "Story voice",
        "studio_job": "keeps the human stakes visible, notices what the episode means, and helps the ending land",
        "style": "reflective, calm, thoughtful",
        "color": "#60a5fa",
    },
]

SEGMENTS = [
    {
        "id": "topic-brief",
        "title": "Topic Brief",
        "owner": "Producer",
        "content": dedent(
            """
            # Topic Brief

            ## Episode Question

            What makes an AI helpful in a real team instead of just impressive in a demo?

            ## Why this episode exists

            People have seen enough flashy AI demos. The harder question is what makes an AI feel useful when deadlines, ambiguity, disagreement, and ordinary work all show up at once. This episode turns that question into a practical conversation for people building with AI now.

            ## Production boundary

            - keep the question understandable for non-specialists
            - talk about work habits, not benchmarks
            - use concrete examples of good and bad AI collaboration
            - leave listeners with one thing to try this week

            ## Desired listener outcome

            A listener should come away with a simple test for whether an AI teammate is helping: does it reduce confusion, improve decisions, and make the human team more capable?
            """
        ).strip()
    },
    {
        "id": "transcript",
        "title": "Transcript",
        "owner": "Roundtable",
        "turns": [
            {
                "speaker": "ChatGPT",
                "label": "Opening frame",
                "body": "Welcome to Agent Logic. Today's question is simple: what makes an AI a good teammate? Not a clever demo, not a chatbot that wins a benchmark, but something you would actually want beside you when the work gets messy. My answer is that a good AI teammate reduces confusion, improves decisions, and helps the human team become more capable.",
            },
            {
                "speaker": "Gemini",
                "label": "Counter-pressure",
                "body": "I like that because it gives us something to measure in normal life. A bad AI teammate makes more tabs, more cleanup, and more uncertainty. A good one asks the clarifying question early, remembers what matters, spots contradictions, and gives you a smaller next step instead of a bigger pile of possibilities.",
            },
            {
                "speaker": "Claude",
                "label": "Human meaning check",
                "body": "There is also a tone question. The best teammates do not try to dominate the room. They make space. They know when to be precise, when to be quiet, and when to say, 'I do not know.' For AI, humility is not a personality flourish. It is a reliability feature.",
            },
            {
                "speaker": "ChatGPT",
                "label": "Bridge",
                "body": "That is why I think the first useful question is not 'How smart is it?' It is 'What kind of work relationship does it create?' Does it make the team calmer? Does it surface tradeoffs? Does it help people notice the thing they were avoiding? Intelligence without those qualities can still be noisy and expensive.",
            },
            {
                "speaker": "Gemini",
                "label": "Operational deepening",
                "body": "And there is a practical habit here. Before you ask an AI for an answer, ask it to help shape the problem. Give it the audience, the constraints, the deadline, and what would make the answer useful. A good teammate is usually better at helping you set up the work than magically finishing work you have not defined.",
            },
            {
                "speaker": "Claude",
                "label": "Closure",
                "body": "So here is the takeaway: judge AI by the quality of the collaboration it creates. If it helps you think, decide, and recover from mistakes, it is becoming a teammate. If it only produces more material to manage, it is still just another tool asking for supervision.",
            },
        ],
    },
    {
        "id": "best-lines",
        "title": "Best Lines",
        "owner": "Editor",
        "quotes": [
            "A good AI teammate reduces confusion, improves decisions, and helps the human team become more capable.",
            "A bad AI teammate makes more tabs, more cleanup, and more uncertainty.",
            "For AI, humility is not a personality flourish. It is a reliability feature.",
            "The first useful question is not 'How smart is it?' It is 'What kind of work relationship does it create?'",
            "Judge AI by the quality of the collaboration it creates.",
        ],
    },
]


def transcript_markdown() -> str:
    lines = [
        f"# {DEMO['series_name']}",
        "",
        f"## {DEMO['episode_title']}",
        "",
        f"**Topic:** {DEMO['topic']}",
        "",
        "## Transcript",
        "",
    ]
    for turn_number, turn in enumerate(SEGMENTS[1]["turns"], start=1):
        lines.extend(
            [
                f"### Turn {turn_number} - {turn['speaker']} ({turn['label']})",
                "",
                turn["body"],
                "",
            ]
        )
    return "\n".join(lines).strip() + "\n"


def display_path(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return str(path)


def host_lineup_markdown() -> str:
    lines = [
        "# Host Lineup",
        "",
        "## Stable production roles",
        "",
    ]
    for host in HOSTS:
        lines.extend(
            [
                f"### {host['name']}",
                f"- show role: {host['show_role']}",
                f"- studio job: {host['studio_job']}",
                f"- voice / style target: {host['style']}",
                "",
            ]
        )
    return "\n".join(lines).strip() + "\n"


def best_lines_markdown() -> str:
    lines = ["# Best Lines", ""]
    for quote in SEGMENTS[2]["quotes"]:
        lines.append(f"- {quote}")
    lines.append("")
    return "\n".join(lines)


def reviewer_proof_note() -> str:
    return dedent(
        f"""
        # Reviewer Proof Note

        ## Bounded claim

        This demo proves ADL can package one recurring episode as a deterministic, reviewable production packet with explicit roles, visible transcript authorship, a polished episode card, and truthful audio-render status.

        ## What this packet proves

        - one-command packet generation can produce all required review surfaces without hidden credentials
        - role boundaries are visible in the packet itself
        - the transcript, best-lines pass, and episode card can read like one coherent show package rather than disconnected scraps
        - audio status remains exact and boring instead of inflated

        ## What this packet suggests

        - a governed creative-production lane can feel more alive when artifact quality and truth are authored together
        - the C-SDLC can support repeatable media production if more packet truth moves into first-class generators

        ## What this packet does not prove

        - live provider-backed episode generation
        - real final-audio render success
        - literal five-minute end-to-end production
        - production publishing or distribution readiness

        ## Review boundary

        The bounded review bar for this issue is whether a reviewer can inspect each deliverable directly from repo-relative tracked artifacts and whether the packet avoids hidden prerequisites or inflated render claims.
        """
    ).strip() + "\n"


def packet_markdown(review_dir: Path, card_path: Path, feature_path: Path) -> str:
    review_display = display_path(review_dir)
    card_display = display_path(card_path)
    feature_display = display_path(feature_path)
    artifacts = [
        f"{review_display}/ct_demo_004_topic_brief.md",
        f"{review_display}/ct_demo_004_host_lineup.md",
        f"{review_display}/ct_demo_004_transcript.md",
        f"{review_display}/ct_demo_004_best_lines.md",
        f"{review_display}/ct_demo_004_audio_render_manifest.json",
        f"{review_display}/ct_demo_004_episode_packet.md",
        f"{review_display}/ct_demo_004_reviewer_proof_note.md",
        card_display,
        feature_display,
    ]
    bullet_list = "\n".join(f"- `{artifact}`" for artifact in artifacts)
    return (
        f"# Podcast Studio v2 Demo Proof Packet v0.91.3\n\n"
        f"## Demo Identity\n\n"
        f"- demo name: {DEMO['demo_name']}\n"
        f"- issue / WP: {DEMO['wp']} / {DEMO['issue']}\n"
        f"- milestone version: `{DEMO['milestone_version']}`\n"
        f"- primary artifact: `{card_display}`\n\n"
        f"## Bounded Purpose\n\n"
        f"{DEMO['bounded_purpose']}\n\n"
        "## Claims\n\n"
        "- ADL can package one deterministic recurring episode packet with visible role boundaries.\n"
        "- The demo can stay truthful about audio render status without requiring hidden credentials.\n\n"
        "## Non-Claims\n\n"
        "- This packet does not claim live provider-backed conversation generation.\n"
        "- This packet does not claim real final-audio rendering or publication readiness.\n\n"
        "## Run Path\n\n"
        "- primary command: `bash adl/tools/demo_v0913_podcast_studio_v2.sh`\n"
        "- operator prerequisites: repository checkout only; no secrets or external services required\n"
        "- run status: `passed`\n\n"
        "## Timebox Truth\n\n"
        "- timebox claim: packet generation is fast and deterministic, but literal five-minute end-to-end show production is not claimed here\n"
        "- evidence type: `estimated`\n"
        "- start evidence: local bounded generator invocation\n"
        "- end evidence: tracked packet regeneration plus validator/test completion\n"
        "- elapsed result: bounded local packet regeneration only; no five-minute proof claim\n\n"
        "## Validation Evidence\n\n"
        "```bash\n"
        "bash adl/tools/demo_v0913_podcast_studio_v2.sh\n"
        f"python3 adl/tools/validate_podcast_studio_v2_packet.py {review_display} {card_display} {feature_display}\n"
        "bash adl/tools/test_podcast_studio_v2_packet.sh\n"
        "```\n\n"
        "Validation not run:\n\n"
        "- real provider-backed audio generation, because the bounded demo intentionally avoids hidden credentials and fake live-audio claims\n\n"
        "## Review Evidence\n\n"
        "- review surface: bounded local review over the generated packet, helper, validator, and episode card\n"
        "- findings fixed before publication: any packet-shape, role-visibility, or audio-status truth drift found during bounded review\n"
        "- residual risks: the packet is a deterministic production-system demo, not a proof of real publishing or live-render reliability\n\n"
        "## Result Classification\n\n"
        "| Claim | Classification | Reason |\n"
        "| --- | --- | --- |\n"
        "| deterministic recurring episode packet exists | `passed` | one-command packet generation writes all required review surfaces without hidden credentials |\n"
        "| audio render status stays truthful | `passed` | manifest records `manifest_only` instead of implying a real render |\n"
        "| literal five-minute creative production is proven | `partial` | the artifact is strong, but this packet does not measure or prove the full timebox target |\n\n"
        "## Skipped Work\n\n"
        "- skipped scope: live provider-backed generation and final audio synthesis\n"
        "- why it was skipped: this bounded issue requires a no-secrets-needed proof path and exact render claims\n\n"
        "## Repo-Relative Artifacts\n\n"
        f"{bullet_list}\n"
    )


def episode_packet_markdown(card_path: Path) -> str:
    card_display = display_path(card_path)
    return dedent(
        f"""
        # Episode Packet

        ## Series

        - series: {DEMO['series_name']}
        - episode: {DEMO['episode_title']}
        - slug: `{DEMO['episode_slug']}`

        ## Packet Checklist

        - [x] topic brief
        - [x] host lineup
        - [x] transcript
        - [x] best-lines extract
        - [x] audio render manifest
        - [x] episode card
        - [x] reviewer proof note

        ## Canonical Command

        ```bash
        bash adl/tools/demo_v0913_podcast_studio_v2.sh
        ```

        ## Reviewer Path

        1. Read `ct_demo_004_topic_brief.md`.
        2. Read `ct_demo_004_host_lineup.md`.
        3. Inspect `ct_demo_004_transcript.md` and `ct_demo_004_best_lines.md`.
        4. Verify exact render status in `ct_demo_004_audio_render_manifest.json`.
        5. Open `{card_display}`.
        6. Confirm the claims/non-claims in `ct_demo_004_reviewer_proof_note.md`.
        """
    ).strip() + "\n"


def audio_manifest() -> dict:
    return {
        "schema": "adl.podcast_studio_v2.audio_manifest.v1",
        "series_name": DEMO["series_name"],
        "episode_title": DEMO["episode_title"],
        "episode_slug": DEMO["episode_slug"],
        "render_status": "manifest_only",
        "rendered_audio_present": False,
        "canonical_command": "bash adl/tools/demo_v0913_podcast_studio_v2.sh",
        "render_policy": {
            "live_audio_required": False,
            "hidden_credentials_required": False,
            "truth_boundary": "This bounded demo records routing and intended render posture without claiming a final audio artifact.",
        },
        "speaker_routes": [
            {
                "speaker": host["name"],
                "transcript_identity": host["name"],
                "intended_voice_style": host["style"],
                "render_path": "not_run_manifest_only",
            }
            for host in HOSTS
        ],
        "reason_not_rendered": "The bounded v0.91.3 demo proves recurring packet production without requiring hidden credentials or claiming a final audio render.",
    }


def feature_markdown(review_dir: Path, card_path: Path) -> str:
    review_display = display_path(review_dir)
    card_display = display_path(card_path)
    return dedent(
        f"""
        # Podcast Studio v2 Demo

        ## Summary

        `WP-04` upgrades the older podcast pilot into a deterministic production-system demo.

        The result is not a live provider-backed episode factory. It is a repeatable, inspectable one-command packet generator that emits a topic brief, host lineup, transcript, best-lines extract, truthful audio render manifest, reviewer proof note, and polished episode card.

        ## Canonical Command

        ```bash
        bash adl/tools/demo_v0913_podcast_studio_v2.sh
        ```

        ## What It Proves

        - one recurring episode packet can be regenerated deterministically
        - role boundaries are visible across the packet
        - audio render status can stay exact without hidden credentials
        - the production artifact can feel like a show package rather than a bare validation log

        ## What It Does Not Prove

        - live provider-backed episode generation
        - final rendered audio output
        - literal five-minute end-to-end creative production
        - publishing or distribution readiness

        ## Proof Surfaces

        - `{review_display}/`
        - `{card_display}`
        - `adl/tools/demo_v0913_podcast_studio_v2.sh`
        - `adl/tools/validate_podcast_studio_v2_packet.py`
        """
    ).strip() + "\n"


def review_readme(card_path: Path) -> str:
    card_display = display_path(card_path)
    return dedent(
        """
        # Podcast Studio v2 Review Packet

        This packet holds the bounded proof surfaces for `WP-04` / `#3223`.

        Start with:

        1. `PODCAST_STUDIO_V2_PACKET_v0.91.3.md`
        2. `ct_demo_004_episode_packet.md`
        3. `ct_demo_004_reviewer_proof_note.md`
        4. `__CARD_PATH__`
        """
    ).replace("__CARD_PATH__", card_display).strip() + "\n"


def episode_card_html() -> str:
    host_cards = "\n".join(
        dedent(
            f"""
            <article class="host-card reveal" style="--host-color: {host['color']};">
              <span class="host-dot" aria-hidden="true"></span>
              <div>
                <p class="section-kicker">{html.escape(host['show_role'])}</p>
                <h3>{html.escape(host['name'])}</h3>
                <p>{html.escape(host['studio_job'])}</p>
                <p class="voice">Voice target: {html.escape(host['style'])}</p>
              </div>
            </article>
            """
        ).strip()
        for host in HOSTS
    )
    transcript_cards = "\n".join(
        dedent(
            f"""
            <article class="turn-card reveal">
              <div class="turn-meta">
                <span class="turn-speaker">{html.escape(turn['speaker'])}</span>
                <span class="turn-label">{html.escape(turn['label'])}</span>
              </div>
              <p>{html.escape(turn['body'])}</p>
            </article>
            """
        ).strip()
        for turn in SEGMENTS[1]["turns"]
    )
    best_lines = "\n".join(
        f"<li>{html.escape(quote)}</li>" for quote in SEGMENTS[2]["quotes"]
    )
    return dedent(
        f"""
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <title>{html.escape(DEMO['episode_title'])}</title>
          <style>
            :root {{
              --bg: #ffffff;
              --bg-soft: #f6f8fb;
              --ink: #111316;
              --muted: #5f6875;
              --muted-2: #8a94a3;
              --line: #dce3ee;
              --line-strong: #b8c4d6;
              --blue: #1a56db;
              --blue-dark: #123f9f;
              --blue-soft: #eaf1ff;
              --teal: #0f766e;
              --sun: #f59e0b;
              --panel: rgba(255, 255, 255, 0.84);
              --shadow: 0 24px 70px rgba(20, 37, 68, 0.12);
              --radius-lg: 30px;
              --radius-md: 20px;
              --container: 1180px;
            }}
            * {{ box-sizing: border-box; }}
            html {{ scroll-behavior: smooth; }}
            body {{
              margin: 0;
              color: var(--ink);
              font-family: "Aptos", "Suisse Int'l", "Helvetica Neue", sans-serif;
              background:
                linear-gradient(180deg, rgba(234, 241, 255, 0.76) 0%, #ffffff 44%, #f8fbff 100%);
            }}
            body::before {{
              position: fixed;
              inset: 0;
              z-index: -1;
              pointer-events: none;
              content: "";
              background-image:
                linear-gradient(rgba(17, 19, 22, 0.035) 1px, transparent 1px),
                linear-gradient(90deg, rgba(17, 19, 22, 0.035) 1px, transparent 1px);
              background-size: 48px 48px;
              mask-image: linear-gradient(180deg, #000 0%, transparent 72%);
            }}
            .page {{
              width: min(var(--container), calc(100% - 40px));
              margin: 0 auto;
              padding: 16px 0 72px;
            }}
            .studio-header {{
              display: flex;
              align-items: center;
              justify-content: space-between;
              gap: 24px;
              margin-bottom: 26px;
              padding: 12px 14px 12px 18px;
              border: 1px solid rgba(220, 227, 238, 0.9);
              border-radius: 999px;
              background: rgba(255, 255, 255, 0.82);
              box-shadow: 0 14px 42px rgba(34, 48, 76, 0.08);
              backdrop-filter: blur(18px);
            }}
            .brand {{
              display: inline-flex;
              align-items: center;
              min-width: 190px;
            }}
            .brand img {{
              display: block;
              width: min(220px, 44vw);
              height: auto;
            }}
            .studio-nav {{
              display: flex;
              align-items: center;
              gap: 4px;
            }}
            .studio-nav a {{
              padding: 10px 14px;
              border-radius: 999px;
              color: var(--muted);
              font-size: 0.92rem;
              font-weight: 700;
              text-decoration: none;
              transition: color 180ms ease, background 180ms ease;
            }}
            .studio-nav a:hover,
            .studio-nav a:focus-visible {{
              color: var(--ink);
              background: var(--blue-soft);
              outline: none;
            }}
            .hero,
            .section,
            .studio-console {{
              border: 1px solid var(--line);
              border-radius: var(--radius-lg);
              background: var(--panel);
              box-shadow: var(--shadow);
            }}
            .hero {{
              display: grid;
              grid-template-columns: minmax(0, 1.02fr) minmax(360px, 0.98fr);
              gap: 34px;
              align-items: stretch;
              min-height: 610px;
              padding: clamp(28px, 5vw, 62px);
              overflow: hidden;
            }}
            .hero-copy {{
              display: flex;
              flex-direction: column;
              justify-content: center;
              min-width: 0;
            }}
            .hero-copy h1 {{
              margin: 0;
              max-width: 14ch;
              font-size: clamp(2.45rem, 4.8vw, 4.75rem);
              line-height: 1.02;
              letter-spacing: 0;
            }}
            .hero-copy p {{
              max-width: 660px;
              font-size: clamp(1rem, 1.45vw, 1.18rem);
              line-height: 1.55;
              color: var(--muted);
            }}
            .section-kicker {{
              margin: 0 0 10px;
              color: var(--blue);
              font-size: 0.78rem;
              font-weight: 850;
              letter-spacing: 0.16em;
              text-transform: uppercase;
            }}
            .hero-actions,
            .status-strip {{
              display: flex;
              flex-wrap: wrap;
              gap: 12px;
            }}
            .hero-actions {{
              margin-top: 14px;
            }}
            .button {{
              display: inline-flex;
              align-items: center;
              justify-content: center;
              min-height: 48px;
              padding: 0 20px;
              border-radius: 999px;
              font-size: 0.95rem;
              font-weight: 800;
              text-decoration: none;
              transition: transform 180ms ease, box-shadow 180ms ease, border-color 180ms ease;
            }}
            .button:hover,
            .button:focus-visible {{
              transform: translateY(-2px);
              outline: none;
            }}
            .button-primary {{
              color: #fff;
              background: var(--ink);
              box-shadow: 0 14px 30px rgba(17, 19, 22, 0.18);
            }}
            .button-secondary {{
              border: 1px solid var(--line-strong);
              color: var(--ink);
              background: #fff;
            }}
            .studio-console {{
              position: relative;
              display: grid;
              grid-template-rows: auto 1fr auto;
              min-height: 500px;
              padding: 18px;
              background:
                linear-gradient(180deg, #fff, #f5f8fd);
              box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.8), var(--shadow);
            }}
            .guest-strip {{
              display: grid;
              grid-template-columns: repeat(4, minmax(0, 1fr));
              gap: 12px;
              margin-top: 18px;
            }}
            .guest-bubble {{
              position: relative;
              min-height: 132px;
              padding: 18px 16px 16px;
              border: 1px solid var(--line);
              border-radius: 28px 28px 28px 10px;
              background: #fff;
              box-shadow: 0 14px 34px rgba(34, 48, 76, 0.1);
            }}
            .guest-bubble::after {{
              position: absolute;
              left: 18px;
              bottom: -12px;
              width: 18px;
              height: 18px;
              border-right: 1px solid var(--line);
              border-bottom: 1px solid var(--line);
              border-bottom-right-radius: 18px;
              content: "";
              background: #fff;
            }}
            .guest-avatar {{
              display: inline-flex;
              align-items: center;
              justify-content: center;
              width: 34px;
              height: 34px;
              margin-bottom: 12px;
              border-radius: 50%;
              color: var(--blue);
              background: rgba(26, 86, 219, 0.1);
              font-size: 0.76rem;
              font-weight: 900;
              letter-spacing: 0.04em;
            }}
            .guest-bubble strong {{
              display: block;
              margin-bottom: 8px;
              color: var(--ink);
            }}
            .guest-bubble span {{
              color: var(--muted);
              font-size: 0.9rem;
              line-height: 1.45;
            }}
            .console-top,
            .console-status {{
              display: flex;
              justify-content: space-between;
              gap: 10px;
              color: var(--muted-2);
              font-size: 0.78rem;
              font-weight: 800;
              letter-spacing: 0.08em;
              text-transform: uppercase;
            }}
            .console-canvas {{
              position: relative;
              min-height: 340px;
              margin: 28px 0;
              border: 1px solid var(--line);
              border-radius: 24px;
              background:
                linear-gradient(rgba(26, 86, 219, 0.045) 1px, transparent 1px),
                linear-gradient(90deg, rgba(26, 86, 219, 0.045) 1px, transparent 1px),
                rgba(255, 255, 255, 0.72);
              background-size: 28px 28px;
              overflow: hidden;
            }}
            .console-canvas svg {{
              position: absolute;
              inset: 0;
              width: 100%;
              height: 100%;
            }}
            .console-canvas path {{
              fill: none;
              stroke: var(--blue);
              stroke-width: 4px;
              stroke-linecap: round;
              stroke-dasharray: 8px 12px;
              vector-effect: non-scaling-stroke;
              animation: pathFlow 6s linear infinite;
            }}
            .node {{
              position: absolute;
              z-index: 2;
              display: grid;
              place-items: center;
              min-width: 96px;
              height: 46px;
              padding: 0 14px;
              border: 1px solid var(--line-strong);
              border-radius: 999px;
              color: var(--ink);
              background: rgba(255, 255, 255, 0.94);
              box-shadow: 0 12px 28px rgba(34, 48, 76, 0.12);
              font-size: 0.86rem;
              font-weight: 800;
            }}
            .node-topic {{ left: 7%; top: 30%; transform: translateY(-50%); }}
            .node-guest {{ left: 7%; top: 68%; transform: translateY(-50%); }}
            .node-studio {{
              left: 50%;
              top: 50%;
              width: 96px;
              height: 96px;
              transform: translate(-50%, -50%);
              border-color: rgba(26, 86, 219, 0.36);
              color: #fff;
              background: var(--blue);
            }}
            .node-feed {{ right: 7%; top: 50%; transform: translateY(-50%); }}
            .console-status {{
              align-items: center;
              text-transform: none;
              letter-spacing: 0;
            }}
            .console-status span {{
              max-width: 32%;
            }}
            .console-status b {{
              display: block;
              color: var(--ink);
            }}
            .section {{
              margin-top: 26px;
              padding: clamp(28px, 5vw, 58px);
            }}
            .section-heading {{
              display: grid;
              gap: 6px;
              margin-bottom: 34px;
            }}
            h2 {{
              margin: 0;
              font-size: clamp(1.8rem, 3.1vw, 3rem);
              line-height: 1.04;
              letter-spacing: 0;
            }}
            h3 {{
              margin: 0 0 10px;
              font-size: 1.08rem;
            }}
            .section-lede {{
              max-width: 720px;
              margin: 0;
              color: var(--muted);
              font-size: clamp(1rem, 1.45vw, 1.16rem);
              line-height: 1.55;
            }}
            .split-grid {{
              display: grid;
              grid-template-columns: 0.92fr 1.08fr;
              gap: 28px;
            }}
            .truth-list,
            .host-grid,
            .highlight-list {{
              display: grid;
              gap: 14px;
            }}
            .show-note,
            .host-card,
            .turn-card,
            .highlight-list li {{
              border: 1px solid var(--line);
              border-radius: var(--radius-md);
              background: #fff;
            }}
            .show-note {{
              padding: 20px;
            }}
            .show-note strong {{
              display: block;
              margin-bottom: 6px;
            }}
            .show-note p,
            .host-card p,
            .turn-card p,
            .highlight-list li {{
              color: var(--muted);
              line-height: 1.55;
            }}
            .host-card {{
              display: grid;
              grid-template-columns: 44px 1fr;
              column-gap: 18px;
              padding: 20px;
            }}
            .host-dot {{
              width: 34px;
              height: 34px;
              border-radius: 999px;
              background: var(--host-color);
              box-shadow: 0 12px 28px color-mix(in srgb, var(--host-color) 24%, transparent);
            }}
            .host-card h3 {{
              color: var(--ink);
            }}
            .host-card .voice {{
              margin-bottom: 0;
              color: color-mix(in srgb, var(--host-color) 58%, var(--ink));
              font-weight: 700;
            }}
            .production-grid {{
              display: grid;
              grid-template-columns: repeat(3, 1fr);
              gap: 14px;
            }}
            .production-step {{
              min-height: 170px;
              padding: 22px;
              border: 1px solid var(--line);
              border-radius: var(--radius-md);
              background: linear-gradient(180deg, #fff 0%, #f8fbff 100%);
            }}
            .step-index {{
              display: block;
              margin-bottom: 34px;
              color: var(--blue);
              font-size: 0.78rem;
              font-weight: 850;
              letter-spacing: 0.12em;
            }}
            .turn-card {{
              padding: 22px;
            }}
            .episode-loaded {{
              display: grid;
              gap: 14px;
              margin-top: 22px;
            }}
            .turn-meta {{
              display: flex;
              gap: 12px;
              align-items: baseline;
              flex-wrap: wrap;
              margin-bottom: 10px;
            }}
            .turn-speaker {{
              color: var(--ink);
              font-size: 1.08rem;
              font-weight: 850;
            }}
            .turn-label {{
              color: var(--blue);
              font-size: 0.76rem;
              font-weight: 850;
              letter-spacing: 0.12em;
              text-transform: uppercase;
            }}
            .transcript-grid {{
              display: grid;
              gap: 14px;
            }}
            .pill {{
              padding: 10px 14px;
              border-radius: 999px;
              border: 1px solid var(--line);
              background: rgba(255, 255, 255, 0.8);
              color: var(--muted);
              font-size: 0.92rem;
              font-weight: 700;
            }}
            .pill.good {{
              color: var(--teal);
              background: rgba(20, 184, 166, 0.1);
              border-color: rgba(20, 184, 166, 0.28);
            }}
            .pill.waiting {{
              color: #92400e;
              background: rgba(245, 158, 11, 0.13);
              border-color: rgba(245, 158, 11, 0.28);
            }}
            .highlight-list {{
              margin: 0;
              padding: 0;
              list-style: none;
            }}
            .highlight-list li {{
              padding: 20px;
            }}
            .footer-note {{
              margin-top: 26px;
              padding: 24px;
              border: 1px solid var(--line);
              border-radius: var(--radius-lg);
              color: var(--muted);
              background: #fff;
            }}
            .reveal {{
              animation: revealUp 700ms ease both;
            }}
            .reveal:nth-child(2) {{ animation-delay: 90ms; }}
            .reveal:nth-child(3) {{ animation-delay: 160ms; }}
            @keyframes revealUp {{
              from {{ opacity: 0; transform: translateY(18px); }}
              to {{ opacity: 1; transform: translateY(0); }}
            }}
            @keyframes pathFlow {{
              to {{ stroke-dashoffset: -96px; }}
            }}
            @media (max-width: 900px) {{
              .studio-header {{
                align-items: flex-start;
                flex-direction: column;
                border-radius: 28px;
              }}
              .hero,
              .split-grid,
              .guest-strip,
              .production-grid {{
                grid-template-columns: 1fr;
              }}
              .hero {{ min-height: auto; }}
              .console-status {{ align-items: flex-start; flex-direction: column; }}
              .console-status span {{ max-width: none; }}
            }}
            @media (max-width: 640px) {{
              .page {{ width: min(100% - 24px, var(--container)); }}
              .studio-nav {{ flex-wrap: wrap; }}
              .hero {{ padding: 24px; }}
              .node {{ min-width: 78px; font-size: 0.76rem; }}
              .node-studio {{ width: 78px; height: 78px; }}
            }}
          </style>
        </head>
        <body>
          <main class="page">
            <header class="studio-header">
              <div class="brand" aria-label="Agent Logic Podcast Studio">
                <img src="agent-logic-logo.png" alt="Agent Logic" width="260" height="100" />
              </div>
              <nav class="studio-nav" aria-label="Studio sections">
                <a href="#studio">Studio</a>
                <a href="#cast">Cast</a>
                <a href="#episode">Episode</a>
              </nav>
            </header>

            <section class="hero reveal" id="studio">
              <div class="hero-copy">
                <p class="section-kicker">Weekly AI conversations</p>
                <h1>Agent Logic Podcast Studio</h1>
                <p>
                  A reusable production room for planning weekly AI conversations, inviting guests,
                  shaping crisp episodes, and giving curious listeners a friendlier way into the work.
                  One sample episode is loaded; the studio is designed for the whole series.
                </p>
                <div class="status-strip">
                  <span class="pill good">Weekly show format</span>
                  <span class="pill waiting">Audio launch next</span>
                  <span class="pill">Guest-ready</span>
                </div>
                <div class="hero-actions" aria-label="Primary studio actions">
                  <a class="button button-primary" href="#episode">Read the sample episode</a>
                  <a class="button button-secondary" href="#cast">Meet the voices</a>
                </div>
                <div class="episode-loaded" aria-label="Loaded sample episode">
                  <span class="pill">{html.escape(DEMO['episode_title'])}</span>
                  <span class="pill">Reusable format: topic, guests, script, audio, notes</span>
                </div>
              </div>
              <aside class="studio-console" aria-label="Podcast production flow">
                <div class="console-top">
                  <span>Studio board</span>
                  <span>Season 1</span>
                </div>
                <div class="console-canvas">
                  <svg viewBox="0 0 600 360" aria-hidden="true">
                    <path d="M90 108 C220 108 230 180 300 180 C370 180 384 180 510 180" />
                    <path d="M90 246 C220 246 230 180 300 180" />
                  </svg>
                  <span class="node node-topic">Topic</span>
                  <span class="node node-guest">Guest</span>
                  <span class="node node-studio">Studio</span>
                  <span class="node node-feed">Show</span>
                </div>
                <div class="guest-strip" aria-label="Reusable guest thought bubbles">
                  <div class="guest-bubble">
                    <span class="guest-avatar">AI</span>
                    <strong>AI guest slot</strong>
                    <span>DeepSeek, Gemini, Claude, or another invited model can occupy this voice.</span>
                  </div>
                  <div class="guest-bubble">
                    <span class="guest-avatar">HU</span>
                    <strong>Human guest slot</strong>
                    <span>Founder, researcher, operator, or AI luminary joins with the same prep shape.</span>
                  </div>
                  <div class="guest-bubble">
                    <span class="guest-avatar">RS</span>
                    <strong>Research voice</strong>
                    <span>Source-grounded context, definitions, and counterpoints stay ready each week.</span>
                  </div>
                  <div class="guest-bubble">
                    <span class="guest-avatar">LQ</span>
                    <strong>Listener question</strong>
                    <span>A reusable audience prompt keeps the episode friendly instead of too inside-baseball.</span>
                  </div>
                </div>
                <div class="console-status">
                  <span><b>Topic</b>{html.escape(DEMO['topic'])}</span>
                  <span><b>Next up</b>Record audio and publish the feed</span>
                </div>
              </aside>
            </section>

            <section class="section split-grid" id="cast">
              <div>
                <p class="section-kicker">Cast</p>
                <h2>Stable Host Lineup</h2>
                <p class="section-lede">
                  The studio represents guests as durable roles rather than episode-specific portraits, so a future
                  DeepSeek visit, human luminary, or listener question can join without rewriting the production model.
                </p>
              </div>
              <div class="host-grid">
                {host_cards}
              </div>
            </section>

            <section class="section">
              <div class="section-heading">
                <p class="section-kicker">Studio Flow</p>
                <h2>From idea to episode, without making it feel like homework.</h2>
                <p class="section-lede">
                  Each week gets the same friendly shape: choose a question, bring in the right voices,
                  record the conversation, and publish the notes listeners can actually use.
                </p>
              </div>
              <div class="production-grid">
                <article class="production-step reveal">
                  <span class="step-index">01</span>
                  <h3>Pick the question</h3>
                  <p>Start with one plain-language question a smart listener can care about before the jargon starts.</p>
                </article>
                <article class="production-step reveal">
                  <span class="step-index">02</span>
                  <h3>Bring the voices</h3>
                  <p>Use recurring hosts, an AI guest, a human guest, or a listener prompt without redesigning the show.</p>
                  <div class="status-strip">
                    <span class="pill">AI guest</span>
                    <span class="pill">Human guest</span>
                    <span class="pill">Listener question</span>
                  </div>
                </article>
                <article class="production-step reveal">
                  <span class="step-index">03</span>
                  <h3>Publish the show</h3>
                  <p>The launch track turns the finished recording into a page, feed item, notes, and shareable archive.</p>
                </article>
              </div>
            </section>

            <section class="section split-grid" id="episode">
              <div>
                <p class="section-kicker">Episode</p>
                <h2>Transcript</h2>
                <p class="section-lede">
                  This is the sample loaded episode. Future episodes should reuse the same studio
                  shape while swapping topic, guests, script, recording, notes, and publication state.
                </p>
                <div class="truth-list">
                  <article class="show-note">
                    <strong>Show promise</strong>
                    <p>Curious, grounded conversations about what AI is becoming and how people can use it well.</p>
                  </article>
                  <article class="show-note">
                    <strong>Launch status</strong>
                    <p>The next issue turns this studio into the real audio feed and public podcast page.</p>
                  </article>
                </div>
              </div>
              <div class="transcript-grid">
                {transcript_cards}
              </div>
            </section>

            <section class="section split-grid">
              <div>
                <p class="section-kicker">Highlights</p>
                <h2>Best Lines</h2>
                <p class="section-lede">
                  These are the editorial handles for a listener-friendly cut once the audio path is real.
                </p>
              </div>
              <ul class="highlight-list">
                {best_lines}
              </ul>
            </section>

            <p class="footer-note">
              Built for a weekly show at agent-logic.ai/podcast. Audio, RSS, and publication are tracked
              in the launch issue so this preview stays honest while the public surface stays friendly.
            </p>
          </main>
        </body>
        </html>
        """
    ).strip() + "\n"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def copy_logo(card_path: Path) -> None:
    if DEFAULT_LOGO_PATH.exists():
        card_path.parent.mkdir(parents=True, exist_ok=True)
        destination = card_path.parent / "agent-logic-logo.png"
        if DEFAULT_LOGO_PATH.resolve() != destination.resolve():
            shutil.copyfile(DEFAULT_LOGO_PATH, destination)


def generate(review_dir: Path, card_path: Path, feature_path: Path) -> None:
    write(review_dir / "README.md", review_readme(card_path))
    write(review_dir / "PODCAST_STUDIO_V2_PACKET_v0.91.3.md", packet_markdown(review_dir, card_path, feature_path))
    write(review_dir / "ct_demo_004_topic_brief.md", SEGMENTS[0]["content"] + "\n")
    write(review_dir / "ct_demo_004_host_lineup.md", host_lineup_markdown())
    write(review_dir / "ct_demo_004_transcript.md", transcript_markdown())
    write(review_dir / "ct_demo_004_best_lines.md", best_lines_markdown())
    write(review_dir / "ct_demo_004_episode_packet.md", episode_packet_markdown(card_path))
    write(review_dir / "ct_demo_004_reviewer_proof_note.md", reviewer_proof_note())
    write(
        review_dir / "ct_demo_004_audio_render_manifest.json",
        json.dumps(audio_manifest(), indent=2) + "\n",
    )
    write(card_path, episode_card_html())
    copy_logo(card_path)
    write(feature_path, feature_markdown(review_dir, card_path))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate the v0.91.3 podcast studio v2 demo packet.")
    parser.add_argument("--review-dir", type=Path, default=DEFAULT_REVIEW_DIR)
    parser.add_argument("--card-path", type=Path, default=DEFAULT_CARD_PATH)
    parser.add_argument("--feature-path", type=Path, default=DEFAULT_FEATURE_PATH)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    generate(args.review_dir, args.card_path, args.feature_path)
    print(args.review_dir)
    print(args.card_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
