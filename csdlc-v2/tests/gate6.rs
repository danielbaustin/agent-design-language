use csdlc_v2::{
    publication::{validate_merged_remote, validate_remote},
    reconcile_action, MergedPublicationReconciliationRequest, PublicationAction, PublicationIntent,
    PublicationRequest, RemotePullRequest,
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
fn ambiguous_create_outage_is_reconciled_by_observation_before_retry() {
    let i = intent();
    assert_eq!(
        reconcile_action(&i, None).unwrap(),
        PublicationAction::Create
    );
    assert_eq!(
        reconcile_action(&i, Some(&remote())).unwrap(),
        PublicationAction::Noop
    );
    assert_eq!(
        reconcile_action(&i, None).unwrap(),
        PublicationAction::Create,
        "an outage with no observed side effect remains safely retryable"
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
    assert!(bundle
        .get("merged_publication_reconciliation_request")
        .is_some());
    assert!(bundle.get("publication_intent").is_some());
    assert!(bundle.get("remote_pull_request").is_some());
}

#[test]
fn merged_reconciliation_request_is_versioned_and_requires_explicit_pr() {
    let publication = PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 5466,
        expected_generation: 1,
        expected_digest: "digest".into(),
        claim_id: "claim".into(),
        actor: "operator".into(),
        repository: "owner/repo".into(),
        base: "main".into(),
        head: "codex/5466".into(),
        title: "title".into(),
        body: "Resolves #5466".into(),
        draft: true,
        remote: "origin".into(),
        token_file: None,
    };
    let mut request = MergedPublicationReconciliationRequest {
        schema: "csdlc.merged_publication_reconciliation_request.v1".into(),
        publication,
        pull_request: 7,
    };
    assert!(request.validate().is_ok());
    request.schema = "csdlc.merged_publication_reconciliation_request.v0".into();
    assert!(request.validate().is_err());
    request.schema = "csdlc.merged_publication_reconciliation_request.v1".into();
    request.pull_request = 0;
    assert!(request.validate().is_err());
}

#[test]
fn merged_reconciliation_requires_exact_final_reviewed_identity() {
    let mut i = intent();
    i.draft = false;
    let mut r = remote();
    r.draft = false;
    r.state = "merged".into();
    assert!(validate_merged_remote(&i, &r).is_ok());

    for drift in 0..8 {
        let mut candidate = r.clone();
        match drift {
            0 => candidate.repository = "other/repo".into(),
            1 => candidate.base = "release".into(),
            2 => candidate.head = "codex/wrong".into(),
            3 => candidate.title = "wrong".into(),
            4 => candidate.body = "wrong #5236".into(),
            5 => candidate.head_sha = "wrong".into(),
            6 => candidate.draft = true,
            _ => candidate.state = "closed".into(),
        }
        assert!(
            validate_merged_remote(&i, &candidate).is_err(),
            "drift case {drift} must fail closed"
        );
    }

    let mut draft_intent = i;
    draft_intent.draft = true;
    assert!(validate_merged_remote(&draft_intent, &r).is_err());
}
