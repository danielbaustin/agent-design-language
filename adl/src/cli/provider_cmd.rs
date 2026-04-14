use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn real_provider(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
    real_provider_in_repo(args, &repo_root)
}

fn real_provider_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!("provider requires a subcommand: setup"));
    };

    match subcommand {
        "setup" => real_provider_setup(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown provider subcommand '{other}' (expected setup)"
        )),
    }
}

fn real_provider_setup(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut family: Option<String> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut force = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--force" => force = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            value if family.is_none() => family = Some(value.to_string()),
            other => return Err(anyhow!("unknown arg for provider setup: {other}")),
        }
        i += 1;
    }

    let family = family.ok_or_else(|| anyhow!("provider setup requires <family>"))?;
    let template = template_for_family(&family)?;
    let out_dir = out_dir.unwrap_or_else(|| {
        repo_root
            .join(".adl")
            .join("provider-setup")
            .join(template.family)
    });

    if out_dir.exists() && !force {
        return Err(anyhow!(
            "provider setup output already exists at {} (pass --force to overwrite)",
            out_dir.display()
        ));
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create setup directory {}", out_dir.display()))?;

    let provider_path = out_dir.join("provider.adl.yaml");
    let env_path = out_dir.join(".env.example");
    let readme_path = out_dir.join("README.md");

    fs::write(&provider_path, render_provider_yaml(template).as_bytes())
        .with_context(|| format!("failed to write {}", provider_path.display()))?;
    fs::write(&env_path, render_env_example(template).as_bytes())
        .with_context(|| format!("failed to write {}", env_path.display()))?;
    fs::write(&readme_path, render_readme(template).as_bytes())
        .with_context(|| format!("failed to write {}", readme_path.display()))?;

    println!("PROVIDER_SETUP_FAMILY={}", template.family);
    println!("PROVIDER_SETUP_DIR={}", out_dir.display());
    println!("PROVIDER_SNIPPET_PATH={}", provider_path.display());
    println!("PROVIDER_ENV_EXAMPLE={}", env_path.display());
    println!("PROVIDER_SETUP_README={}", readme_path.display());
    Ok(())
}

struct ProviderSetupTemplate {
    family: &'static str,
    profile: Option<&'static str>,
    kind: Option<&'static str>,
    env_var: &'static str,
    provider_id: &'static str,
    agent_id: &'static str,
    model_ref: &'static str,
    provider_model_id: &'static str,
    endpoint_hint: Option<&'static str>,
    notes: &'static str,
}

fn template_for_family(family: &str) -> Result<&'static ProviderSetupTemplate> {
    let normalized = family.trim().to_lowercase();
    let template = match normalized.as_str() {
        "chatgpt" => &ProviderSetupTemplate {
            family: "chatgpt",
            profile: Some("chatgpt:gpt-5.4"),
            kind: None,
            env_var: "OPENAI_API_KEY",
            provider_id: "chatgpt_primary",
            agent_id: "chatgpt_agent",
            model_ref: "gpt-5.4",
            provider_model_id: "gpt-5.4",
            endpoint_hint: Some("https://api.example.invalid/v1/complete"),
            notes: "Use this when you want the ChatGPT/GPT-5 family surface. Keep the endpoint pointed at an ADL-compatible completion endpoint and supply your own OpenAI key through the generated env file.",
        },
        "claude" => &ProviderSetupTemplate {
            family: "claude",
            profile: Some("claude:claude-3-7-sonnet"),
            kind: None,
            env_var: "ANTHROPIC_API_KEY",
            provider_id: "claude_primary",
            agent_id: "claude_agent",
            model_ref: "claude-3-7-sonnet",
            provider_model_id: "claude-3-7-sonnet-latest",
            endpoint_hint: Some("https://api.example.invalid/v1/complete"),
            notes: "Use this when you want the first-class Claude family surface. Keep the endpoint pointed at an ADL-compatible completion endpoint and supply your Anthropic credential through the generated env file.",
        },
        "openai" => &ProviderSetupTemplate {
            family: "openai",
            profile: None,
            kind: Some("openai"),
            env_var: "OPENAI_API_KEY",
            provider_id: "openai_primary",
            agent_id: "openai_agent",
            model_ref: "reasoning/default",
            provider_model_id: "gpt-4.1-mini",
            endpoint_hint: None,
            notes: "Use this for the Rust-native OpenAI provider path. The default endpoint is OpenAI's Responses API; override config.endpoint only for tests or a trusted compatible endpoint.",
        },
        "anthropic" => &ProviderSetupTemplate {
            family: "anthropic",
            profile: None,
            kind: Some("anthropic"),
            env_var: "ANTHROPIC_API_KEY",
            provider_id: "anthropic_primary",
            agent_id: "anthropic_agent",
            model_ref: "reasoning/default",
            provider_model_id: "claude-haiku-4-5-20251001",
            endpoint_hint: None,
            notes: "Use this for the Rust-native Anthropic provider path. The default endpoint is Anthropic's Messages API; override config.endpoint only for tests or a trusted compatible endpoint.",
        },
        "gemini" => &ProviderSetupTemplate {
            family: "gemini",
            profile: Some("http:gemini-2.0-flash"),
            kind: None,
            env_var: "GEMINI_API_KEY",
            provider_id: "gemini_primary",
            agent_id: "gemini_agent",
            model_ref: "reasoning/default",
            provider_model_id: "gemini-2.0-flash",
            endpoint_hint: Some("https://api.example.invalid/v1/complete"),
            notes: "Use this with an ADL-compatible HTTP endpoint that fronts Gemini-compatible models. The generated env file is local-only and should not be committed.",
        },
        "deepseek" => &ProviderSetupTemplate {
            family: "deepseek",
            profile: Some("http:deepseek-chat"),
            kind: None,
            env_var: "DEEPSEEK_API_KEY",
            provider_id: "deepseek_primary",
            agent_id: "deepseek_agent",
            model_ref: "reasoning/default",
            provider_model_id: "deepseek-chat",
            endpoint_hint: Some("https://api.example.invalid/v1/complete"),
            notes: "Use this with an ADL-compatible HTTP endpoint that fronts DeepSeek-compatible models.",
        },
        "http" | "generic-http" => &ProviderSetupTemplate {
            family: "http",
            profile: Some("http:gpt-4.1-mini"),
            kind: None,
            env_var: "ADL_REMOTE_BEARER_TOKEN",
            provider_id: "portable_http",
            agent_id: "http_agent",
            model_ref: "reasoning/default",
            provider_model_id: "gpt-4.1-mini",
            endpoint_hint: Some("https://api.example.invalid/v1/complete"),
            notes: "Use this as a provider-agnostic bounded HTTP setup. Replace the endpoint and token env var with the ones your remote gateway expects.",
        },
        other => {
            return Err(anyhow!(
                "unsupported provider setup family '{other}' (supported: chatgpt, claude, openai, anthropic, gemini, deepseek, http)"
            ))
        }
    };
    Ok(template)
}

fn render_provider_yaml(template: &ProviderSetupTemplate) -> String {
    let provider_identity = match (template.profile, template.kind) {
        (Some(profile), None) => format!("profile: \"{profile}\""),
        (None, Some(kind)) => format!("type: \"{kind}\""),
        _ => unreachable!("provider setup templates must declare exactly one identity surface"),
    };
    let endpoint_line = template
        .endpoint_hint
        .map(|endpoint| format!("      endpoint: \"{endpoint}\"\n"))
        .unwrap_or_default();
    format!(
        "version: \"0.5\"\n\nproviders:\n  {provider_id}:\n    {provider_identity}\n    config:\n{endpoint_line}      auth:\n        type: bearer\n        env: {env_var}\n      headers:\n        X-Client: \"adl-provider-setup\"\n      timeout_secs: 15\n      model_ref: \"{model_ref}\"\n      provider_model_id: \"{provider_model_id}\"\n\nagents:\n  {agent_id}:\n    provider: \"{provider_id}\"\n    model: \"{model_ref}\"\n\n# Merge this provider/agent snippet into your workflow file.\n",
        provider_id = template.provider_id,
        provider_identity = provider_identity,
        endpoint_line = endpoint_line,
        env_var = template.env_var,
        model_ref = template.model_ref,
        provider_model_id = template.provider_model_id,
        agent_id = template.agent_id,
    )
}

fn render_env_example(template: &ProviderSetupTemplate) -> String {
    format!(
        "# Copy to a local env file and fill in your real secret.\n# Do not commit the filled-in file.\n{env_var}=replace-me\n",
        env_var = template.env_var
    )
}

fn render_readme(template: &ProviderSetupTemplate) -> String {
    let transport_note = if template.kind.is_some() {
        "- This family uses ADL's Rust-native provider adapter for its vendor API.\n- Leave `config.endpoint` unset for the default vendor endpoint unless you are testing against a trusted compatible endpoint."
    } else {
        "- ADL's bounded HTTP provider expects a completion-style HTTP contract: request body with `{\"prompt\": ...}`, response body with `{\"output\": ...}`.\n- Raw vendor-native endpoints may require a compatibility gateway or adapter if they do not expose that contract directly."
    };
    let endpoint_step = if template.kind.is_some() {
        "2. Leave `config.endpoint` unset unless you are testing against a trusted compatible endpoint."
    } else {
        "2. Set `config.endpoint` in `provider.adl.yaml` to a real ADL-compatible completion endpoint."
    };
    format!(
        "# Provider setup: {family}\n\nThis bundle gives you a local starting point for configuring the `{family}` provider family.\n\nFiles:\n- `provider.adl.yaml`: mergeable ADL provider/agent snippet\n- `.env.example`: local env template for your credential\n\nSteps:\n1. Copy `.env.example` to a local untracked env file and put your real credential in `{env_var}`.\n{endpoint_step}\n3. Merge the provider/agent snippet into your workflow file.\n4. Source your local env file before running ADL.\n\nImportant:\n{transport_note}\n- No secrets are stored by this command; the generated env file is only a local template.\n\nNotes:\n{notes}\n",
        family = template.family,
        env_var = template.env_var,
        endpoint_step = endpoint_step,
        transport_note = transport_note,
        notes = template.notes
    )
}

fn required_value(args: &[String], index: usize, flag: &str) -> Result<String> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn run_git_capture(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("failed to invoke git {}", args.join(" ")))?;
    if !output.status.success() {
        return Err(anyhow!(
            "git {} failed with status {}",
            args.join(" "),
            output.status
        ));
    }
    String::from_utf8(output.stdout).context("git output was not valid UTF-8")
}

fn repo_root() -> Result<PathBuf> {
    Ok(PathBuf::from(
        run_git_capture(&["rev-parse", "--show-toplevel"])?
            .trim()
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_repo(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let repo = env::temp_dir().join(format!("adl-provider-{name}-{unique}"));
        fs::create_dir_all(&repo).expect("create repo dir");
        Command::new(if Path::new("/usr/bin/git").exists() {
            "/usr/bin/git"
        } else {
            "git"
        })
        .arg("init")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success()
        .then_some(())
        .expect("git init should succeed");
        repo
    }

    #[test]
    fn provider_requires_subcommand_and_rejects_unknown_subcommand() {
        let repo = temp_repo("subcommands");
        let err = real_provider_in_repo(&[], &repo).expect_err("missing subcommand should fail");
        assert!(err
            .to_string()
            .contains("provider requires a subcommand: setup"));

        let err =
            real_provider_in_repo(&["nope".to_string()], &repo).expect_err("unknown should fail");
        assert!(err
            .to_string()
            .contains("unknown provider subcommand 'nope'"));
    }

    #[test]
    fn provider_help_paths_succeed() {
        let repo = temp_repo("help");
        real_provider_in_repo(&["help".to_string()], &repo).expect("help should succeed");
        real_provider_in_repo(&["--help".to_string()], &repo).expect("--help should succeed");
        real_provider_in_repo(&["-h".to_string()], &repo).expect("-h should succeed");
    }

    #[test]
    fn provider_setup_writes_expected_bundle_for_chatgpt() {
        let repo = temp_repo("chatgpt");
        real_provider_in_repo(&["setup".to_string(), "chatgpt".to_string()], &repo)
            .expect("chatgpt setup should succeed");

        let out = repo.join(".adl/provider-setup/chatgpt");
        let provider_text =
            fs::read_to_string(out.join("provider.adl.yaml")).expect("provider yaml");
        let env_text = fs::read_to_string(out.join(".env.example")).expect("env example");
        let readme = fs::read_to_string(out.join("README.md")).expect("readme");

        assert!(provider_text.contains("profile: \"chatgpt:gpt-5.4\""));
        assert!(provider_text.contains("env: OPENAI_API_KEY"));
        assert!(env_text.contains("OPENAI_API_KEY=replace-me"));
        assert!(readme.contains("compatibility gateway or adapter"));
    }

    #[test]
    fn provider_setup_writes_expected_bundle_for_claude() {
        let repo = temp_repo("claude");
        real_provider_in_repo(&["setup".to_string(), "claude".to_string()], &repo)
            .expect("claude setup should succeed");

        let out = repo.join(".adl/provider-setup/claude");
        let provider_text =
            fs::read_to_string(out.join("provider.adl.yaml")).expect("provider yaml");
        let env_text = fs::read_to_string(out.join(".env.example")).expect("env example");
        let readme = fs::read_to_string(out.join("README.md")).expect("readme");

        assert!(provider_text.contains("profile: \"claude:claude-3-7-sonnet\""));
        assert!(provider_text.contains("provider: \"claude_primary\""));
        assert!(provider_text.contains("env: ANTHROPIC_API_KEY"));
        assert!(env_text.contains("ANTHROPIC_API_KEY=replace-me"));
        assert!(readme.contains("first-class Claude family surface"));
    }

    #[test]
    fn provider_setup_supports_explicit_out_and_force() {
        let repo = temp_repo("out-force");
        let out = repo.join("custom/provider-setup/openai");

        real_provider_in_repo(
            &[
                "setup".to_string(),
                "openai".to_string(),
                "--out".to_string(),
                out.display().to_string(),
            ],
            &repo,
        )
        .expect("first write should succeed");

        let err = real_provider_in_repo(
            &[
                "setup".to_string(),
                "openai".to_string(),
                "--out".to_string(),
                out.display().to_string(),
            ],
            &repo,
        )
        .expect_err("existing dir without force should fail");
        assert!(err.to_string().contains("pass --force to overwrite"));

        real_provider_in_repo(
            &[
                "setup".to_string(),
                "openai".to_string(),
                "--out".to_string(),
                out.display().to_string(),
                "--force".to_string(),
            ],
            &repo,
        )
        .expect("force overwrite should succeed");
    }

    #[test]
    fn provider_setup_validates_setup_args() {
        let repo = temp_repo("bad-args");

        real_provider_in_repo(&["setup".to_string(), "--help".to_string()], &repo)
            .expect("setup help should succeed");

        let err = real_provider_in_repo(&["setup".to_string()], &repo)
            .expect_err("missing family should fail");
        assert!(err.to_string().contains("provider setup requires <family>"));

        let err = real_provider_in_repo(
            &[
                "setup".to_string(),
                "chatgpt".to_string(),
                "--bogus".to_string(),
            ],
            &repo,
        )
        .expect_err("unknown arg should fail");
        assert!(err
            .to_string()
            .contains("unknown arg for provider setup: --bogus"));

        let err = real_provider_in_repo(
            &[
                "setup".to_string(),
                "chatgpt".to_string(),
                "--out".to_string(),
            ],
            &repo,
        )
        .expect_err("missing out value should fail");
        assert!(err.to_string().contains("--out requires a value"));
    }

    #[test]
    fn provider_setup_rejects_unknown_family() {
        let repo = temp_repo("unknown-family");
        let err = real_provider_in_repo(&["setup".to_string(), "bogus".to_string()], &repo)
            .expect_err("unknown family should fail");
        assert!(err
            .to_string()
            .contains("unsupported provider setup family"));
    }

    #[test]
    fn provider_setup_supports_all_declared_families() {
        let families = [
            ("chatgpt", "profile: \"chatgpt:gpt-5.4\"", "OPENAI_API_KEY"),
            (
                "claude",
                "profile: \"claude:claude-3-7-sonnet\"",
                "ANTHROPIC_API_KEY",
            ),
            ("openai", "type: \"openai\"", "OPENAI_API_KEY"),
            ("anthropic", "type: \"anthropic\"", "ANTHROPIC_API_KEY"),
            (
                "gemini",
                "profile: \"http:gemini-2.0-flash\"",
                "GEMINI_API_KEY",
            ),
            (
                "deepseek",
                "profile: \"http:deepseek-chat\"",
                "DEEPSEEK_API_KEY",
            ),
            (
                "generic-http",
                "profile: \"http:gpt-4.1-mini\"",
                "ADL_REMOTE_BEARER_TOKEN",
            ),
        ];

        for (family, provider_identity, env_var) in families {
            let template = template_for_family(family).expect("family should resolve");
            assert_eq!(template.env_var, env_var);

            let provider_yaml = render_provider_yaml(template);
            let env_example = render_env_example(template);
            let readme = render_readme(template);

            assert!(provider_yaml.contains(provider_identity));
            assert!(env_example.contains(env_var));
            assert!(readme.contains(template.family));
            if template.kind.is_some() {
                assert!(readme.contains("Rust-native provider adapter"));
            } else {
                assert!(readme.contains("completion-style HTTP contract"));
            }
        }
    }

    #[test]
    fn real_provider_uses_repo_root_for_default_output() {
        let _guard = env_lock();
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("adl crate lives under repo root")
            .to_path_buf();
        let prev_dir = env::current_dir().expect("cwd");
        env::set_current_dir(&repo_root).expect("switch to repo root");
        let out = repo_root.join(".adl/provider-setup/deepseek");
        if out.exists() {
            fs::remove_dir_all(&out).expect("remove stale output");
        }

        real_provider(&["setup".to_string(), "deepseek".to_string()])
            .expect("top-level provider entrypoint should succeed");

        assert!(out.join("provider.adl.yaml").exists());
        assert!(out.join(".env.example").exists());
        assert!(out.join("README.md").exists());

        fs::remove_dir_all(out).expect("cleanup generated output");
        env::set_current_dir(prev_dir).expect("restore cwd");
    }
}
