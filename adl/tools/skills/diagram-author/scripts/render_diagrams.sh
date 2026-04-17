#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: render_diagrams.sh --input DIR --out DIR [options]

Render diagram-author source files when local renderers are available.

Options:
  --input DIR       Directory containing diagram sources.
  --out DIR         Output directory for rendered artifacts and manifest.
  --formats LIST    Comma-separated formats. Supported: svg,png. Default: svg.
  --skip-backends LIST
                   Comma-separated backends to skip. Supported: mermaid,d2,plantuml,structurizr.
  --required        Fail when a renderer required for a discovered source is missing.
  --dry-run         Report planned actions without rendering.
  --check-tools     Print renderer availability and exit.
  -h, --help        Show this help.

Supported inputs:
  Mermaid:      .mmd, .mermaid, and markdown files with ```mermaid fences
  D2:           .d2
  PlantUML:     .puml, .plantuml
  Structurizr:  .dsl (validate/export source model; raster output is downstream)
USAGE
}

input_dir=""
out_dir=""
formats="svg"
required=false
dry_run=false
check_tools=false
skip_backends=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --input)
      input_dir="${2:-}"
      shift 2
      ;;
    --out)
      out_dir="${2:-}"
      shift 2
      ;;
    --formats)
      formats="${2:-}"
      shift 2
      ;;
    --skip-backends)
      skip_backends="${2:-}"
      shift 2
      ;;
    --required)
      required=true
      shift
      ;;
    --dry-run)
      dry_run=true
      shift
      ;;
    --check-tools)
      check_tools=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "render_diagrams: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

have() {
  command -v "$1" >/dev/null 2>&1
}

tool_status() {
  local tool="$1"
  if have "$tool"; then
    printf '%s\tavailable\t%s\n' "$tool" "$(command -v "$tool")"
  else
    printf '%s\tmissing\t-\n' "$tool"
  fi
}

if [[ "$check_tools" == true ]]; then
  tool_status mmdc
  tool_status d2
  tool_status plantuml
  tool_status structurizr
  tool_status convert
  exit 0
fi

if [[ -z "$input_dir" || -z "$out_dir" ]]; then
  echo "render_diagrams: --input and --out are required" >&2
  usage >&2
  exit 2
fi

if [[ ! -d "$input_dir" ]]; then
  echo "render_diagrams: input directory not found: $input_dir" >&2
  exit 2
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cache_root="${ADL_DIAGRAM_RENDER_CACHE:-${repo_root}/.adl/.cache/diagram-renderers}"
mkdir -p "$cache_root"

# Keep renderer browser downloads local to the repository unless callers opt out.
# Mermaid CLI uses Puppeteer; D2 may use Playwright for raster output.
export PUPPETEER_CACHE_DIR="${PUPPETEER_CACHE_DIR:-${cache_root}/puppeteer}"
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-${cache_root}/playwright}"

case ",${formats}," in
  *",svg,"*|*",png,"*) ;;
  *)
    echo "render_diagrams: --formats must include svg and/or png" >&2
    exit 2
    ;;
esac

for format in ${formats//,/ }; do
  case "$format" in
    svg|png) ;;
    *)
      echo "render_diagrams: unsupported format: $format" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$out_dir"
manifest="${out_dir}/render-manifest.tsv"
tmp_dir="${out_dir}/.tmp"
mkdir -p "$tmp_dir"

printf 'backend\tsource\tstatus\tartifact\tnote\n' > "$manifest"

record() {
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" "$5" >> "$manifest"
}

missing_tool() {
  local backend="$1"
  local source="$2"
  local tool="$3"
  record "$backend" "$source" "SKIP" "-" "missing renderer: ${tool}"
  if [[ "$required" == true ]]; then
    echo "render_diagrams: required renderer missing for ${source}: ${tool}" >&2
    exit 1
  fi
}

want_format() {
  case ",${formats}," in
    *",$1,"*) return 0 ;;
    *) return 1 ;;
  esac
}

backend_skipped() {
  case ",${skip_backends}," in
    *",$1,"*) return 0 ;;
    *) return 1 ;;
  esac
}

png_from_svg() {
  local svg="$1"
  local png="$2"
  local source="$3"
  local backend="$4"

  if [[ "$dry_run" == true ]]; then
    record "$backend" "$source" "PLAN" "$png" "would derive PNG from SVG"
    return
  fi

  if have rsvg-convert; then
    rsvg-convert "$svg" -o "$png"
    record "$backend" "$source" "PASS" "$png" "derived PNG from SVG with rsvg-convert"
  elif have convert; then
    convert "$svg" "$png"
    record "$backend" "$source" "PASS" "$png" "derived PNG from SVG with convert"
  else
    record "$backend" "$source" "SKIP" "$png" "missing raster converter: rsvg-convert or convert"
    if [[ "$required" == true ]]; then
      echo "render_diagrams: required raster converter missing: rsvg-convert or convert" >&2
      exit 1
    fi
  fi
}

render_mermaid_file() {
  local source="$1"
  local stem="$2"
  local svg="${out_dir}/${stem}.svg"
  local png="${out_dir}/${stem}.png"

  if ! have mmdc; then
    missing_tool "mermaid" "$source" "mmdc"
    return
  fi

  if want_format svg; then
    if [[ "$dry_run" == true ]]; then
      record "mermaid" "$source" "PLAN" "$svg" "would render SVG with mmdc"
    else
      if mmdc -i "$source" -o "$svg"; then
        record "mermaid" "$source" "PASS" "$svg" "rendered SVG with mmdc"
      else
        record "mermaid" "$source" "FAIL" "$svg" "mmdc SVG render failed"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
  fi

  if want_format png; then
    if [[ ! -f "$svg" && "$dry_run" != true ]]; then
      if mmdc -i "$source" -o "$svg"; then
        record "mermaid" "$source" "PASS" "$svg" "rendered intermediate SVG with mmdc"
      else
        record "mermaid" "$source" "FAIL" "$png" "mmdc SVG render failed before PNG conversion"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
    png_from_svg "$svg" "$png" "$source" "mermaid"
  fi
}

extract_mermaid_from_markdown() {
  local source="$1"
  local base="$2"
  local count=0
  local in_block=0
  local current=""

  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$in_block" -eq 0 && "$line" == '```mermaid'* ]]; then
      count=$((count + 1))
      current="${tmp_dir}/${base}-${count}.mmd"
      : > "$current"
      in_block=1
      continue
    fi
    if [[ "$in_block" -eq 1 && "$line" == '```' ]]; then
      render_mermaid_file "$current" "${base}-${count}"
      in_block=0
      current=""
      continue
    fi
    if [[ "$in_block" -eq 1 ]]; then
      printf '%s\n' "$line" >> "$current"
    fi
  done < "$source"

  if [[ "$count" -eq 0 ]]; then
    record "mermaid" "$source" "SKIP" "-" "no mermaid fence found"
  fi
}

render_d2_file() {
  local source="$1"
  local stem="$2"
  local svg="${out_dir}/${stem}.svg"
  local png="${out_dir}/${stem}.png"

  if ! have d2; then
    missing_tool "d2" "$source" "d2"
    return
  fi

  if want_format svg; then
    if [[ "$dry_run" == true ]]; then
      record "d2" "$source" "PLAN" "$svg" "would render SVG with d2"
    else
      if d2 "$source" "$svg"; then
        record "d2" "$source" "PASS" "$svg" "rendered SVG with d2"
      else
        record "d2" "$source" "FAIL" "$svg" "d2 SVG render failed"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
  fi

  if want_format png; then
    if [[ ! -f "$svg" && "$dry_run" != true ]]; then
      if d2 "$source" "$svg"; then
        record "d2" "$source" "PASS" "$svg" "rendered intermediate SVG with d2"
      else
        record "d2" "$source" "FAIL" "$png" "d2 SVG render failed before PNG conversion"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
    png_from_svg "$svg" "$png" "$source" "d2"
  fi
}

render_plantuml_file() {
  local source="$1"
  local stem="$2"
  local svg="${out_dir}/${stem}.svg"
  local png="${out_dir}/${stem}.png"

  if ! have plantuml; then
    missing_tool "plantuml" "$source" "plantuml"
    return
  fi

  if want_format svg; then
    if [[ "$dry_run" == true ]]; then
      record "plantuml" "$source" "PLAN" "$svg" "would render SVG with plantuml"
    else
      if plantuml -tsvg -pipe < "$source" > "$svg"; then
        record "plantuml" "$source" "PASS" "$svg" "rendered SVG with plantuml"
      else
        record "plantuml" "$source" "FAIL" "$svg" "plantuml SVG render failed"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
  fi

  if want_format png; then
    if [[ ! -f "$svg" && "$dry_run" != true ]]; then
      if plantuml -tsvg -pipe < "$source" > "$svg"; then
        record "plantuml" "$source" "PASS" "$svg" "rendered intermediate SVG with plantuml"
      else
        record "plantuml" "$source" "FAIL" "$png" "plantuml SVG render failed before PNG conversion"
        [[ "$required" == true ]] && exit 1
        return
      fi
    fi
    png_from_svg "$svg" "$png" "$source" "plantuml"
  fi
}

render_structurizr_file() {
  local source="$1"
  local stem="$2"
  local export_dir="${out_dir}/${stem}-structurizr-export"

  if ! have structurizr; then
    missing_tool "structurizr" "$source" "structurizr"
    return
  fi

  if [[ "$dry_run" == true ]]; then
    record "structurizr" "$source" "PLAN" "$export_dir" "would validate and export Mermaid sources"
    return
  fi

  structurizr validate -workspace "$source"
  mkdir -p "$export_dir"
  structurizr export -workspace "$source" -format mermaid -output "$export_dir"
  record "structurizr" "$source" "PASS" "$export_dir" "validated DSL and exported Mermaid sources"
}

shopt -s nullglob
for source in "$input_dir"/*; do
  [[ -f "$source" ]] || continue
  filename="$(basename "$source")"
  stem="${filename%.*}"
  case "$source" in
    *.mmd|*.mermaid)
      if backend_skipped mermaid; then
        record "mermaid" "$source" "SKIP" "-" "backend skipped by operator"
      else
        render_mermaid_file "$source" "$stem"
      fi
      ;;
    *.md)
      if backend_skipped mermaid; then
        record "mermaid" "$source" "SKIP" "-" "backend skipped by operator"
      else
        extract_mermaid_from_markdown "$source" "$stem"
      fi
      ;;
    *.d2)
      if backend_skipped d2; then
        record "d2" "$source" "SKIP" "-" "backend skipped by operator"
      else
        render_d2_file "$source" "$stem"
      fi
      ;;
    *.puml|*.plantuml)
      if backend_skipped plantuml; then
        record "plantuml" "$source" "SKIP" "-" "backend skipped by operator"
      else
        render_plantuml_file "$source" "$stem"
      fi
      ;;
    *.dsl)
      if backend_skipped structurizr; then
        record "structurizr" "$source" "SKIP" "-" "backend skipped by operator"
      else
        render_structurizr_file "$source" "$stem"
      fi
      ;;
    *)
      record "unknown" "$source" "SKIP" "-" "unsupported extension"
      ;;
  esac
done

if want_format png && want_format svg; then
  for svg in "$out_dir"/*.svg; do
    [[ -f "$svg" ]] || continue
    png="${svg%.svg}.png"
    [[ -f "$png" ]] && continue
    png_from_svg "$svg" "$png" "$svg" "svg-rasterize"
  done
fi

rm -rf "$tmp_dir"
echo "render_diagrams: wrote ${manifest}"
