use csdlc_v2::{
    publication::validate_remote, reconcile_action, PublicationAction, PublicationIntent,
    RemotePullRequest,
};

fn intent() -> PublicationIntent {
    PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: 5236,
        repository: "agent-logic/agent-design-language".into(),
        base: "main".into(),
        head: "codex/5236".into(),
        title: "Gate 6".into(),
        body: "Closes #5236".into(),
        draft: true,
        revision: "revision".into(),
        commit_sha: "abc123".into(),
    }
}
fn remote() -> RemotePullRequest {
    let i = intent();
    RemotePullRequest {
        number: 7,
        url: "https://example.invalid/pr/7".into(),
        repository: i.repository,
        base: i.base,
        head: i.head,
        title: i.title,
        body: i.body,
        draft: i.draft,
        state: "open".into(),
        head_sha: i.commit_sha,
    }
}

#[test]
fn absent_remote_creates_and_exact_retry_is_noop() {
    let i = intent();
    assert_eq!(
        reconcile_action(&i, None).unwrap(),
        PublicationAction::Create
    );
    assert_eq!(
        reconcile_action(&i, Some(&remote())).unwrap(),
        PublicationAction::Noop
    );
}

#[test]
fn drifted_mutable_fields_update_same_pr() {
    let i = intent();
    let mut r = remote();
    r.body = "Old text for #5236".into();
    assert_eq!(
        reconcile_action(&i, Some(&r)).unwrap(),
        PublicationAction::Update
    );
}

#[test]
fn base_head_or_repository_mismatch_fails_closed() {
    let i = intent();
    for field in 0..3 {
        let mut r = remote();
        match field {
            0 => r.base = "release".into(),
            1 => r.head = "wrong".into(),
            _ => r.repository = "other/repo".into(),
        };
        assert!(validate_remote(&i, &r).is_err());
    }
}

#[test]
fn request_and_result_schemas_are_published() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle.get("publication_request").is_some());
    assert!(bundle.get("publication_intent").is_some());
    assert!(bundle.get("remote_pull_request").is_some());
}
