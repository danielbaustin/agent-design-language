#!/usr/bin/env python3
import argparse
import email.utils
import hashlib
import html
import json
import re
import shutil
from datetime import datetime, timezone
from pathlib import Path


TURN_FILES = [
    "01-chatgpt-opening.md",
    "02-gemini-challenge.md",
    "03-claude-reframe.md",
    "04-chatgpt-bridge.md",
    "05-gemini-deepening.md",
    "06-claude-closure.md",
]

STUDIO_HTML = "podcast-studio.html"


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def safe_slug(slug: str) -> str:
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", slug):
        raise SystemExit(f"unsafe episode slug: {slug}")
    return slug


def first_episode(packet: dict) -> dict:
    episodes = packet.get("episodes") or []
    if len(episodes) < 10:
        raise SystemExit("podcast launch packet requires 10 episode records")
    episode = episodes[0]
    if len(episode.get("turns") or []) != 6:
        raise SystemExit("first episode must contain six launch transcript turns")
    return episode


def write_audio_source(root: Path, episode: dict) -> None:
    podcast_dir = root / "out" / "podcast"
    podcast_dir.mkdir(parents=True, exist_ok=True)
    transcript_lines = [
        f"# {episode['title']}",
        "",
        f"Listener question: {episode['listener_question']}",
        "",
    ]
    turns = episode["turns"]
    for expected, turn in zip(TURN_FILES, turns):
        if turn["file"] != expected:
            raise SystemExit(f"turn file order mismatch: expected {expected}, got {turn['file']}")
        text = turn["text"].strip()
        (podcast_dir / expected).write_text(text + "\n", encoding="utf-8")
        transcript_lines.extend([f"## {turn['speaker']}", "", text, ""])
    (root / "transcript.md").write_text("\n".join(transcript_lines).rstrip() + "\n", encoding="utf-8")


def write_launch_pages(out_root: Path, packet: dict, audio_file: str, audio_bytes: int) -> None:
    show = packet["show"]
    episode = first_episode(packet)
    slug = safe_slug(episode["slug"])
    out_root.mkdir(parents=True, exist_ok=True)
    episode_root = out_root / "episodes" / slug
    episode_root.mkdir(parents=True, exist_ok=True)
    (out_root / "audio").mkdir(parents=True, exist_ok=True)

    rows = []
    for item in packet["episodes"]:
        item_slug = safe_slug(item["slug"])
        href = f"episodes/{item_slug}/"
        rows.append(
            f"<li><a href=\"{href}\">Episode {item['number']:02d}: {html.escape(item['title'])}</a>"
            f"<span>{html.escape(item['listener_question'])}</span></li>"
        )

    guest_bubbles = [
        ("ChatGPT", "Lead host"),
        ("Gemini", "Field reporter"),
        ("Claude", "Story voice"),
        ("DeepSeek", "Week 2 guest"),
        ("Human guest", "Future interview"),
        ("Listener", "Question of the week"),
    ]
    bubbles = "\n".join(
        f"<div class=\"guest\"><strong>{html.escape(name)}</strong><span>{html.escape(role)}</span></div>"
        for name, role in guest_bubbles
    )

    index = f"""<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <title>{html.escape(show['title'])}</title>
  <link rel=\"alternate\" type=\"application/rss+xml\" title=\"Agent Logic Podcast\" href=\"feed.xml\">
  <style>
    :root {{ color-scheme: light; --blue: #2563eb; --ink: #101113; --muted: #5b6472; --line: #d8deea; --wash: #f6f9ff; }}
    * {{ box-sizing: border-box; }}
    body {{ margin: 0; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: var(--ink); background: #ffffff; }}
    main {{ min-height: 100vh; }}
    .hero {{ padding: 42px 6vw 32px; background: linear-gradient(180deg, #ffffff 0%, var(--wash) 100%); border-bottom: 1px solid var(--line); }}
    .logo {{ width: min(420px, 80vw); height: auto; display: block; margin-bottom: 34px; }}
    .eyebrow {{ color: var(--blue); font-weight: 700; font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0; }}
    h1 {{ font-size: clamp(2.3rem, 5vw, 5.2rem); line-height: 0.94; max-width: 940px; margin: 12px 0 16px; letter-spacing: 0; }}
    .intro {{ max-width: 760px; color: var(--muted); font-size: 1.18rem; line-height: 1.65; }}
    .actions {{ display: flex; flex-wrap: wrap; gap: 12px; margin-top: 28px; }}
    .button {{ display: inline-flex; align-items: center; justify-content: center; min-height: 44px; padding: 0 18px; border: 1px solid var(--ink); text-decoration: none; color: var(--ink); font-weight: 700; border-radius: 6px; }}
    .button.primary {{ background: var(--ink); color: #fff; }}
    .section {{ padding: 34px 6vw; border-bottom: 1px solid var(--line); }}
    h2 {{ font-size: 1.55rem; margin: 0 0 18px; letter-spacing: 0; }}
    .guests {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 14px; max-width: 940px; }}
    .guest {{ min-height: 112px; border: 1px solid var(--line); border-radius: 999px; display: grid; place-items: center; text-align: center; padding: 20px; background: #fff; box-shadow: 0 18px 40px rgba(37,99,235,0.08); }}
    .guest span {{ color: var(--muted); font-size: 0.92rem; margin-top: 4px; }}
    .episode-list {{ list-style: none; padding: 0; margin: 0; display: grid; gap: 10px; max-width: 980px; }}
    .episode-list li {{ border-top: 1px solid var(--line); padding: 16px 0; }}
    .episode-list a {{ color: var(--ink); font-weight: 800; text-decoration: none; }}
    .episode-list span {{ display: block; color: var(--muted); margin-top: 4px; line-height: 1.45; }}
    audio {{ width: min(760px, 100%); margin-top: 12px; }}
  </style>
</head>
<body>
  <main>
    <section class=\"hero\">
      <img class=\"logo\" src=\"studio/uploads/agent-logic-logo.svg\" alt=\"Agent Logic\">
      <div class=\"eyebrow\">Weekly AI conversations</div>
      <h1>Agent Logic Podcast</h1>
      <p class=\"intro\">A reusable studio for friendly conversations with AI hosts, invited model guests, human guests, and listener questions.</p>
      <div class=\"actions\">
        <a class=\"button primary\" href=\"episodes/{slug}/\">Play episode 01</a>
        <a class=\"button\" href=\"studio/\">Open studio design</a>
        <a class=\"button\" href=\"feed.xml\">RSS feed</a>
      </div>
      <audio controls preload=\"metadata\" src=\"audio/{html.escape(audio_file)}\"></audio>
    </section>
    <section class=\"section\">
      <h2>Voices This Season</h2>
      <div class=\"guests\">{bubbles}</div>
    </section>
    <section class=\"section\">
      <h2>First Ten Episodes</h2>
      <ol class=\"episode-list\">
        {''.join(rows)}
      </ol>
    </section>
  </main>
</body>
</html>
"""
    (out_root / "index.html").write_text(index, encoding="utf-8")

    transcript = "\n".join(
        f"<h3>{html.escape(turn['speaker'])}</h3><p>{html.escape(turn['text'])}</p>"
        for turn in episode["turns"]
    )
    episode_page = f"""<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <title>Episode 01: {html.escape(episode['title'])}</title>
  <link rel=\"alternate\" type=\"application/rss+xml\" title=\"Agent Logic Podcast\" href=\"../../feed.xml\">
  <style>
    body {{ margin: 0; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #101113; }}
    main {{ max-width: 880px; margin: 0 auto; padding: 42px 24px 64px; }}
    a {{ color: #2563eb; }}
    h1 {{ font-size: clamp(2rem, 5vw, 4.4rem); line-height: 1; letter-spacing: 0; margin-bottom: 14px; }}
    .summary, p {{ color: #4b5563; line-height: 1.65; font-size: 1.05rem; }}
    audio {{ width: 100%; margin: 18px 0 28px; }}
    h3 {{ margin-top: 24px; }}
  </style>
</head>
<body>
  <main>
    <a href=\"../../\">Agent Logic Podcast</a>
    <h1>Episode 01: {html.escape(episode['title'])}</h1>
    <p class=\"summary\">{html.escape(episode['summary'])}</p>
    <audio controls preload=\"metadata\" src=\"../../audio/{html.escape(audio_file)}\"></audio>
    <h2>Transcript</h2>
    {transcript}
  </main>
</body>
</html>
"""
    (episode_root / "index.html").write_text(episode_page, encoding="utf-8")

    for item in packet["episodes"][1:]:
        item_slug = safe_slug(item["slug"])
        item_root = out_root / "episodes" / item_slug
        item_root.mkdir(parents=True, exist_ok=True)
        planned_page = f"""<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <title>Episode {item['number']:02d}: {html.escape(item['title'])}</title>
  <link rel=\"alternate\" type=\"application/rss+xml\" title=\"Agent Logic Podcast\" href=\"../../feed.xml\">
  <style>
    body {{ margin: 0; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #101113; }}
    main {{ max-width: 760px; margin: 0 auto; padding: 42px 24px 64px; }}
    a {{ color: #2563eb; }}
    h1 {{ font-size: clamp(2rem, 5vw, 4rem); line-height: 1; letter-spacing: 0; margin-bottom: 14px; }}
    p {{ color: #4b5563; line-height: 1.65; font-size: 1.05rem; }}
  </style>
</head>
<body>
  <main>
    <a href=\"../../\">Agent Logic Podcast</a>
    <h1>Episode {item['number']:02d}: {html.escape(item['title'])}</h1>
    <p>{html.escape(item['listener_question'])}</p>
    <p>This proposed episode is queued for the weekly launch calendar.</p>
  </main>
</body>
</html>
"""
        (item_root / "index.html").write_text(planned_page, encoding="utf-8")

    pub_dt = datetime.fromisoformat(episode["publish_date"]).replace(tzinfo=timezone.utc)
    rss_date = email.utils.format_datetime(pub_dt)
    item_url = f"{show['site_url']}/episodes/{slug}/"
    audio_url = f"{show['site_url']}/audio/{audio_file}"
    rss = f"""<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<rss version=\"2.0\">
  <channel>
    <title>{html.escape(show['title'])}</title>
    <link>{html.escape(show['site_url'])}</link>
    <description>{html.escape(show['description'])}</description>
    <language>en-us</language>
    <item>
      <title>Episode 01: {html.escape(episode['title'])}</title>
      <link>{html.escape(item_url)}</link>
      <guid isPermaLink=\"true\">{html.escape(item_url)}</guid>
      <pubDate>{rss_date}</pubDate>
      <description>{html.escape(episode['summary'])}</description>
      <enclosure url=\"{html.escape(audio_url)}\" length=\"{audio_bytes}\" type=\"audio/wav\" />
    </item>
  </channel>
</rss>
"""
    (out_root / "feed.xml").write_text(rss, encoding="utf-8")


def read_studio_reference_digest(reference_dir: Path) -> str:
    digest_path = reference_dir / "REFERENCE_DIGESTS.txt"
    if not digest_path.is_file():
        raise SystemExit(f"missing studio reference digest manifest: {digest_path}")
    for line in digest_path.read_text(encoding="utf-8").splitlines():
        parts = line.split(maxsplit=1)
        if len(parts) == 2 and parts[1] == STUDIO_HTML:
            return parts[0]
    raise SystemExit(f"studio reference digest manifest is missing {STUDIO_HTML}")


def copy_studio_reference(out_root: Path, reference_dir: Path) -> None:
    html_path = reference_dir / STUDIO_HTML
    if not html_path.is_file():
        raise SystemExit(f"missing studio reference HTML: {html_path}")
    expected_digest = read_studio_reference_digest(reference_dir)
    source_digest = hashlib.sha256(html_path.read_bytes()).hexdigest()
    if source_digest != expected_digest:
        raise SystemExit("studio reference HTML digest does not match REFERENCE_DIGESTS.txt")
    target = out_root / "studio"
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(reference_dir, target)
    copied_digest = hashlib.sha256((target / STUDIO_HTML).read_bytes()).hexdigest()
    if copied_digest != expected_digest:
        raise SystemExit("copied studio reference HTML digest does not match source manifest")
    (target / "reference.sha256").write_text(
        f"{expected_digest}  {STUDIO_HTML}\n",
        encoding="utf-8",
    )
    (target / "index.html").write_text(
        """<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="refresh" content="0; url=podcast-studio.html">
  <title>Podcast Studio Design</title>
  <style>
    body { margin: 0; min-height: 100vh; display: grid; place-items: center; font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }
    a { color: #2563eb; font-weight: 700; }
  </style>
</head>
<body>
  <a href="podcast-studio.html">Open Podcast Studio design</a>
</body>
</html>
""",
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--episodes", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--audio-source", type=Path, required=True)
    parser.add_argument("--audio-file", default="meet-the-ai-coworkers.wav")
    parser.add_argument("--audio-bytes", type=int, default=0)
    parser.add_argument(
        "--studio-reference",
        type=Path,
        default=Path(__file__).resolve().parents[2] / "demos" / "podcast" / "studio-reference",
    )
    args = parser.parse_args()
    packet = read_json(args.episodes)
    episode = first_episode(packet)
    write_audio_source(args.audio_source, episode)
    write_launch_pages(args.out, packet, args.audio_file, args.audio_bytes)
    copy_studio_reference(args.out, args.studio_reference)


if __name__ == "__main__":
    main()
