use crate::uts_acc_compiler::ToolProposalV1;

pub(crate) fn fenced_json_body(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if !trimmed.starts_with("```") {
        return None;
    }
    let after_open = trimmed.find('\n')?;
    let body = &trimmed[(after_open + 1)..];
    let close = body.rfind("```")?;
    Some(body[..close].trim())
}

pub(crate) fn first_json_object_body(raw: &str) -> Option<&str> {
    let start = raw.find('{')?;
    let bytes = raw.as_bytes();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (idx, byte) in bytes.iter().enumerate().skip(start) {
        let ch = *byte as char;
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match ch {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                if depth == 0 {
                    return Some(raw[start..=idx].trim());
                }
            }
            _ => {}
        }
    }

    None
}

pub(crate) fn parse_model_turn_response(raw: &str) -> Option<ToolProposalTurnResponseV1> {
    if let Some(fenced) = fenced_json_body(raw) {
        return serde_json::from_str(fenced).ok();
    }
    if let Some(body) = first_json_object_body(raw) {
        return serde_json::from_str(body).ok();
    }
    serde_json::from_str(raw.trim()).ok()
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ToolProposalTurnResponseV1 {
    pub narrative: String,
    pub proposal: Option<ToolProposalV1>,
}

pub(crate) fn bounded_response_excerpt(raw: &str) -> Option<String> {
    let normalized = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    let trimmed = normalized.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut excerpt = trimmed.chars().take(240).collect::<String>();
    if trimmed.chars().count() > 240 {
        excerpt.push_str("...");
    }
    Some(excerpt)
}
