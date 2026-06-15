use anyhow::{anyhow, bail, ensure, Context, Result};
use markdown::{mdast::Node, ParseOptions};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::absolutize;
use super::markdown::split_front_matter;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkdownShape {
    headings: Vec<(u8, String)>,
    code_fences: usize,
    tables: usize,
    links: Vec<String>,
    has_front_matter: bool,
}

#[derive(Debug, Clone)]
struct ReplaceSectionArgs {
    input: PathBuf,
    heading: String,
    replacement: PathBuf,
    out: PathBuf,
    repair_note_out: Option<PathBuf>,
}

pub(super) fn real_markdown_ast_edit(args: &[String]) -> Result<()> {
    let Some(command) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "markdown-ast-edit requires a command: replace-section"
        ));
    };
    match command {
        "replace-section" => replace_section(parse_replace_section_args(&args[1..])?),
        "--help" | "-h" | "help" => {
            println!("{}", markdown_ast_edit_usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown markdown-ast-edit command '{other}' (expected replace-section)"
        )),
    }
}

fn markdown_ast_edit_usage() -> &'static str {
    "adl tooling markdown-ast-edit replace-section --input <path> --heading <heading> --replacement <path> --out <path> [--repair-note-out <path>]"
}

fn parse_replace_section_args(args: &[String]) -> Result<ReplaceSectionArgs> {
    let mut input = None;
    let mut heading = None;
    let mut replacement = None;
    let mut out = None;
    let mut repair_note_out = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--input" => input = Some(PathBuf::from(next_arg(&mut iter, "--input")?)),
            "--heading" => heading = Some(next_arg(&mut iter, "--heading")?),
            "--replacement" => {
                replacement = Some(PathBuf::from(next_arg(&mut iter, "--replacement")?))
            }
            "--out" => out = Some(PathBuf::from(next_arg(&mut iter, "--out")?)),
            "--repair-note-out" => {
                repair_note_out = Some(PathBuf::from(next_arg(&mut iter, "--repair-note-out")?))
            }
            "--help" | "-h" => {
                bail!("{}", markdown_ast_edit_usage());
            }
            other => bail!("unexpected markdown-ast-edit argument: {other}"),
        }
    }

    Ok(ReplaceSectionArgs {
        input: input.ok_or_else(|| anyhow!("missing --input"))?,
        heading: heading.ok_or_else(|| anyhow!("missing --heading"))?,
        replacement: replacement.ok_or_else(|| anyhow!("missing --replacement"))?,
        out: out.ok_or_else(|| anyhow!("missing --out"))?,
        repair_note_out,
    })
}

fn next_arg<'a>(iter: &mut impl Iterator<Item = &'a String>, flag: &str) -> Result<String> {
    iter.next()
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn replace_section(args: ReplaceSectionArgs) -> Result<()> {
    let input_abs = absolutize(&args.input)?;
    let input_text = fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let replacement_text = fs::read_to_string(&args.replacement)
        .with_context(|| format!("failed to read {}", args.replacement.display()))?;

    let out_abs = absolutize(&args.out)?;
    if is_lifecycle_card_path(&input_abs) || is_lifecycle_card_path(&out_abs) {
        return fail_closed(
            &args,
            &input_abs,
            "lifecycle-card input or output path requires prompt-template/card-editor authority; markdown-ast-edit did not mutate it",
        );
    }

    let before_shape = match inspect_markdown_shape("input", &input_text) {
        Ok(shape) => shape,
        Err(err) => return fail_closed(&args, &input_abs, &err.to_string()),
    };
    if !before_shape
        .headings
        .iter()
        .any(|(_, text)| text == args.heading.trim())
    {
        return fail_closed(
            &args,
            &input_abs,
            &format!("heading '{}' not found in input AST", args.heading),
        );
    }

    let replacement_body = strip_matching_heading(&replacement_text, &args.heading)?;
    inspect_markdown_shape("replacement", &replacement_body)
        .map_err(|err| anyhow!("replacement failed AST guard: {err}"))?;
    let edited = replace_markdown_section_body(&input_text, &args.heading, &replacement_body)
        .map_err(|err| anyhow!("section replacement failed: {err}"))?;
    let after_shape = match inspect_markdown_shape("edited", &edited) {
        Ok(shape) => shape,
        Err(err) => return fail_closed(&args, &input_abs, &err.to_string()),
    };
    ensure!(
        before_shape.has_front_matter == after_shape.has_front_matter,
        "front matter preservation guard failed"
    );
    ensure!(
        after_shape.code_fences >= before_shape.code_fences,
        "code fence preservation guard failed"
    );
    ensure!(
        after_shape.tables >= before_shape.tables,
        "table preservation guard failed"
    );
    ensure!(
        before_shape
            .links
            .iter()
            .collect::<BTreeSet<_>>()
            .is_subset(&after_shape.links.iter().collect::<BTreeSet<_>>()),
        "link preservation guard failed"
    );
    ensure!(
        before_shape
            .headings
            .iter()
            .map(|(_, text)| text)
            .collect::<BTreeSet<_>>()
            .is_subset(
                &after_shape
                    .headings
                    .iter()
                    .map(|(_, text)| text)
                    .collect::<BTreeSet<_>>()
            ),
        "heading preservation guard failed"
    );

    ensure_parent_dir(&args.out)?;
    fs::write(&args.out, edited).with_context(|| format!("failed to write {}", args.out.display()))
}

fn fail_closed(args: &ReplaceSectionArgs, input: &Path, reason: &str) -> Result<()> {
    if let Some(note_path) = &args.repair_note_out {
        ensure_parent_dir(note_path)?;
        let note = format!(
            "# Markdown AST edit repair note\n\n- Input: {}\n- Heading: {}\n- Status: not_mutated\n- Reason: {}\n\nThe editor failed closed. Use the prompt-template/card editor path for lifecycle cards or repair this document by hand before retrying.\n",
            display_path(input),
            args.heading,
            reason
        );
        fs::write(note_path, note)
            .with_context(|| format!("failed to write repair note {}", note_path.display()))?;
    }
    bail!("markdown AST edit failed closed: {reason}")
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    Ok(())
}

fn display_path(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized
        .find("agent-design-language/")
        .map(|idx| normalized[idx + "agent-design-language/".len()..].to_string())
        .unwrap_or(normalized)
}

fn is_lifecycle_card_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.contains("/.adl/")
        && normalized.contains("/tasks/issue-")
        && matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some("sip.md" | "stp.md" | "spp.md" | "srp.md" | "sor.md")
        )
}

fn strip_matching_heading(text: &str, heading: &str) -> Result<String> {
    let lines = text.lines().collect::<Vec<_>>();
    let Some(first_nonblank) = lines.iter().position(|line| !line.trim().is_empty()) else {
        return Ok(String::new());
    };
    if let Some((_, text_heading)) = parse_heading_line(lines[first_nonblank]) {
        if text_heading == heading.trim() {
            return Ok(lines[first_nonblank + 1..].join("\n").trim().to_string());
        }
    }
    Ok(text.trim().to_string())
}

fn replace_markdown_section_body(input: &str, heading: &str, replacement: &str) -> Result<String> {
    let lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let mut in_fence = false;
    let mut start = None;
    let mut target_depth = None;
    let mut end = lines.len();

    for (idx, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some((depth, text)) = parse_heading_line(line) {
            if start.is_none() && text == heading.trim() {
                start = Some(idx);
                target_depth = Some(depth);
                continue;
            }
            if start.is_some() && depth <= target_depth.expect("target depth") {
                end = idx;
                break;
            }
        }
    }

    let start = start.ok_or_else(|| anyhow!("heading '{heading}' not found"))?;
    let mut out = Vec::new();
    out.extend_from_slice(&lines[..=start]);
    out.push(String::new());
    if !replacement.trim().is_empty() {
        out.extend(replacement.trim().lines().map(str::to_string));
        out.push(String::new());
    }
    out.extend_from_slice(&lines[end..]);
    Ok(format!("{}\n", trim_trailing_blank_lines(out).join("\n")))
}

fn parse_heading_line(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim_start();
    let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = trimmed.get(hashes..)?;
    if !rest.starts_with(' ') {
        return None;
    }
    Some((hashes, rest.trim().to_string()))
}

fn trim_trailing_blank_lines(mut lines: Vec<String>) -> Vec<String> {
    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    lines
}

fn inspect_markdown_shape(label: &str, text: &str) -> Result<MarkdownShape> {
    let (has_front_matter, body) = match split_front_matter(text) {
        Ok((_front_matter, body)) => (true, body),
        Err(_) => (false, text.to_string()),
    };
    let ast = markdown::to_mdast(&body, &ParseOptions::default())
        .map_err(|err| anyhow!("{label} markdown-rs AST parse failed: {err}"))?;
    let mut shape = MarkdownShape {
        headings: Vec::new(),
        code_fences: 0,
        tables: 0,
        links: Vec::new(),
        has_front_matter,
    };
    collect_shape(label, &ast, &mut shape)?;
    Ok(shape)
}

fn collect_shape(label: &str, node: &Node, shape: &mut MarkdownShape) -> Result<()> {
    match node {
        Node::Root(root) => {
            for child in &root.children {
                collect_shape(label, child, shape)?;
            }
        }
        Node::Heading(heading) => {
            shape
                .headings
                .push((heading.depth, children_plain_text(&heading.children)));
        }
        Node::Code(_) => shape.code_fences += 1,
        Node::Table(_) => shape.tables += 1,
        Node::Link(link) => shape.links.push(link.url.clone()),
        Node::Html(_) => bail!("{label} contains unsupported raw HTML node"),
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    collect_shape(label, child, shape)?;
                }
            }
        }
    }
    Ok(())
}

fn children_plain_text(children: &[Node]) -> String {
    children.iter().map(node_plain_text).collect::<String>()
}

fn node_plain_text(node: &Node) -> String {
    match node {
        Node::Text(text) => text.value.clone(),
        Node::InlineCode(code) => code.value.clone(),
        Node::Emphasis(emphasis) => children_plain_text(&emphasis.children),
        Node::Strong(strong) => children_plain_text(&strong.children),
        Node::Delete(delete) => children_plain_text(&delete.children),
        Node::Link(link) => children_plain_text(&link.children),
        _ => node
            .children()
            .map(|children| children_plain_text(children))
            .unwrap_or_default(),
    }
}
