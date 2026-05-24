use std::path::PathBuf;

use adl::csdlc_prompt_editor::{
    render_all_sample_cards, repo_root_from_arg, write_editor_model_js,
};
use anyhow::{bail, Result};

use super::tooling_usage;

pub(crate) fn real_csdlc_prompt_editor(args: &[String]) -> Result<()> {
    let mut repo_root: Option<PathBuf> = None;
    let mut emit_model_js: Option<PathBuf> = None;
    let mut render_samples: Option<PathBuf> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--repo-root" => {
                idx += 1;
                let Some(value) = args.get(idx) else {
                    bail!("missing value for --repo-root");
                };
                repo_root = Some(PathBuf::from(value));
            }
            "--emit-model-js" => {
                idx += 1;
                let Some(value) = args.get(idx) else {
                    bail!("missing value for --emit-model-js");
                };
                emit_model_js = Some(PathBuf::from(value));
            }
            "--render-samples" => {
                idx += 1;
                let Some(value) = args.get(idx) else {
                    bail!("missing value for --render-samples");
                };
                render_samples = Some(PathBuf::from(value));
            }
            "--help" | "-h" | "help" => {
                println!("{}", tooling_usage());
                return Ok(());
            }
            other => bail!("unknown arg for tooling csdlc-prompt-editor: {other}"),
        }
        idx += 1;
    }

    if emit_model_js.is_none() && render_samples.is_none() {
        bail!("csdlc-prompt-editor requires --emit-model-js and/or --render-samples");
    }

    let root = repo_root_from_arg(repo_root)?;
    if let Some(path) = emit_model_js {
        write_editor_model_js(&root, &path)?;
    }
    if let Some(path) = render_samples {
        render_all_sample_cards(&root, &path)?;
    }

    Ok(())
}
