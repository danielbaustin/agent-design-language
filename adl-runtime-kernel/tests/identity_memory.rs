use std::collections::{BTreeMap, BTreeSet};

use adl_runtime_kernel::{IdentityAuthority, IdentityMemoryError, MemoryClass, MemoryLedger};

fn authority() -> IdentityAuthority {
    IdentityAuthority::from_bytes("identity-test-key", &[11_u8; 32])
}

fn trusted_keys(authority: &IdentityAuthority) -> BTreeMap<String, ed25519_dalek::VerifyingKey> {
    BTreeMap::from([("identity-test-key".to_owned(), authority.verifying_key())])
}

fn binding(authority: &IdentityAuthority) -> adl_runtime_kernel::IdentityBinding {
    authority
        .bind(
            "citizen-alpha",
            "runtime-v3",
            "continuity-alpha",
            1,
            BTreeSet::from(["memory.write".to_owned(), "lifelog.project".to_owned()]),
        )
        .unwrap()
}

fn facts(values: &[(&str, &str)]) -> BTreeMap<String, String> {
    values
        .iter()
        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
        .collect()
}

#[test]
fn signed_identity_binding_allows_memory_checkpoint_and_redacted_lifelog() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    let private_ref = format!("private-state:{}", "7".repeat(64));

    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada"), ("private_note", "hidden")]),
            Some(private_ref.clone()),
        )
        .unwrap();
    let head = ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Semantic,
            facts(&[("preference", "deterministic-runtime")]),
            None,
        )
        .unwrap();

    let checkpoint = ledger.checkpoint(&binding, &keys).unwrap();
    assert_eq!(checkpoint.accepted_through, 2);
    assert_eq!(checkpoint.head_hash, head);
    assert_eq!(checkpoint.facts["display_name"], "Ada");
    assert_eq!(checkpoint.private_refs, vec![private_ref]);

    let lifelog = ledger
        .lifelog(
            &binding,
            &keys,
            &BTreeSet::from(["display_name".to_owned()]),
        )
        .unwrap();
    assert_eq!(lifelog.len(), 2);
    assert_eq!(lifelog[0].visible_fields["display_name"], "Ada");
    assert_eq!(
        lifelog[0].redacted_fields,
        vec!["private_note".to_owned(), "private_state_ref".to_owned()]
    );
    assert!(!serde_json::to_string(&lifelog)
        .unwrap()
        .contains("private-state:"));
    assert!(!serde_json::to_string(&lifelog).unwrap().contains("hidden"));
}

#[test]
fn forged_identity_binding_is_rejected_before_memory_append() {
    let authority = authority();
    let mut binding = binding(&authority);
    binding.citizen_id = "citizen-forged".to_owned();
    let mut ledger = MemoryLedger::default();

    let err = ledger
        .append(
            &binding,
            &trusted_keys(&authority),
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            None,
        )
        .unwrap_err();
    assert_eq!(err, IdentityMemoryError::Signature);
}

#[test]
fn invalid_private_state_reference_fails_closed() {
    let authority = authority();
    let binding = binding(&authority);
    let mut ledger = MemoryLedger::default();

    let err = ledger
        .append(
            &binding,
            &trusted_keys(&authority),
            MemoryClass::Episodic,
            facts(&[("event", "checkpoint")]),
            Some("raw-private-state".to_owned()),
        )
        .unwrap_err();
    assert_eq!(err, IdentityMemoryError::InvalidPrivateReference);
}

#[test]
fn restore_requires_matching_identity_and_head_before_continuing() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            None,
        )
        .unwrap();
    let checkpoint = ledger.checkpoint(&binding, &keys).unwrap();

    let mut restored = MemoryLedger::restore(&checkpoint, &binding, &keys).unwrap();
    let bad_head = "8".repeat(64);
    let err = restored
        .append_after_restore(
            &binding,
            &keys,
            &bad_head,
            MemoryClass::Procedural,
            facts(&[("skill", "runtime-v3")]),
            None,
        )
        .unwrap_err();
    assert_eq!(err, IdentityMemoryError::ContinuityMismatch);

    restored
        .append_after_restore(
            &binding,
            &keys,
            &checkpoint.head_hash,
            MemoryClass::Procedural,
            facts(&[("skill", "runtime-v3")]),
            None,
        )
        .unwrap();
}

#[test]
fn restore_rejects_checkpoint_bound_to_other_citizen() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            None,
        )
        .unwrap();
    let mut checkpoint = ledger.checkpoint(&binding, &keys).unwrap();
    checkpoint.citizen_id = "citizen-other".to_owned();

    let err = MemoryLedger::restore(&checkpoint, &binding, &keys).unwrap_err();
    assert_eq!(err, IdentityMemoryError::ContinuityMismatch);
}

#[test]
fn checkpoint_lifelog_and_restore_reject_forged_binding() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            None,
        )
        .unwrap();
    let checkpoint = ledger.checkpoint(&binding, &keys).unwrap();
    let mut forged = binding.clone();
    forged.runtime_id = "runtime-forged".to_owned();

    assert_eq!(
        ledger.checkpoint(&forged, &keys).unwrap_err(),
        IdentityMemoryError::Signature
    );
    assert_eq!(
        ledger
            .lifelog(&forged, &keys, &BTreeSet::from(["display_name".to_owned()]))
            .unwrap_err(),
        IdentityMemoryError::Signature
    );
    assert_eq!(
        MemoryLedger::restore(&checkpoint, &forged, &keys).unwrap_err(),
        IdentityMemoryError::Signature
    );
}

#[test]
fn same_continuity_different_citizen_or_runtime_cannot_append() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            None,
        )
        .unwrap();

    let other_citizen = authority
        .bind(
            "citizen-other",
            "runtime-v3",
            "continuity-alpha",
            2,
            BTreeSet::from(["memory.write".to_owned()]),
        )
        .unwrap();
    let err = ledger
        .append(
            &other_citizen,
            &keys,
            MemoryClass::Semantic,
            facts(&[("preference", "mix")]),
            None,
        )
        .unwrap_err();
    assert_eq!(err, IdentityMemoryError::ContinuityOwnerMismatch);

    let other_runtime = authority
        .bind(
            "citizen-alpha",
            "runtime-other",
            "continuity-alpha",
            2,
            BTreeSet::from(["memory.write".to_owned()]),
        )
        .unwrap();
    let err = ledger
        .append(
            &other_runtime,
            &keys,
            MemoryClass::Semantic,
            facts(&[("preference", "mix")]),
            None,
        )
        .unwrap_err();
    assert_eq!(err, IdentityMemoryError::ContinuityOwnerMismatch);
}

#[test]
fn restored_checkpoint_preserves_summary_across_next_checkpoint() {
    let authority = authority();
    let binding = binding(&authority);
    let keys = trusted_keys(&authority);
    let mut ledger = MemoryLedger::default();
    let private_ref = format!("private-state:{}", "9".repeat(64));
    ledger
        .append(
            &binding,
            &keys,
            MemoryClass::Identity,
            facts(&[("display_name", "Ada")]),
            Some(private_ref.clone()),
        )
        .unwrap();
    let checkpoint = ledger.checkpoint(&binding, &keys).unwrap();
    let mut restored = MemoryLedger::restore(&checkpoint, &binding, &keys).unwrap();
    restored
        .append_after_restore(
            &binding,
            &keys,
            &checkpoint.head_hash,
            MemoryClass::Semantic,
            facts(&[("preference", "deterministic-runtime")]),
            None,
        )
        .unwrap();

    let next = restored.checkpoint(&binding, &keys).unwrap();
    assert_eq!(next.accepted_through, 2);
    assert_eq!(next.facts["display_name"], "Ada");
    assert_eq!(next.facts["preference"], "deterministic-runtime");
    assert_eq!(next.private_refs, vec![private_ref]);
}
