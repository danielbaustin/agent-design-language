# Output Contract

The default `medium-article-writer` artifact is markdown with these sections in this order:

```md
## Metadata
- Skill: medium-article-writer
- Subject: <article brief target>
- Date: <UTC timestamp or calendar date>
- Output Location: <path or none>

## Target
- Mode: draft_from_brief_path | draft_from_brief_text | draft_from_demo_doc
- Brief Surface: <path, doc, or inline target>
- Audience: <explicit audience or unknown>
- Intended Angle: <one concise angle>

## Packet
- Produced Sections:
  - <outline / titles / draft / editorial notes / reviewer summary>
- Primary Draft Surface: <path or explicit none>
- Result: PASS | FAIL | PARTIAL | NOT_RUN

## Medium Rules Applied
- Title Quality: <what was enforced>
- Lead Quality: <what was enforced>
- Structure and Readability: <what was enforced>
- Editorial Notes: <what was surfaced>

## Publication Boundary
- Publication Attempted: true | false
- Reason: <must explain why publish is out of scope>

## Follow-up
- Recommended Next Step: <one bounded next action or explicit none>
```

## Rules

- Do not claim publication or platform posting.
- Do not claim audience or performance outcomes as facts.
- Keep the output reviewable and bounded.
- Do not emit raw secrets, raw prompts, raw tool arguments, or unjustified absolute host paths.
