# Issue 5715 design

Integrate the operator-provided podcast studio page export into the current
Agent Logic podcast route while preserving the audio and RSS launch behavior
already landed by #5711.

The exported page is an immutable artifact for this issue. The implementation
must use its exact HTML text and referenced images, with no content rewrite,
copy normalization, brand copy replacement, or in-place HTML editing. The only
allowed changes around the export are route and filename wiring: the podcast
landing page links to `studio/`, `studio/` opens the exported HTML under the
clean filename `podcast-studio.html`, and validation anchors the copied file to
the committed reference digest.

The change is bounded to the podcast launch/demo generator, generated podcast
demo pages/feed, validation tooling, copied studio reference assets, and
issue-local lifecycle evidence. It does not claim production deployment,
replace the proven audio/RSS path, edit the exported HTML content, or commit the
operator's zip unless separately authorized.
