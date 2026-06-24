use crate::cli::pr_cmd::github_client::{AdlGithubClient, GithubClientBackend};
use crate::cli::tokio_runtime::with_current_thread_runtime;
use anyhow::{anyhow, bail, Context, Result};
use octocrab::models::repos::Release;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReleaseAction {
    EnsureAbsent,
    EnsurePresent,
    Draft,
    Publish,
}

#[derive(Debug, Clone)]
struct ReleaseArgs {
    action: ReleaseAction,
    repo: String,
    tag: String,
    name: Option<String>,
    notes_file: Option<PathBuf>,
    target: Option<String>,
}

pub(super) fn real_github_release(args: &[String]) -> Result<()> {
    let args = parse_args(args)?;
    let result = run_release_action(&args)?;
    if !result.is_empty() {
        println!("{result}");
    }
    Ok(())
}

fn parse_args(args: &[String]) -> Result<ReleaseArgs> {
    let Some(action) = args.first().map(String::as_str) else {
        bail!("{}", usage());
    };
    let action = match action {
        "ensure-absent" => ReleaseAction::EnsureAbsent,
        "ensure-present" => ReleaseAction::EnsurePresent,
        "draft" => ReleaseAction::Draft,
        "publish" => ReleaseAction::Publish,
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            return Err(anyhow!("help requested"));
        }
        other => bail!("unknown github-release action '{other}'\n{}", usage()),
    };

    let mut repo = None;
    let mut tag = None;
    let mut name = None;
    let mut notes_file = None;
    let mut target = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => {
                repo = Some(require_value(args, &mut i, "--repo")?);
            }
            "--tag" => {
                tag = Some(require_value(args, &mut i, "--tag")?);
            }
            "--name" => {
                name = Some(require_value(args, &mut i, "--name")?);
            }
            "--notes-file" => {
                notes_file = Some(PathBuf::from(require_value(args, &mut i, "--notes-file")?));
            }
            "--target" => {
                target = Some(require_value(args, &mut i, "--target")?);
            }
            other => bail!("unknown github-release argument '{other}'"),
        }
        i += 1;
    }

    let repo = repo.ok_or_else(|| anyhow!("github-release requires --repo <owner/repo>"))?;
    if !valid_repo(&repo) {
        bail!("github-release --repo must be owner/repo");
    }
    let tag = tag.ok_or_else(|| anyhow!("github-release requires --tag <tag>"))?;
    if tag.trim().is_empty() {
        bail!("github-release --tag cannot be empty");
    }

    Ok(ReleaseArgs {
        action,
        repo,
        tag,
        name,
        notes_file,
        target,
    })
}

fn require_value(args: &[String], i: &mut usize, flag: &str) -> Result<String> {
    *i += 1;
    args.get(*i)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| anyhow!("{flag} requires a non-empty value"))
}

fn usage() -> &'static str {
    "adl tooling github-release ensure-absent|ensure-present|draft|publish --repo <owner/repo> --tag <tag> [--name <name>] [--notes-file <path>] [--target <branch>]"
}

fn valid_repo(value: &str) -> bool {
    let mut parts = value.split('/');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(owner), Some(repo), None) if !owner.is_empty() && !repo.is_empty()
    )
}

fn run_release_action(args: &ReleaseArgs) -> Result<String> {
    let (owner, repo) = args
        .repo
        .split_once('/')
        .ok_or_else(|| anyhow!("github-release --repo must be owner/repo"))?;
    with_current_thread_runtime(
        "github_release.octocrab_runtime: failed to build runtime",
        |runtime| {
            let octo = build_octocrab()?;
            runtime.block_on(async move {
                match args.action {
                    ReleaseAction::EnsureAbsent => {
                        if release_by_tag(&octo, owner, repo, &args.tag)
                            .await?
                            .is_some()
                        {
                            bail!("GitHub release already exists for tag {}", args.tag);
                        }
                        Ok(format!("release_absent tag={}", args.tag))
                    }
                    ReleaseAction::EnsurePresent => {
                        let release = release_by_tag(&octo, owner, repo, &args.tag).await?;
                        if release.is_none() {
                            bail!("GitHub release does not exist for tag {}", args.tag);
                        }
                        Ok(format!("release_present tag={}", args.tag))
                    }
                    ReleaseAction::Draft => {
                        if release_by_tag(&octo, owner, repo, &args.tag)
                            .await?
                            .is_some()
                        {
                            bail!("GitHub release already exists for tag {}", args.tag);
                        }
                        let body = read_notes(args.notes_file.as_ref())?;
                        let name = args.name.as_deref().unwrap_or(&args.tag);
                        let repo_handler = octo.repos(owner, repo);
                        let releases = repo_handler.releases();
                        let mut builder = releases
                            .create(&args.tag)
                            .name(name)
                            .body(&body)
                            .draft(true)
                            .prerelease(false);
                        if let Some(target) = args.target.as_deref() {
                            builder = builder.target_commitish(target);
                        }
                        let release = builder.send().await.with_context(|| {
                            format!(
                                "github_release.octocrab_transport: failed to draft {}",
                                args.tag
                            )
                        })?;
                        Ok(format!(
                            "release_drafted tag={} url={}",
                            release.tag_name, release.html_url
                        ))
                    }
                    ReleaseAction::Publish => {
                        let release = release_by_tag(&octo, owner, repo, &args.tag)
                            .await?
                            .ok_or_else(|| {
                                anyhow!("GitHub release does not exist for tag {}", args.tag)
                            })?;
                        if !release.draft {
                            bail!("GitHub release for tag {} is not a draft", args.tag);
                        }
                        let updated = octo
                            .repos(owner, repo)
                            .releases()
                            .update(release_id_u64(&release)?)
                            .draft(false)
                            .send()
                            .await
                            .with_context(|| {
                                format!(
                                    "github_release.octocrab_transport: failed to publish {}",
                                    args.tag
                                )
                            })?;
                        Ok(format!(
                            "release_published tag={} url={}",
                            updated.tag_name, updated.html_url
                        ))
                    }
                }
            })
        },
    )
}

async fn release_by_tag(
    octo: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    tag: &str,
) -> Result<Option<Release>> {
    match octo.repos(owner, repo).releases().get_by_tag(tag).await {
        Ok(release) => Ok(Some(release)),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            Ok(None)
        }
        Err(err) => Err(anyhow!(
            "github_release.octocrab_transport: failed to inspect release {tag}: {err}"
        )),
    }
}

fn read_notes(path: Option<&PathBuf>) -> Result<String> {
    match path {
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("failed to read release notes file '{}'", path.display())),
        None => Ok(String::new()),
    }
}

fn release_id_u64(release: &Release) -> Result<u64> {
    let raw = format!("{:?}", release.id);
    raw.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .with_context(|| format!("failed to parse release id from {raw}"))
}

fn build_octocrab() -> Result<octocrab::Octocrab> {
    let client =
        AdlGithubClient::from_env().map_err(|err| anyhow!("github_release.credentials: {err}"))?;
    if client.backend() != GithubClientBackend::Octocrab {
        let credential_status = client
            .token_source()
            .map(|source| {
                format!(
                    "credential_status=token_present source={}",
                    source.env_name()
                )
            })
            .unwrap_or_else(|| "credential_status=missing_token".to_string());
        bail!(
            "github-release requires octocrab-backed ADL GitHub credentials; {}; set GITHUB_TOKEN or GH_TOKEN before live release operations; release operations do not use gh fallback",
            credential_status
        );
    }
    client
        .octocrab()
        .map_err(|err| anyhow!("github_release.octocrab_build: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests as cli_tests;
    use std::net::TcpListener;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tiny_http::{Header, Response, Server};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        cli_tests::env_lock()
    }

    fn json_response(body: String) -> Response<std::io::Cursor<Vec<u8>>> {
        Response::from_string(body).with_header(
            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                .expect("json content-type"),
        )
    }

    fn release_fixture(id: u64, tag: &str, draft: bool) -> String {
        serde_json::json!({
            "url": format!("https://api.github.test/repos/owner/repo/releases/{id}"),
            "html_url": format!("https://github.com/owner/repo/releases/tag/{tag}"),
            "assets_url": format!("https://api.github.test/repos/owner/repo/releases/{id}/assets"),
            "upload_url": format!("https://uploads.github.test/repos/owner/repo/releases/{id}/assets{{?name,label}}"),
            "tarball_url": null,
            "zipball_url": null,
            "id": id,
            "node_id": format!("RE_kwDO{id}"),
            "tag_name": tag,
            "target_commitish": "main",
            "name": format!("ADL {tag}"),
            "body": "release notes",
            "draft": draft,
            "prerelease": false,
            "immutable": false,
            "created_at": "2026-06-15T00:00:00Z",
            "published_at": if draft { serde_json::Value::Null } else { serde_json::json!("2026-06-15T00:01:00Z") },
            "author": null,
            "assets": []
        })
        .to_string()
    }

    fn spawn_release_server() -> (String, thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind release test listener");
        let bind_addr = listener.local_addr().expect("release test listener addr");
        let server = Server::from_listener(listener, None).expect("bind release test server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for _ in 0..6 {
                let Some(mut request) = server
                    .recv_timeout(Duration::from_secs(5))
                    .expect("release test server receive")
                else {
                    break;
                };
                let method = request.method().as_str().to_string();
                let url = request.url().to_string();
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                seen.push(format!("{method} {url} {body}"));
                let path = url.split('?').next().unwrap_or(url.as_str());
                let response = match (method.as_str(), path) {
                    ("GET", "/repos/owner/repo/releases/tags/v0.91.5-missing") => {
                        Response::from_string(r#"{"message":"Not Found"}"#).with_status_code(404)
                    }
                    ("GET", "/repos/owner/repo/releases/tags/v0.91.5") => {
                        json_response(release_fixture(123, "v0.91.5", true))
                    }
                    ("POST", "/repos/owner/repo/releases") => {
                        json_response(release_fixture(124, "v0.91.5-missing", true))
                    }
                    ("PATCH", "/repos/owner/repo/releases/123") => {
                        json_response(release_fixture(123, "v0.91.5", false))
                    }
                    _ => json_response(
                        serde_json::json!({
                            "message": format!("unexpected request {method} {url}")
                        })
                        .to_string(),
                    )
                    .with_status_code(500),
                };
                let _ = request.respond(response);
            }
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    #[test]
    fn github_release_octocrab_covers_absent_draft_present_publish() {
        let _guard = env_lock();
        let old_token = std::env::var("GITHUB_TOKEN").ok();
        let old_gh_token = std::env::var("GH_TOKEN").ok();
        let old_mode = std::env::var("ADL_GITHUB_CLIENT").ok();
        let old_base = std::env::var("ADL_GITHUB_OCTOCRAB_BASE_URI").ok();
        let (base_uri, server) = spawn_release_server();
        let notes_path = std::env::temp_dir().join(format!(
            "adl-release-notes-{}.md",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        fs::write(&notes_path, "release notes\n").expect("write notes");
        unsafe {
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::remove_var("GH_TOKEN");
            std::env::remove_var("ADL_GITHUB_CLIENT");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        }

        real_github_release(&[
            "ensure-absent".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5-missing".into(),
        ])
        .expect("ensure absent");
        real_github_release(&[
            "draft".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5-missing".into(),
            "--name".into(),
            "ADL v0.91.5-missing".into(),
            "--notes-file".into(),
            notes_path.to_string_lossy().to_string(),
            "--target".into(),
            "main".into(),
        ])
        .expect("draft release");
        real_github_release(&[
            "ensure-present".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5".into(),
        ])
        .expect("ensure present");
        real_github_release(&[
            "publish".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5".into(),
        ])
        .expect("publish release");

        let seen = server.join().expect("server join");
        assert_eq!(seen.len(), 6, "unexpected release API calls: {seen:#?}");
        assert!(seen
            .iter()
            .any(|call| call.starts_with("POST /repos/owner/repo/releases ")));
        assert!(seen.iter().any(|call| call.contains("\"draft\":true")));
        assert!(seen
            .iter()
            .any(|call| call.starts_with("PATCH /repos/owner/repo/releases/123 ")));
        assert!(seen.iter().any(|call| call.contains("\"draft\":false")));

        restore_env("GITHUB_TOKEN", old_token);
        restore_env("GH_TOKEN", old_gh_token);
        restore_env("ADL_GITHUB_CLIENT", old_mode);
        restore_env("ADL_GITHUB_OCTOCRAB_BASE_URI", old_base);
        let _ = fs::remove_file(notes_path);
    }

    #[test]
    fn github_release_octocrab_accepts_gh_token_when_github_token_missing() {
        let _guard = env_lock();
        let old_token = std::env::var("GITHUB_TOKEN").ok();
        let old_gh_token = std::env::var("GH_TOKEN").ok();
        let old_mode = std::env::var("ADL_GITHUB_CLIENT").ok();
        let old_base = std::env::var("ADL_GITHUB_OCTOCRAB_BASE_URI").ok();
        let (base_uri, server) = spawn_release_server();
        unsafe {
            std::env::remove_var("GITHUB_TOKEN");
            std::env::set_var("GH_TOKEN", "test-gh-token");
            std::env::remove_var("ADL_GITHUB_CLIENT");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        }

        real_github_release(&[
            "ensure-present".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5".into(),
        ])
        .expect("ensure present with GH_TOKEN");

        let seen = server.join().expect("server join");
        assert!(
            seen.iter()
                .any(|call| call.starts_with("GET /repos/owner/repo/releases/tags/v0.91.5")),
            "unexpected release API calls: {seen:#?}"
        );

        restore_env("GITHUB_TOKEN", old_token);
        restore_env("GH_TOKEN", old_gh_token);
        restore_env("ADL_GITHUB_CLIENT", old_mode);
        restore_env("ADL_GITHUB_OCTOCRAB_BASE_URI", old_base);
    }

    #[test]
    fn github_release_rejects_gh_fallback_mode_even_with_token() {
        let _guard = env_lock();
        let old_token = std::env::var("GITHUB_TOKEN").ok();
        let old_gh_token = std::env::var("GH_TOKEN").ok();
        let old_mode = std::env::var("ADL_GITHUB_CLIENT").ok();
        unsafe {
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::remove_var("GH_TOKEN");
            std::env::set_var("ADL_GITHUB_CLIENT", "gh");
        }

        let err = real_github_release(&[
            "ensure-absent".into(),
            "--repo".into(),
            "owner/repo".into(),
            "--tag".into(),
            "v0.91.5".into(),
        ])
        .expect_err("gh fallback mode should be rejected");
        let message = format!("{err:#}");
        assert!(
            message.contains("do not use gh fallback"),
            "unexpected error: {message}"
        );

        restore_env("GITHUB_TOKEN", old_token);
        restore_env("GH_TOKEN", old_gh_token);
        restore_env("ADL_GITHUB_CLIENT", old_mode);
    }

    fn restore_env(name: &str, value: Option<String>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var(name, value);
            } else {
                std::env::remove_var(name);
            }
        }
    }
}
