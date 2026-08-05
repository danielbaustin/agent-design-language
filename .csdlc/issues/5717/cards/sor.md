# Structured Output Record

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Prepared the Synthetic Minds Podcast surface for launch testing: promoted the polished studio design to the production `/podcast/` entrypoint, added an unlisted `/_preview/podcast/` route with noindex/nofollow, updated RSS metadata and owner/contact truth, aligned the first episode page naming, and added a launch-readiness packet covering gates 1-8 while preserving human review as post-PR.

## Artifacts

- .csdlc/evidence/5717
- /Volumes/FastWork/adl-podcast-launch-5717/audio-render/audio_manifest.json
- http://127.0.0.1:8915/studio/podcast-studio.html
- demos/podcast/LAUNCH_READINESS.md
- http://127.0.0.1:8916/podcast/
- http://127.0.0.1:8916/_preview/podcast/
- http://127.0.0.1:8916/podcast/feed.xml
- http://127.0.0.1:8916/podcast/audio/meet-the-ai-coworkers.wav

## Execution

- .csdlc/issues/5715/index.json
- .csdlc/issues/5715/audit.jsonl
- .csdlc/issues/5717
- .csdlc/prepared/issues/5717
- demos/podcast/studio-reference/REFERENCE_DIGESTS.txt
- demos/podcast/studio-reference/podcast-studio.html
- demos/podcast/studio-reference/uploads/agent-logic-logo.svg
- demos/podcast/studio/REFERENCE_DIGESTS.txt
- demos/podcast/studio/podcast-studio.html
- demos/podcast/studio/reference.sha256
- demos/podcast/studio/uploads/agent-logic-logo.svg
- .csdlc/issues/5717
- .csdlc/prepared/issues/5717
- demos/_preview/podcast/index.html
- demos/podcast/LAUNCH_READINESS.md
- demos/podcast/episodes/meet-the-ai-coworkers/index.html
- demos/podcast/feed.xml
- demos/podcast/index.html
- demos/podcast/studio/podcast-studio.html
- demos/podcast/studio/uploads/agent-logic-logo.svg

## Validation

[
  {
    "command": [
      "python3",
      "-c",
      "from pathlib import Path\npaths=[Path('demos/podcast/studio-reference/podcast-studio.html'),Path('demos/podcast/studio/podcast-studio.html')]\nrequired=['<title>Synthetic Minds Podcast</title>','Synthetic <span style=\"color:oklch(55% 0.2 265); font-weight:600;\">Minds</span> Podcast','Special guests join us occasionally.','href=\"mailto:podcast@agent-logic.ai\"','Contact the studio','href=\"../feed.xml\"','Frequently Asked Questions','No. Synthetic Minds Podcast is audio-first for launch.','agent-logic-logo.svg','height:56px','height:34px','num: 1','num: 10','DeepSeek Drops By','© 2026 Agent Logic, Inc.</div>']\nfor path in paths:\n    text=path.read_text()\n    footer=text.split('<!-- FOOTER -->',1)[1]\n    missing=[s for s in required if s not in text]\n    forbidden=[s for s in ['href=\"#\"','<svg width=\"18\"','translate(120, 190)','New guests drop in most weeks','num: 42','num: 41','num: 40','num: 39','num: 38','Most episodes ship as audio-first; select episodes get a full video cut.','Frequently asked questions','YouTube'] if s in text]\n    if 'podcast@agent-logic.ai</a>' in footer:\n        forbidden.append('bare footer email link')\n    if missing or forbidden:\n        raise SystemExit(f'{path}: missing={missing} forbidden={forbidden}')\nfor path in [Path('demos/podcast/studio-reference/uploads/agent-logic-logo.svg'),Path('demos/podcast/studio/uploads/agent-logic-logo.svg')]:\n    text=path.read_text()\n    forbidden=[s for s in ['translate(120, 190)'] if s in text]\n    if forbidden:\n        raise SystemExit(f'{path}: forbidden={forbidden}')\nfor path in [Path('demos/podcast/feed.xml'),Path('demos/podcast/audio/meet-the-ai-coworkers.wav')]:\n    if not path.exists():\n        raise SystemExit(f'missing linked asset: {path}')\nprint('studio_copy_contract: PASS')"
    ],
    "purpose": "Prove the reference and served studio HTML contain the operator-requested copy/logo/episode/video/footer/contact fixes, prove the subscribe/contact links are wired, and prove no stale fake episode, video-platform copy, footer email link, or old inline logo artifact remains.",
    "outcome": "passed",
    "evidence_ref": "podcast-studio-copy-contract.log"
  },
  {
    "command": [
      "python3",
      "- <<'PY' ... podcast launch structural validation ... PY"
    ],
    "purpose": "Validate required `/podcast/` and `/_preview/podcast/` source files, hidden-route noindex truth, contact link, RSS owner/title/enclosure metadata, enclosure byte length, and local HTML href/src references.",
    "outcome": "passed",
    "evidence_ref": "commentary: podcast launch structural validation PASS"
  },
  {
    "command": [
      "zsh",
      "-c",
      "for url in http://127.0.0.1:8916/podcast/ http://127.0.0.1:8916/_preview/podcast/ http://127.0.0.1:8916/podcast/feed.xml http://127.0.0.1:8916/podcast/audio/meet-the-ai-coworkers.wav; do curl -s -o /dev/null -w \"$url %{http_code} %{content_type} %{size_download}\\n\" \"$url\"; done"
    ],
    "purpose": "Smoke-test the hosted route shape from a local server rooted at `demos/`: production page, hidden preview, RSS feed, and audio enclosure all return 200 with expected content types and byte counts.",
    "outcome": "passed",
    "evidence_ref": "commentary: /podcast 200 text/html 29237; /_preview/podcast 200 text/html 29361; /podcast/feed.xml 200 application/xml 1881; audio WAV 200 audio/x-wav 417644"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify launch-prep edits and typed lifecycle projections have no whitespace errors.",
    "outcome": "passed",
    "evidence_ref": "command output: no output, exit 0"
  },
  {
    "command": [
      "python3",
      "- <<'PY' ... podcast launch structural validation with review-fix assertions ... PY"
    ],
    "purpose": "Revalidate the podcast launch-prep packet after pre-PR review fixes: Friday planned cadence, human guest invitation, directory availability as after-approval truth, smoke sample separated from proposed episodes, RSS trailer metadata, route files, feed enclosure length, contact button, and local asset references.",
    "outcome": "passed",
    "evidence_ref": "commentary: podcast launch structural validation PASS; HTTP deploy-shape smoke returned 200 for /podcast/, /_preview/podcast/, feed.xml, and WAV audio; git diff --check PASS"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
