use super::support::*;
use super::*;

#[test]
fn prompt_spec_validation_accepts_canonical_spec() {
    let spec = valid_prompt_spec_yaml();
    validate_prompt_spec(&spec).expect("canonical prompt spec should validate");
    assert_eq!(
        prompt_spec_sections(&spec),
        vec![
            "goal",
            "required_outcome",
            "acceptance_criteria",
            "inputs",
            "target_files_surfaces",
            "validation_plan",
            "demo_proof_requirements",
            "constraints_policies",
            "system_invariants",
            "reviewer_checklist",
            "non_goals_out_of_scope",
            "notes_risks",
            "instructions_to_agent",
        ]
    );
    assert_eq!(
        prompt_spec_bool(&spec, "include_system_invariants"),
        Some(true)
    );
    assert_eq!(
        prompt_spec_bool(&spec, "required_outcome_type_supported"),
        Some(true)
    );
    let extracted = extract_prompt_spec_yaml(&format!(
        "# Heading\n\n## Prompt Spec\n```yaml\n{}\n```\n",
        spec
    ))
    .expect("prompt spec block should extract");
    assert!(extracted.contains("prompt_schema: adl.v1"));
}
