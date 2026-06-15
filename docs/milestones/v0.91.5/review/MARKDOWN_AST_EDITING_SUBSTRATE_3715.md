# Markdown AST Editing Substrate Proof - #3715

Status: implementation proof for review

## Summary

Issue `#3715` adds a bounded Markdown AST editing substrate for ADL docs work:

```bash
adl tooling markdown-ast-edit replace-section \
  --input <path> \
  --heading <heading> \
  --replacement <path> \
  --out <path> \
  [--repair-note-out <path>]
```

The substrate uses `markdown-rs` (`markdown = "1.0.0"`) for AST inspection and
guards edits before and after mutation. It intentionally does not introduce a
full Markdown pretty-printer. The mutation is line-preserving and limited to
heading-anchored section body replacement.

## Supported Node Policy

The initial supported preservation surface is:

- headings
- fenced code blocks
- tables
- links
- YAML front matter

The command parses the input and output with `markdown-rs`, verifies that the
target heading exists in the AST, and verifies that original headings and front
matter remain present after the edit. It also rejects edits that remove existing
fenced code blocks, tables, or links.

## Unsupported Node Policy

Raw HTML is treated as unsupported for this initial substrate and fails closed.
When `--repair-note-out` is supplied, the command writes a human-readable repair
note with:

- input path
- target heading
- `Status: not_mutated`
- failure reason

## Lifecycle Card Boundary

Lifecycle cards remain governed by prompt-template rendering and card-editor
skills. The command rejects `.adl/.../tasks/issue-.../{sip,stp,spp,srp,sor}.md`
input and output paths and writes a repair note when requested. This prevents
the AST editor from becoming an ungoverned parallel card editor.

## Validation

Focused tests were added in:

- `adl/src/cli/tooling_cmd/tests/markdown_ast_edit.rs`

The fixture coverage proves:

- section replacement preserves front matter, headings, tables, links, and code
  fences
- lifecycle card mutation fails closed and writes a repair note
- lifecycle-card output targeting fails closed and writes a repair note
- unsupported raw HTML fails closed and writes a repair note
- replacements that remove existing protected nodes fail closed

## Non-Claims

- This is not a general Markdown formatter.
- This does not replace prompt-template rendering.
- This does not authorize direct lifecycle-card mutation.
- This does not claim all Markdown node types are safely editable.
