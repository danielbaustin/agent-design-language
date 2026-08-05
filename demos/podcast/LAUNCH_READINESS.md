# Synthetic Minds Podcast Launch Readiness

This packet prepares the podcast surface for hidden-route testing before launch.

## Routes

- Production launch route: `/podcast/`
- Hidden test route: `/_preview/podcast/`
- RSS feed: `/podcast/feed.xml`
- First episode page: `/podcast/episodes/meet-the-ai-coworkers/`

The hidden test route is intentionally unlinked from the site navigation and
declares `noindex,nofollow`.

## Current Launch Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| Hidden test URL | Ready | `demos/_preview/podcast/index.html` |
| Production route target | Ready | `demos/podcast/index.html` |
| RSS feed | Ready for local/feed-reader smoke | `demos/podcast/feed.xml` |
| Audio playback | Ready for smoke | `demos/podcast/audio/meet-the-ai-coworkers.wav` |
| First ten topics | Drafted | episode list in `demos/podcast/index.html` |
| Guest workflow | Page-ready | contact button uses `mailto:podcast@agent-logic.ai`; FAQ invites guest suggestions |
| Contact path | Ready pending mailbox verification | `podcast@agent-logic.ai` appears only in CTA/FAQ/feed owner metadata |
| Final launch route | Planned | promote `/podcast/` as the public route after review |

## Audio Truth

The current audio file is a short WAV smoke sample, not a full final episode.
It is used to prove browser playback and RSS enclosure wiring. Final launch
audio can replace the file at the same route or switch the feed enclosure to a
production MP3/M4A export once the production audio pipeline emits one.

## Directory Submission Prerequisites

Apple Podcasts, Spotify, YouTube Music, and similar directories still require
live public hosting and account-side setup after this PR lands. Before
submission, verify:

- `https://agent-logic.ai/podcast/feed.xml` is publicly reachable over HTTPS.
- `https://agent-logic.ai/podcast/audio/meet-the-ai-coworkers.wav` or its
  production replacement is publicly reachable and stable.
- `podcast@agent-logic.ai` receives verification mail for RSS ownership checks.
- Final show artwork is present in the feed and satisfies directory artwork
  requirements.
- The first submitted episode uses final approved audio, title, description,
  publish date, and content-rights truth.

Directory availability is not claimed by this packet. It prepares the source
routes and feed shape that those submissions will consume.

## Human Review

Human review is intentionally after PR publication. This packet does not claim
that the page, episode order, or final launch copy has had final human approval.
