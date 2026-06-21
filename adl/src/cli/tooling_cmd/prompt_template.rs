use adl::csdlc_prompt_editor::{
    edit_values_file, import_values_from_rendered_card_file, render_all_cards_from_values_dir,
    render_card_from_values_file, repo_root_from_arg,
    validate_rendered_card_structure_file_for_template_set,
    validate_structure_schema_files_for_template_set, validate_values_file,
    write_all_sample_values_for_template_set, write_all_structure_schemas_for_template_set,
    PromptCardKind,
};
use anyhow::{bail, ensure, Result};
use std::fs;
use std::path::PathBuf;

use super::tooling_usage;

pub(crate) fn real_prompt_template(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        bail!("prompt-template requires a subcommand: render | render-all | edit-values | edit-rendered | import-values | validate-values | validate-structure | validate-schemas | write-sample-values | write-structure-schemas");
    };

    match subcommand {
        "render" => render_one(&args[1..]),
        "render-all" => render_all(&args[1..]),
        "edit-values" => edit_values(&args[1..]),
        "edit-rendered" => edit_rendered(&args[1..]),
        "import-values" => import_values(&args[1..]),
        "validate-values" => validate_one(&args[1..]),
        "validate-structure" => validate_structure_one(&args[1..]),
        "validate-schemas" => validate_schemas(&args[1..]),
        "write-sample-values" => write_samples(&args[1..]),
        "write-structure-schemas" => write_structure_schemas(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", tooling_usage());
            Ok(())
        }
        other => bail!(
            "unknown prompt-template subcommand '{other}' (expected render | render-all | edit-values | edit-rendered | import-values | validate-values | validate-structure | validate-schemas | write-sample-values | write-structure-schemas)"
        ),
    }
}

fn render_one(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut values: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;

    parse_common_args(args, &mut repo_root, &mut kind, &mut values, Some(&mut out))?;
    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("render requires --kind"))?;
    let values = values.ok_or_else(|| anyhow::anyhow!("render requires --values"))?;
    let out = out.ok_or_else(|| anyhow::anyhow!("render requires --out"))?;
    let rendered = render_card_from_values_file(&root, kind, &values)?;
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, rendered)?;
    println!("PASS: rendered {} card to {}", kind.key(), out.display());
    Ok(())
}

fn validate_one(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut values: Option<PathBuf> = None;

    parse_common_args(args, &mut repo_root, &mut kind, &mut values, None)?;
    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("validate-values requires --kind"))?;
    let values = values.ok_or_else(|| anyhow::anyhow!("validate-values requires --values"))?;
    validate_values_file(&root, kind, &values)?;
    println!("PASS: values valid for {}", kind.key());
    Ok(())
}

fn edit_values(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut values: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut updates: Vec<(String, String)> = Vec::new();

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--kind" => {
                idx += 1;
                kind = Some(PromptCardKind::parse_key(value_arg(args, idx, "--kind")?)?);
            }
            "--values" => {
                idx += 1;
                values = Some(PathBuf::from(value_arg(args, idx, "--values")?));
            }
            "--set" => {
                idx += 1;
                updates.push(parse_set_arg(value_arg(args, idx, "--set")?)?);
            }
            "--out" => {
                idx += 1;
                out = Some(PathBuf::from(value_arg(args, idx, "--out")?));
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template edit-values: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("edit-values requires --kind"))?;
    let values = values.ok_or_else(|| anyhow::anyhow!("edit-values requires --values"))?;
    let target = edit_values_file(&root, kind, &values, &updates, out.as_deref())?;
    println!("PASS: edited {} values at {}", kind.key(), target.display());
    Ok(())
}

fn edit_rendered(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut input: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut values_out: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;
    let mut updates: Vec<(String, String)> = Vec::new();

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--kind" => {
                idx += 1;
                kind = Some(PromptCardKind::parse_key(value_arg(args, idx, "--kind")?)?);
            }
            "--input" => {
                idx += 1;
                input = Some(PathBuf::from(value_arg(args, idx, "--input")?));
            }
            "--out" => {
                idx += 1;
                out = Some(PathBuf::from(value_arg(args, idx, "--out")?));
            }
            "--values-out" => {
                idx += 1;
                values_out = Some(PathBuf::from(value_arg(args, idx, "--values-out")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--set" => {
                idx += 1;
                updates.push(parse_set_arg(value_arg(args, idx, "--set")?)?);
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template edit-rendered: {other}"),
        }
        idx += 1;
    }

    ensure!(
        !updates.is_empty(),
        "edit-rendered requires at least one --set field=value update"
    );
    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("edit-rendered requires --kind"))?;
    let input = input.ok_or_else(|| anyhow::anyhow!("edit-rendered requires --input"))?;
    let out = out.ok_or_else(|| anyhow::anyhow!("edit-rendered requires --out"))?;
    let values_target = values_out.unwrap_or_else(|| out.with_extension("values.yaml"));

    let report = import_values_from_rendered_card_file(
        &root,
        kind,
        &input,
        &values_target,
        None,
        template_set.as_deref(),
    )?;
    edit_values_file(&root, kind, &values_target, &updates, None)?;
    let rendered = render_card_from_values_file(&root, kind, &values_target)?;
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out, rendered)?;
    validate_rendered_card_structure_file_for_template_set(
        &root,
        kind,
        &out,
        template_set.as_deref(),
    )?;
    println!(
        "PASS: edited rendered {} card to {} via values {} (round_trip={})",
        kind.key(),
        out.display(),
        values_target.display(),
        report.comparison.as_str()
    );
    Ok(())
}

fn import_values(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut input: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut normalized_out: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--kind" => {
                idx += 1;
                kind = Some(PromptCardKind::parse_key(value_arg(args, idx, "--kind")?)?);
            }
            "--input" => {
                idx += 1;
                input = Some(PathBuf::from(value_arg(args, idx, "--input")?));
            }
            "--out" => {
                idx += 1;
                out = Some(PathBuf::from(value_arg(args, idx, "--out")?));
            }
            "--normalized-out" => {
                idx += 1;
                normalized_out = Some(PathBuf::from(value_arg(args, idx, "--normalized-out")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template import-values: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("import-values requires --kind"))?;
    let input = input.ok_or_else(|| anyhow::anyhow!("import-values requires --input"))?;
    let out = out.ok_or_else(|| anyhow::anyhow!("import-values requires --out"))?;
    let report = import_values_from_rendered_card_file(
        &root,
        kind,
        &input,
        &out,
        normalized_out.as_deref(),
        template_set.as_deref(),
    )?;
    println!(
        "PASS: imported {} card values to {} (round_trip={})",
        kind.key(),
        report.values_path.display(),
        report.comparison.as_str()
    );
    if !report.unrepresented_required_fields.is_empty() {
        println!(
            "NOTE: populated unrepresented required fields: {}",
            report.unrepresented_required_fields.join(", ")
        );
    }
    if let Some(normalized_path) = report.normalized_path {
        println!(
            "PASS: wrote normalized rendered card to {}",
            normalized_path.display()
        );
    }
    Ok(())
}

fn validate_structure_one(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut kind: Option<PromptCardKind> = None;
    let mut input: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--kind" => {
                idx += 1;
                kind = Some(PromptCardKind::parse_key(value_arg(args, idx, "--kind")?)?);
            }
            "--input" => {
                idx += 1;
                input = Some(PathBuf::from(value_arg(args, idx, "--input")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template validate-structure: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    let kind = kind.ok_or_else(|| anyhow::anyhow!("validate-structure requires --kind"))?;
    let input = input.ok_or_else(|| anyhow::anyhow!("validate-structure requires --input"))?;
    validate_rendered_card_structure_file_for_template_set(
        &root,
        kind,
        &input,
        template_set.as_deref(),
    )?;
    println!("PASS: rendered structure valid for {}", kind.key());
    Ok(())
}

fn render_all(args: &[String]) -> Result<()> {
    let mut repo_root: Option<PathBuf> = None;
    let mut values_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--values-dir" => {
                idx += 1;
                values_dir = Some(PathBuf::from(value_arg(args, idx, "--values-dir")?));
            }
            "--out-dir" => {
                idx += 1;
                out_dir = Some(PathBuf::from(value_arg(args, idx, "--out-dir")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template render-all: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    let values_dir =
        values_dir.ok_or_else(|| anyhow::anyhow!("render-all requires --values-dir"))?;
    let out_dir = out_dir.ok_or_else(|| anyhow::anyhow!("render-all requires --out-dir"))?;
    render_all_cards_from_values_dir(&root, &values_dir, &out_dir, template_set.as_deref())?;
    println!("PASS: rendered all prompt cards to {}", out_dir.display());
    Ok(())
}

fn validate_schemas(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }
    let mut repo_root: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template validate-schemas: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    validate_structure_schema_files_for_template_set(&root, template_set.as_deref())?;
    println!("PASS: prompt-card structure schemas match active templates");
    Ok(())
}

fn write_samples(args: &[String]) -> Result<()> {
    let mut repo_root: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--out-dir" => {
                idx += 1;
                out_dir = Some(PathBuf::from(value_arg(args, idx, "--out-dir")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template write-sample-values: {other}"),
        }
        idx += 1;
    }

    let root = repo_root_from_arg(repo_root)?;
    let out_dir =
        out_dir.ok_or_else(|| anyhow::anyhow!("write-sample-values requires --out-dir"))?;
    write_all_sample_values_for_template_set(&root, &out_dir, template_set.as_deref())?;
    println!("PASS: wrote sample prompt values to {}", out_dir.display());
    Ok(())
}

fn write_structure_schemas(args: &[String]) -> Result<()> {
    let mut repo_root: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut template_set: Option<String> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--out-dir" => {
                idx += 1;
                out_dir = Some(PathBuf::from(value_arg(args, idx, "--out-dir")?));
            }
            "--template-set" => {
                idx += 1;
                template_set = Some(value_arg(args, idx, "--template-set")?.to_string());
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => {
                bail!("unknown arg for tooling prompt-template write-structure-schemas: {other}")
            }
        }
        idx += 1;
    }

    let out_dir =
        out_dir.ok_or_else(|| anyhow::anyhow!("write-structure-schemas requires --out-dir"))?;
    let root = repo_root_from_arg(repo_root)?;
    write_all_structure_schemas_for_template_set(&root, &out_dir, template_set.as_deref())?;
    println!(
        "PASS: wrote prompt-card structure schemas to {}",
        out_dir.display()
    );
    Ok(())
}

fn parse_common_args(
    args: &[String],
    repo_root: &mut Option<PathBuf>,
    kind: &mut Option<PromptCardKind>,
    values: &mut Option<PathBuf>,
    mut out: Option<&mut Option<PathBuf>>,
) -> Result<()> {
    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                *repo_root = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            "--kind" => {
                idx += 1;
                *kind = Some(PromptCardKind::parse_key(value_arg(args, idx, "--kind")?)?);
            }
            "--values" => {
                idx += 1;
                *values = Some(PathBuf::from(value_arg(args, idx, "--values")?));
            }
            "--out" => {
                let Some(out) = out.as_deref_mut() else {
                    bail!("--out is not supported for this prompt-template subcommand");
                };
                idx += 1;
                *out = Some(PathBuf::from(value_arg(args, idx, "--out")?));
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling prompt-template: {other}"),
        }
        idx += 1;
    }
    Ok(())
}

fn value_arg<'a>(args: &'a [String], idx: usize, flag: &str) -> Result<&'a str> {
    args.get(idx)
        .map(String::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing value for {flag}"))
}

fn has_help_arg(args: &[String]) -> bool {
    args.iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "help"))
}

fn parse_set_arg(value: &str) -> Result<(String, String)> {
    let Some((key, value)) = value.split_once('=') else {
        bail!("--set must use field=value syntax");
    };
    let key = key.trim();
    ensure!(!key.is_empty(), "--set field name must not be empty");
    Ok((key.to_string(), value.to_string()))
}
