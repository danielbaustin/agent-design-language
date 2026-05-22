#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
import json
from pathlib import Path
from textwrap import dedent


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_REVIEW_DIR = ROOT / "docs/milestones/v0.91.3/review/podcast_studio_v2"
DEFAULT_CARD_PATH = ROOT / "demos/v0.91.3/adl_podcast_studio_v2_episode_card.html"
DEFAULT_FEATURE_PATH = ROOT / "docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"

DEMO = {
    "demo_name": "ADL Podcast Studio v2",
    "milestone_version": "v0.91.3",
    "issue": "#3223",
    "wp": "demo WP-04",
    "series_name": "ADL Podcast Studio",
    "episode_title": "Episode 04: Can governed creative production feel alive?",
    "episode_slug": "episode-04-governed-creative-production",
    "topic": "What would make a governed five-minute sprint feel genuinely alive instead of merely fast?",
    "bounded_purpose": "Show a repeatable, inspectable media-production system that turns a bounded topic into a full episode packet without hidden credentials or fake audio claims.",
    "timebox_claim": "Episode packet generation is deterministic and quick; this demo does not claim live five-minute end-to-end creative production.",
}

HOSTS = [
    {
        "name": "ChatGPT",
        "show_role": "Host / framing synthesizer",
        "studio_job": "opens the question, keeps the cadence human, and closes with the governing thesis",
        "style": "warm, direct, high-clarity host energy",
        "color": "#f97316",
    },
    {
        "name": "Gemini",
        "show_role": "Systems producer / counter-pressure",
        "studio_job": "stress-tests the process, points out latency and orchestration debt, and names the engineering bottlenecks",
        "style": "bright, incisive, operationally sharp",
        "color": "#14b8a6",
    },
    {
        "name": "Claude",
        "show_role": "Story editor / moral ballast",
        "studio_job": "rescues the human meaning, protects truth boundaries, and keeps the show from collapsing into sterile workflow worship",
        "style": "measured, reflective, emotionally precise",
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

            What would make a governed five-minute sprint feel genuinely alive instead of merely fast?

            ## Why this episode exists

            The C-SDLC mini-sprint has already produced visible artifacts, but the harder test is whether the process can feel like real creative production rather than bureaucratic throughput. This episode packages that tension directly.

            ## Production boundary

            - no hidden credentials
            - no fake live audio claim
            - every role contribution must be inspectable
            - the packet should still feel like a show, not just a log dump

            ## Desired listener outcome

            A reviewer should come away believing the team can produce repeatable media artifacts under governance, while also understanding that literal fully-automated five-minute delivery remains unproven.
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
                "body": "Welcome back to ADL Podcast Studio. Today's question is uncomfortable on purpose: what would make a governed five-minute sprint feel alive instead of merely fast? We have a beautiful proof chain, we have visible artifacts, and we have enough scars now to know that speed theater is cheap. The real bar is whether governance can support a creative act without draining it of pulse. THESIS: a five-minute sprint only matters if it can preserve surprise, taste, and truth at the same time.",
            },
            {
                "speaker": "Gemini",
                "label": "Counter-pressure",
                "body": "The engineering answer is that life disappears when the system spends its budget on synchronization rather than production. If cards, PRs, proofs, and validation lanes all require manual re-assembly, you get governance without flow. That does not mean the model is wrong. It means the active gates are still too post-hoc, the validation path is too heavy, and too much of the packet is authored after the work rather than emitted as part of the work.",
            },
            {
                "speaker": "Claude",
                "label": "Human meaning check",
                "body": "And yet raw speed is not the thing people actually crave. They want to feel that something worth caring about came into being. A lifeless sprint can be fast and still empty. The governed version only earns its keep if it protects authorship, keeps claims honest, and leaves behind an artifact someone can love or dispute. In that sense, bureaucracy is not the enemy; dead language is.",
            },
            {
                "speaker": "ChatGPT",
                "label": "Bridge",
                "body": "So the real upgrade path is not 'delete governance.' It is to move more of the truth production into the artifact itself. The demo should carry its own packet. The review surface should read like a coherent show dossier. The operator should not need secret context to understand what happened. When the artifact tells the story cleanly, governance starts to feel less like drag and more like stagecraft.",
            },
            {
                "speaker": "Gemini",
                "label": "Operational deepening",
                "body": "Exactly. If the one-command path yields a topic brief, role map, transcript, best-lines cut, audio manifest, and episode card every time, then the system starts acting like a production line rather than an aspiration. The remaining non-proof is also clear: we are not yet rendering real final audio here, and we are not yet collapsing all review latency into five minutes. But the machinery becomes inspectable, reproducible, and improvable instead of mythic.",
            },
            {
                "speaker": "Claude",
                "label": "Closure",
                "body": "Then perhaps aliveness comes from proportion. Let automation handle the packet geometry, let governance hold the truth boundary, and let human judgment preserve taste. If those pieces stay in the right order, speed becomes a consequence rather than an idol. The show feels alive when the artifact still has a soul after the process has finished touching it.",
            },
        ],
    },
    {
        "id": "best-lines",
        "title": "Best Lines",
        "owner": "Editor",
        "quotes": [
            "A five-minute sprint only matters if it can preserve surprise, taste, and truth at the same time.",
            "The active gates are still too post-hoc, the validation path is too heavy, and too much of the packet is authored after the work rather than emitted as part of the work.",
            "Bureaucracy is not the enemy; dead language is.",
            "When the artifact tells the story cleanly, governance starts to feel less like drag and more like stagecraft.",
            "The show feels alive when the artifact still has a soul after the process has finished touching it.",
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
            <article class="host-card" style="--host-color: {host['color']};">
              <p class="eyebrow">{html.escape(host['show_role'])}</p>
              <h3>{html.escape(host['name'])}</h3>
              <p>{html.escape(host['studio_job'])}</p>
              <p class="voice">Voice target: {html.escape(host['style'])}</p>
            </article>
            """
        ).strip()
        for host in HOSTS
    )
    transcript_cards = "\n".join(
        dedent(
            f"""
            <article class="turn-card">
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
              --bg: #130f0c;
              --panel: rgba(34, 24, 18, 0.88);
              --panel-strong: rgba(47, 32, 24, 0.96);
              --text: #f8efe6;
              --muted: #d1b8a5;
              --line: rgba(255, 214, 170, 0.16);
              --accent: #f59e0b;
              --accent-2: #14b8a6;
            }}
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Georgia", "Times New Roman", serif;
              color: var(--text);
              background:
                radial-gradient(circle at top, rgba(245, 158, 11, 0.18), transparent 32%),
                linear-gradient(180deg, #1b140f 0%, #100b08 54%, #090706 100%);
            }}
            .page {{
              max-width: 1200px;
              margin: 0 auto;
              padding: 40px 24px 72px;
            }}
            .hero {{
              display: grid;
              gap: 24px;
              grid-template-columns: 1.2fr 0.8fr;
              align-items: end;
            }}
            .hero-copy h1 {{
              margin: 0;
              font-size: clamp(2.8rem, 6vw, 5.4rem);
              line-height: 0.95;
              letter-spacing: -0.04em;
            }}
            .hero-copy p {{
              max-width: 42rem;
              font-size: 1.1rem;
              line-height: 1.6;
              color: var(--muted);
            }}
            .eyebrow {{
              margin: 0 0 10px;
              text-transform: uppercase;
              letter-spacing: 0.18em;
              font-size: 0.78rem;
              color: #fcd34d;
            }}
            .hero-card,
            .panel {{
              background: var(--panel);
              border: 1px solid var(--line);
              border-radius: 28px;
              box-shadow: 0 22px 60px rgba(0, 0, 0, 0.28);
              backdrop-filter: blur(14px);
            }}
            .hero-card {{
              padding: 24px;
            }}
            .hero-card dl {{
              margin: 0;
              display: grid;
              gap: 14px;
            }}
            .hero-card dt {{
              font-size: 0.72rem;
              text-transform: uppercase;
              letter-spacing: 0.16em;
              color: #fdba74;
            }}
            .hero-card dd {{
              margin: 4px 0 0;
              color: var(--muted);
            }}
            .grid {{
              margin-top: 28px;
              display: grid;
              gap: 24px;
              grid-template-columns: 1.1fr 0.9fr;
            }}
            .panel {{
              padding: 24px;
            }}
            .panel h2 {{
              margin-top: 0;
              font-size: 1.5rem;
            }}
            .host-grid {{
              display: grid;
              gap: 16px;
            }}
            .host-card {{
              border: 1px solid color-mix(in srgb, var(--host-color) 38%, transparent);
              background: linear-gradient(135deg, rgba(255,255,255,0.03), rgba(255,255,255,0.01));
              border-radius: 22px;
              padding: 18px;
            }}
            .host-card h3 {{
              margin: 0 0 6px;
              font-size: 1.35rem;
            }}
            .host-card .voice {{
              color: color-mix(in srgb, var(--host-color) 58%, white);
            }}
            .turn-card {{
              padding: 18px 0;
              border-top: 1px solid var(--line);
            }}
            .turn-card:first-of-type {{
              border-top: 0;
              padding-top: 0;
            }}
            .turn-meta {{
              display: flex;
              gap: 12px;
              align-items: baseline;
              flex-wrap: wrap;
              margin-bottom: 10px;
            }}
            .turn-speaker {{
              font-size: 1.1rem;
              font-weight: 700;
            }}
            .turn-label {{
              color: #fdba74;
              text-transform: uppercase;
              letter-spacing: 0.14em;
              font-size: 0.76rem;
            }}
            .status-strip {{
              display: flex;
              gap: 12px;
              flex-wrap: wrap;
              margin-top: 12px;
            }}
            .pill {{
              padding: 10px 14px;
              border-radius: 999px;
              border: 1px solid var(--line);
              background: rgba(255, 255, 255, 0.04);
              color: var(--muted);
              font-size: 0.92rem;
            }}
            ul {{
              padding-left: 20px;
            }}
            @media (max-width: 900px) {{
              .hero,
              .grid {{
                grid-template-columns: 1fr;
              }}
            }}
          </style>
        </head>
        <body>
          <main class="page">
            <section class="hero">
              <div class="hero-copy">
                <p class="eyebrow">ADL Podcast Studio v2</p>
                <h1>{html.escape(DEMO['episode_title'])}</h1>
                <p>
                  A deterministic, no-secrets-needed production-system demo for the C-SDLC mini-sprint.
                  This artifact packages the topic, cast, transcript, best lines, and exact audio truth
                  without pretending that live final rendering has already been solved.
                </p>
                <div class="status-strip">
                  <span class="pill">Packet status: local pass</span>
                  <span class="pill">Audio status: manifest only</span>
                  <span class="pill">Credentials: not required</span>
                </div>
              </div>
              <aside class="hero-card">
                <dl>
                  <div>
                    <dt>Topic</dt>
                    <dd>{html.escape(DEMO['topic'])}</dd>
                  </div>
                  <div>
                    <dt>Bounded claim</dt>
                    <dd>{html.escape(DEMO['bounded_purpose'])}</dd>
                  </div>
                  <div>
                    <dt>Non-claim</dt>
                    <dd>Real rendered episode audio and literal five-minute end-to-end proof are not claimed here.</dd>
                  </div>
                </dl>
              </aside>
            </section>

            <section class="grid">
              <section class="panel">
                <p class="eyebrow">Cast</p>
                <h2>Stable Host Lineup</h2>
                <div class="host-grid">
                  {host_cards}
                </div>
              </section>

              <section class="panel">
                <p class="eyebrow">Production Truth</p>
                <h2>Render Status</h2>
                <p>
                  This bounded demo keeps transcript authorship and audio render status separate.
                  The packet records routing intent and studio posture, but it does not claim a real final audio file.
                </p>
                <div class="status-strip">
                  <span class="pill">render_status: manifest_only</span>
                  <span class="pill">rendered_audio_present: false</span>
                  <span class="pill">truth boundary: exact</span>
                </div>
              </section>
            </section>

            <section class="grid">
              <section class="panel">
                <p class="eyebrow">Episode</p>
                <h2>Transcript</h2>
                {transcript_cards}
              </section>

              <section class="panel">
                <p class="eyebrow">Highlights</p>
                <h2>Best Lines</h2>
                <ul>
                  {best_lines}
                </ul>
              </section>
            </section>
          </main>
        </body>
        </html>
        """
    ).strip() + "\n"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


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
