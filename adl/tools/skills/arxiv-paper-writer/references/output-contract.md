# Output Contract

The default `arxiv-paper-writer` artifact is markdown with these sections in this order:

```md
## Metadata
- Skill: arxiv-paper-writer
- Subject: <source packet target>
- Date: <UTC timestamp or calendar date>
- Output Location: <path or none>

## Target
- Mode: draft_from_source_packet | draft_section | review_claims_and_citations | revise_from_review_notes
- Source Packet: <path, doc, or inline target>
- Paper Domain: <explicit domain or unknown>
- Target Sections: <sections or none>

## Packet
- Produced Sections:
  - <title options / abstract / outline / draft sections / reviewer notes>
- Primary Draft Surface: <path or explicit none>
- Result: PASS | FAIL | PARTIAL | NOT_RUN

## Claim Boundary Report
- Unsupported Claims Present: true | false
- Claim Labels Used:
  - SUPPORTED
  - NEEDS_CITATION
  - NEEDS_EVIDENCE
  - AUTHOR_DECISION
  - REMOVE_OR_WEAKEN
- Notes: <bounded summary>

## Citation Gap Report
- Citations Invented: true | false
- Citation Gaps Present: true | false
- Missing Bibliographic Details: <bounded list or none>

## Submission Boundary
- Submission Attempted: true | false
- Publication Claimed: true | false
- Human Author Approval Required: true | false
- Reason: <must explain why submit/publish is out of scope>

## Follow-up
- Recommended Next Step: <one bounded next action or explicit none>
```

## Rules

- Do not claim arXiv submission or publication.
- Do not invent citations, authors, affiliations, acknowledgements, or results.
- Do not hide unsupported claims; mark them in the claim-boundary report.
- Do not emit raw secrets, raw prompts, raw tool arguments, or unjustified absolute host paths.
