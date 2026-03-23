use anyhow::Result;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

use crate::adl::PromptSpec;
use crate::resolve::AdlResolved;

/// Hash rendered prompt text for trace events.
pub fn hash_prompt(prompt_text: &str) -> String {
    let mut h = DefaultHasher::new();
    prompt_text.hash(&mut h);
    format!("{:016x}", h.finish())
}

fn assemble_prompt(p: &PromptSpec, params: &HashMap<String, String>) -> String {
    // MVP: just concatenate the structured fields in a deterministic order.
    // Later we’ll do templating, variables, role blocks, etc.
    let mut out = String::new();

    let render = |s: &str| -> String {
        // Very small MVP templating: replace occurrences of {{key}} with values.
        // (Intentionally simple and deterministic; we can evolve this later.)
        let mut rendered = s.to_string();
        for (k, v) in params.iter() {
            let needle = format!("{{{{{k}}}}}");
            if rendered.contains(&needle) {
                rendered = rendered.replace(&needle, v);
            }
        }
        rendered
    };

    if let Some(s) = p.system.as_deref() {
        out.push_str("SYSTEM:\n");
        out.push_str(&render(s));
        out.push_str("\n\n");
    }
    if let Some(d) = p.developer.as_deref() {
        out.push_str("DEVELOPER:\n");
        out.push_str(&render(d));
        out.push_str("\n\n");
    }
    if let Some(u) = p.user.as_deref() {
        out.push_str("USER:\n");
        out.push_str(&render(u));
        out.push_str("\n\n");
    }
    if let Some(c) = p.context.as_deref() {
        out.push_str("CONTEXT:\n");
        out.push_str(&render(c));
        out.push_str("\n\n");
    }
    if let Some(o) = p.output.as_deref() {
        out.push_str("OUTPUT:\n");
        out.push_str(&render(o));
        out.push_str("\n\n");
    }

    out.trim().to_string()
}

/// Assemble a prompt and return it as a string (used for trace/debug).
///
/// This intentionally does not print anything; callers decide what to log.
pub fn trace_prompt_assembly(p: &PromptSpec, params: &HashMap<String, String>) -> String {
    assemble_prompt(p, params)
}

/// Print the assembled prompts for each step.
pub fn print_prompts(resolved: &AdlResolved) -> Result<()> {
    for (idx, step) in resolved.steps.iter().enumerate() {
        let p = match step.effective_prompt_with_defaults(resolved) {
            Some(p) => p,
            None => {
                println!("Step {idx} ({}) has no prompt.", step.id);
                continue;
            }
        };

        let rendered = assemble_prompt(&p, &step.inputs);
        println!("--- step {idx}: {} ---", step.id);
        println!("{rendered}");
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_prompt_is_deterministic() {
        let a = hash_prompt("hello world");
        let b = hash_prompt("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn hash_prompt_changes_with_input() {
        let a = hash_prompt("hello world");
        let b = hash_prompt("hello world!");
        assert_ne!(a, b);
    }

    #[test]
    fn trace_prompt_assembly_substitutes_inputs() {
        let prompt = PromptSpec {
            system: Some("System {{sys}}".to_string()),
            developer: None,
            user: Some("Hello {{name}}".to_string()),
            context: None,
            output: None,
        };

        let mut inputs: HashMap<String, String> = HashMap::new();
        inputs.insert("name".to_string(), "Daniel".to_string());
        inputs.insert("sys".to_string(), "ADL".to_string());

        let assembled = trace_prompt_assembly(&prompt, &inputs);

        assert!(assembled.contains("Daniel"));
        assert!(assembled.contains("ADL"));
        assert!(assembled.contains("Hello"));
        assert!(assembled.contains("SYSTEM:"));
        assert!(assembled.contains("USER:"));
    }

    #[test]
    fn trace_prompt_assembly_handles_missing_inputs_gracefully() {
        let prompt = PromptSpec {
            system: None,
            developer: None,
            user: Some("Hello {{missing}}".to_string()),
            context: None,
            output: None,
        };

        let inputs: HashMap<String, String> = HashMap::new();
        let assembled = trace_prompt_assembly(&prompt, &inputs);

        // We don't enforce replacement semantics yet; just ensure no panic and content present.
        assert!(assembled.contains("Hello"));
        assert!(assembled.contains("{{missing}}"));
    }
}
