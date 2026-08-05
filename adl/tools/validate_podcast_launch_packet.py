#!/usr/bin/env python3
import json
import hashlib
import sys
import wave
import xml.etree.ElementTree as ET
import urllib.request
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


FORBIDDEN_PUBLIC_TEXT = [
    "Packet status",
    "proof boundary",
    "truth boundary",
    "render_status",
    "C-SDLC",
    "manifest only",
    "not live-proven",
]

STUDIO_HTML = "podcast-studio.html"


def fail(message: str) -> None:
    raise SystemExit(f"podcast launch validation failed: {message}")


def reference_digest(path: Path) -> str:
    if not path.is_file():
        fail(f"missing studio reference digest manifest: {path}")
    for line in path.read_text(encoding="utf-8").splitlines():
        parts = line.split(maxsplit=1)
        if len(parts) == 2 and parts[1] == STUDIO_HTML:
            return parts[0]
    fail(f"studio reference digest manifest is missing {STUDIO_HTML}")


class LocalReferenceParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.refs: list[tuple[str, str, str]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attr_map = {key.lower(): value for key, value in attrs if value is not None}
        for attr in ("href", "src"):
            value = attr_map.get(attr)
            if value:
                self.refs.append((tag.lower(), attr, value))
        if tag.lower() == "meta" and attr_map.get("http-equiv", "").lower() == "refresh":
            content = attr_map.get("content", "")
            marker = "url="
            index = content.lower().find(marker)
            if index >= 0:
                self.refs.append(("meta", "content", content[index + len(marker) :].strip()))


def is_skipped_reference(ref: str) -> bool:
    stripped = ref.strip()
    if not stripped or stripped.startswith("#") or "{{" in stripped or "}}" in stripped:
        return True
    split = urlsplit(stripped)
    if split.scheme in {"http", "https", "mailto", "tel", "javascript", "data"}:
        return True
    return False


def resolve_local_reference(html_path: Path, ref: str) -> Path:
    split = urlsplit(ref.strip())
    raw_path = unquote(split.path)
    if not raw_path:
        return html_path
    return (html_path.parent / raw_path).resolve()


def validate_local_references(html_path: Path) -> None:
    parser = LocalReferenceParser()
    parser.feed(html_path.read_text(encoding="utf-8"))
    for tag, attr, ref in parser.refs:
        if is_skipped_reference(ref):
            continue
        target = resolve_local_reference(html_path, ref)
        if not target.exists():
            fail(f"{html_path} has broken local {tag} {attr} reference {ref!r}")


def validate_html_public_text(html_path: Path, require_audio: bool = False) -> None:
    text = html_path.read_text(encoding="utf-8")
    for forbidden in FORBIDDEN_PUBLIC_TEXT:
        if forbidden.lower() in text.lower():
            fail(f"public page contains internal/non-claim wording {forbidden!r}: {html_path}")
    if require_audio and "<audio controls" not in text:
        fail(f"missing playable audio control: {html_path}")
    validate_local_references(html_path)


def validate_feed(root: Path) -> None:
    feed = ET.parse(root / "feed.xml").getroot()
    channel = feed.find("./channel")
    if channel is None:
        fail("RSS feed is missing channel")
    title = channel.findtext("title", "")
    link = channel.findtext("link", "")
    if "Podcast" not in title:
        fail("RSS feed title does not identify a podcast")
    if link.rstrip("/") != "https://agent-logic.ai/podcast":
        fail("RSS feed link does not target the podcast route")
    enclosure = feed.find("./channel/item/enclosure")
    if enclosure is None:
        fail("RSS item is missing enclosure")
    if enclosure.attrib.get("type") != "audio/wav":
        fail("RSS enclosure must be audio/wav for the local launch proof")
    length = int(enclosure.attrib.get("length", "0"))
    actual = (root / "audio" / "meet-the-ai-coworkers.wav").stat().st_size
    if length != actual:
        fail(f"RSS enclosure length {length} does not match audio size {actual}")


def validate_http_route(http_base: str, route: str, contains: str) -> None:
    url = http_base.rstrip("/") + "/" + route.lstrip("/")
    with urllib.request.urlopen(url, timeout=5) as response:
        status = getattr(response, "status", response.getcode())
        body = response.read().decode("utf-8", errors="replace")
    if status != 200:
        fail(f"HTTP route {url} returned {status}")
    if contains not in body:
        fail(f"HTTP route {url} did not contain expected text {contains!r}")


def main() -> None:
    args = sys.argv[1:]
    if len(args) < 2:
        fail("usage: validate_podcast_launch_packet.py <podcast-root> <episodes-json> [--preview-root <path>] [--http-base <url>]")
    root = Path(args[0])
    episodes_path = Path(args[1])
    preview_root: Path | None = None
    http_base: str | None = None
    index = 2
    while index < len(args):
        if args[index] == "--preview-root" and index + 1 < len(args):
            preview_root = Path(args[index + 1])
            index += 2
        elif args[index] == "--http-base" and index + 1 < len(args):
            http_base = args[index + 1]
            index += 2
        else:
            fail(f"unknown argument: {args[index]}")
    packet = json.loads(episodes_path.read_text(encoding="utf-8"))
    episodes = packet.get("episodes") or []
    if len(episodes) != 10:
        fail("expected exactly 10 episode records")

    required = [
        root / "index.html",
        root / "episodes" / "meet-the-ai-coworkers" / "index.html",
        root / "feed.xml",
        root / "audio" / "meet-the-ai-coworkers.wav",
        root / "studio" / "index.html",
        root / "studio" / STUDIO_HTML,
        root / "studio" / "support.js",
        root / "studio" / "image-slot.js",
    ]
    for path in required:
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"missing required launch artifact: {path}")

    for html_path in [root / "index.html", root / "episodes" / "meet-the-ai-coworkers" / "index.html"]:
        validate_html_public_text(html_path, require_audio=True)
    if preview_root is not None:
        validate_html_public_text(preview_root / "index.html", require_audio=True)

    studio_html = root / "studio" / STUDIO_HTML
    studio_text = studio_html.read_text(encoding="utf-8")
    if "Synthetic Minds" not in studio_text or "{{ latest.title }}" not in studio_text:
        fail("studio reference HTML no longer looks like the operator-provided export")
    if '<script src="./support.js"></script>' not in studio_text:
        fail("studio reference HTML is not wired to its local support.js asset")
    digest_file = root / "studio" / "reference.sha256"
    if not digest_file.is_file():
        fail("studio route is missing reference.sha256")
    expected_digest = reference_digest(root / "studio" / "REFERENCE_DIGESTS.txt")
    generated_digest = digest_file.read_text(encoding="utf-8").split()[0]
    if generated_digest != expected_digest:
        fail("studio route reference.sha256 does not match source reference digest")
    actual_digest = hashlib.sha256(studio_html.read_bytes()).hexdigest()
    if expected_digest != actual_digest:
        fail("studio reference HTML digest does not match source reference digest")
    source_reference = root / "studio-reference" / STUDIO_HTML
    if source_reference.is_file():
        source_digest = hashlib.sha256(source_reference.read_bytes()).hexdigest()
        source_expected = reference_digest(root / "studio-reference" / "REFERENCE_DIGESTS.txt")
        if source_digest != source_expected or source_digest != actual_digest:
            fail("generated studio HTML is not byte-identical to the tracked studio reference")

    with wave.open(str(root / "audio" / "meet-the-ai-coworkers.wav"), "rb") as wav:
        duration = wav.getnframes() / wav.getframerate()
        if duration <= 1.0:
            fail("episode audio is too short to prove playable output")

    validate_feed(root)
    if http_base is not None:
        validate_http_route(http_base, "/podcast/", "Synthetic Minds Podcast")
        validate_http_route(http_base, "/podcast/feed.xml", "Synthetic Minds Podcast")
        validate_http_route(http_base, "/podcast/studio/podcast-studio.html", "Synthetic Minds Podcast")
        validate_http_route(http_base, "/_preview/podcast/", "Synthetic Minds Podcast")

    print("podcast_launch_packet: PASS")


if __name__ == "__main__":
    main()
