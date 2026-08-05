use csdlc_v2::{
    publication::{body_has_github_closing_keyword, validate_remote},
    reconcile_action, PublicationAction, PublicationIntent, RemotePullRequest,
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
    let intent = intent();
    RemotePullRequest {
        number: 7,
        url: "https://example.invalid/pr/7".into(),
        repository: intent.repository,
        base: intent.base,
        head: intent.head,
        title: intent.title,
        body: intent.body,
        draft: intent.draft,
        state: "open".into(),
        head_sha: intent.commit_sha,
    }
}

#[test]
fn ambiguous_create_outage_is_reconciled_by_observation_before_retry() {
    let intent = intent();
    assert_eq!(
        reconcile_action(&intent, None).unwrap(),
        PublicationAction::Create
    );
    assert_eq!(
        reconcile_action(&intent, Some(&remote())).unwrap(),
        PublicationAction::Noop
    );
}

#[test]
fn drifted_mutable_fields_update_same_pr() {
    let intent = intent();
    let mut remote = remote();
    remote.body = "Closes #5236\nOld text".into();
    assert_eq!(
        reconcile_action(&intent, Some(&remote)).unwrap(),
        PublicationAction::Update
    );
}

#[test]
fn base_head_or_repository_mismatch_fails_closed() {
    let intent = intent();
    for field in 0..3 {
        let mut remote = remote();
        match field {
            0 => remote.base = "release".into(),
            1 => remote.head = "wrong".into(),
            _ => remote.repository = "other/repo".into(),
        };
        assert!(validate_remote(&intent, &remote).is_err());
    }
}

#[test]
fn publication_body_requires_github_closing_keyword_for_issue() {
    for body in [
        "Closes #5236",
        "fixes #5236",
        "Resolved #5236",
        "Closes: agent-logic/agent-design-language#5236",
    ] {
        assert!(body_has_github_closing_keyword(
            body,
            5236,
            "agent-logic/agent-design-language"
        ));
    }
    for body in [
        "Related #5236",
        "See #5236",
        "Closes #52360",
        "Closes issue 5236",
        "Close\n#5236",
        "Closes wrong/repo#5236",
    ] {
        assert!(!body_has_github_closing_keyword(
            body,
            5236,
            "agent-logic/agent-design-language"
        ));
    }
}

#[test]
fn public_schema_keeps_publication_and_drops_merged_reconciliation() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle.get("publication_request").is_some());
    assert!(bundle.get("publication_intent").is_some());
    assert!(bundle.get("remote_pull_request").is_some());
    assert!(bundle
        .get("merged_publication_reconciliation_request")
        .is_none());
}
