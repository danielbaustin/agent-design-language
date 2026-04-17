# Renderer Setup

The `diagram-author` skill is source-first and SVG-first.

Mermaid source remains the default for GitHub, PRs, issues, Markdown docs, and
Codex chat because those surfaces can render Mermaid inline. External rendered
assets are optional unless the caller asks for `render_required`.

## Recommended macOS Install

Use Homebrew for the current CLI renderer path:

```sh
brew install mermaid-cli d2 plantuml graphviz librsvg
```

This provides:

- `mmdc` for Mermaid source to SVG.
- `d2` for D2 source to SVG.
- `plantuml` for PlantUML source to SVG.
- `dot` from Graphviz for PlantUML diagram families that need DOT layout.
- `rsvg-convert` from librsvg for SVG to PNG rasterization.

Optional:

```sh
brew install structurizr-cli
```

Structurizr is useful for C4/model-consistent architecture packets, but treat it
as specialized. Homebrew marks `structurizr-cli` as deprecated because upstream
is archived, so it should not be required for the default ADL diagram path.

## First-Run Browser Caches

Some renderers use browser engines under the hood:

- Mermaid CLI uses Puppeteer.
- D2 may use Playwright for some raster paths.

The ADL renderer harness keeps browser caches repo-local by default:

```sh
ADL_DIAGRAM_RENDER_CACHE=.adl/.cache/diagram-renderers \
  adl/tools/skills/diagram-author/scripts/render_diagrams.sh \
  --input .adl/docs/TBD/diagram-test \
  --out .adl/docs/TBD/diagram-test/rendered \
  --formats svg,png
```

If Mermaid CLI asks for a Puppeteer browser, install it into the same repo-local
cache:

```sh
npm_config_cache=.adl/.cache/npm \
PUPPETEER_CACHE_DIR=.adl/.cache/diagram-renderers/puppeteer \
npx puppeteer browsers install chrome-headless-shell@131.0.6778.204
```

The exact Chrome version can change with `mermaid-cli`. If `mmdc` reports a
different required version, install that version instead.

## Sandbox Notes

Headless browsers may fail in restricted macOS sandboxes even when installed.
The symptom is usually a Chromium mach-service or browser-launch error.

Recommended behavior:

- Prefer Mermaid inline preview for chat/GitHub surfaces.
- Render D2 and PlantUML through the harness when those CLIs work locally.
- Use `--skip-backends mermaid` in restricted sandboxes where Chromium cannot
  launch.
- Run Mermaid rendering from a normal terminal if the sandbox blocks Chromium.

Example:

```sh
adl/tools/skills/diagram-author/scripts/render_diagrams.sh \
  --input .adl/docs/TBD/diagram-test \
  --out .adl/docs/TBD/diagram-test/rendered \
  --formats svg,png \
  --skip-backends mermaid
```

## User Assistance Model

The skill should help users with rendering in this order:

1. Run `render_diagrams.sh --check-tools` and report missing tools.
2. Recommend the smallest install set for the requested backends.
3. Keep browser and npm caches under `.adl/.cache/` when possible.
4. Prefer SVG as the durable visual artifact.
5. Derive PNG from SVG with `rsvg-convert` or ImageMagick rather than asking
   each renderer for native raster output.
6. Record `SKIP` for optional missing or sandbox-blocked renderers instead of
   pretending a rendered artifact exists.

## Rust Integration Options

Current CLI renderers are the safest first-class path because they match the
upstream diagram languages.

Potential Rust-backed improvements:

- `mermaid-rs-renderer`: promising pure-Rust Mermaid SVG/PNG renderer. This is
  the most interesting candidate for reducing the Node/Puppeteer dependency in
  ADL's default path.
- `resvg` / `usvg`: useful for SVG to PNG conversion if ADL wants a Rust-native
  rasterization step instead of shelling out to `rsvg-convert` or ImageMagick.
- `graphviz-rust` / related crates: useful for DOT parsing/generation, but not
  a replacement for full D2, Mermaid, PlantUML, or Structurizr rendering.
- `mdbook-d2` and `mdbook-plantuml`: useful reference points for docs pipelines,
  but they are mdBook integrations rather than general ADL renderer engines.
- PlantUML crates mostly encode URLs, parse syntax, or call PlantUML server; the
  local high-fidelity renderer is still the Java `plantuml` CLI.

Recommendation: keep the current CLI harness for v0.90, then open a follow-up
issue to evaluate `mermaid-rs-renderer` as an optional Rust-native Mermaid path.
