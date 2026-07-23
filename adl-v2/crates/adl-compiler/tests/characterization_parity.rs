use adl_compiler::compile;
use adl_language::parse_and_validate_yaml;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const APPLICABLE: &[&str] = &[
    "map-a.adl.yaml",
    "map-b.adl.yaml",
    "mock-run.adl.yaml",
    "sequential-a.adl.yaml",
    "sequential-b.adl.yaml",
    "six-primitives.adl.yaml",
];
const LEGACY_PATTERNS: &[&str] = &[
    "branch-a.adl.yaml",
    "branch-b.adl.yaml",
    "fork-join.adl.yaml",
];
const LANGUAGE_NEGATIVE: &[&str] = &[
    "cycle.adl.yaml",
    "malformed.adl.yaml",
    "schema-unknown.adl.yaml",
    "state-missing.adl.yaml",
    "unknown-agent.adl.yaml",
    "unknown-provider.adl.yaml",
    "unknown-task.adl.yaml",
    "unknown-tool.adl.yaml",
    "unknown-workflow.adl.yaml",
    "unsupported-run-field.adl.yaml",
];

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../adl-characterization/corpus/v1/fixtures")
}

#[test]
fn every_landed_fixture_has_one_explicit_treatment() {
    let observed: BTreeSet<String> = fs::read_dir(fixture_root())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".adl.yaml"))
        .collect();
    let classified: BTreeSet<String> = APPLICABLE
        .iter()
        .chain(LEGACY_PATTERNS)
        .chain(LANGUAGE_NEGATIVE)
        .map(|name| (*name).to_owned())
        .collect();
    assert_eq!(
        observed, classified,
        "fixture inventory changed without classification"
    );

    for name in APPLICABLE {
        let source = fs::read_to_string(fixture_root().join(name)).unwrap();
        let document = parse_and_validate_yaml(&source).unwrap_or_else(|error| {
            panic!("applicable fixture {name} failed language validation: {error:?}")
        });
        let plan = compile(&document)
            .unwrap_or_else(|error| panic!("applicable fixture {name} failed compile: {error:?}"));
        assert!(
            !plan.nodes.is_empty(),
            "applicable fixture {name} produced no nodes"
        );
    }

    for name in LEGACY_PATTERNS {
        let source = fs::read_to_string(fixture_root().join(name)).unwrap();
        let diagnostics = parse_and_validate_yaml(&source).unwrap_err();
        assert!(
            format!("{diagnostics:?}").contains("pattern"),
            "legacy fixture {name} was not rejected at the pattern boundary"
        );
    }

    for name in LANGUAGE_NEGATIVE {
        let source = fs::read_to_string(fixture_root().join(name)).unwrap();
        assert!(
            parse_and_validate_yaml(&source).is_err(),
            "negative fixture {name} unexpectedly passed language validation"
        );
    }
}
