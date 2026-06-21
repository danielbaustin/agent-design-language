use super::{
    form_fields, sample_values, PromptCardForm, PromptCardKind, PLACEHOLDERS, VALUES_SCHEMA,
};
use anyhow::{anyhow, ensure, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub(super) struct PromptValuesDocument {
    pub(super) schema: String,
    pub(super) template_set: String,
    pub(super) card_kind: Option<String>,
    pub(super) system: BTreeMap<String, String>,
    pub(super) values: BTreeMap<String, String>,
}

impl PromptValuesDocument {
    pub(super) fn merged_values(&self) -> BTreeMap<String, String> {
        let mut merged = self.system.clone();
        merged.extend(self.values.clone());
        merged
    }

    pub(super) fn to_yaml(&self, card: &PromptCardForm) -> String {
        let mut out = String::new();
        out.push_str(&format!("schema: {}\n", yaml_scalar(&self.schema)));
        out.push_str(&format!(
            "template_set: {}\n",
            yaml_scalar(&self.template_set)
        ));
        out.push_str(&format!(
            "card_kind: {}\n",
            yaml_scalar(self.card_kind.as_deref().unwrap_or(card.key))
        ));
        out.push_str("system:\n");
        write_yaml_mapping(&mut out, &self.system);
        out.push_str("values:\n");
        write_yaml_mapping(&mut out, &self.values);
        out
    }
}

pub(super) fn load_values_file(
    card: &PromptCardForm,
    path: &Path,
    expected_template_set: &str,
) -> Result<BTreeMap<String, String>> {
    Ok(load_values_document(card, path, expected_template_set)?.merged_values())
}

pub(super) fn load_values_document(
    card: &PromptCardForm,
    path: &Path,
    expected_template_set: &str,
) -> Result<PromptValuesDocument> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read values file {}", path.display()))?;
    let doc: serde_yaml::Value = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse values file {}", path.display()))?;
    let mapping = doc
        .as_mapping()
        .ok_or_else(|| anyhow!("values file must be a YAML/JSON mapping"))?;

    let schema = mapping_string_value(mapping, "schema")?
        .ok_or_else(|| anyhow!("values file must declare schema"))?;
    ensure!(
        schema == VALUES_SCHEMA,
        "values schema must be {VALUES_SCHEMA}: got {schema}"
    );
    let template_set = mapping_string_value(mapping, "template_set")?
        .ok_or_else(|| anyhow!("values file must declare template_set"))?;
    ensure!(
        template_set == expected_template_set,
        "values template_set must match active template set: expected {}, got {}",
        expected_template_set,
        template_set
    );
    if let Some(card_kind) = mapping_string_value(mapping, "card_kind")? {
        ensure!(
            card_kind == card.key,
            "values card_kind must match requested kind: expected {}, got {}",
            card.key,
            card_kind
        );
    }

    let system = if let Some(system) = mapping.get(serde_yaml::Value::String("system".to_string()))
    {
        collect_values_section(card, system, false, "system")?
    } else {
        BTreeMap::new()
    };
    let values =
        if let Some(editable) = mapping.get(serde_yaml::Value::String("values".to_string())) {
            collect_values_section(card, editable, true, "values")?
        } else {
            BTreeMap::new()
        };

    ensure!(
        !system.is_empty() || !values.is_empty(),
        "values file must contain system and/or values mappings"
    );
    Ok(PromptValuesDocument {
        schema,
        template_set,
        card_kind: mapping_string_value(mapping, "card_kind")?,
        system,
        values,
    })
}

fn collect_values_section(
    card: &PromptCardForm,
    section: &serde_yaml::Value,
    ordinary_values: bool,
    section_name: &str,
) -> Result<BTreeMap<String, String>> {
    let mapping = section
        .as_mapping()
        .ok_or_else(|| anyhow!("{section_name} must be a mapping"))?;
    let mut out = BTreeMap::new();
    for (key, value) in mapping {
        let key = key
            .as_str()
            .ok_or_else(|| anyhow!("{section_name} keys must be strings"))?;
        let field = card.fields.iter().find(|field| field.key == key);
        let known_placeholder = PLACEHOLDERS.contains(&key);
        ensure!(
            field.is_some() || known_placeholder,
            "{}.{} is not a supported field for {}",
            section_name,
            key,
            card.key
        );
        if ordinary_values {
            ensure!(
                field.is_some_and(|field| field.editable),
                "values.{} is locked; supply it through system fields",
                key
            );
        } else if let Some(field) = field {
            ensure!(
                !field.editable,
                "system.{} is editable; supply it through values fields",
                key
            );
        }
        let value = scalar_to_string(value)
            .ok_or_else(|| anyhow!("{}.{} must be a scalar value", section_name, key))?;
        out.insert(key.to_string(), value);
    }
    Ok(out)
}

fn scalar_to_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        serde_yaml::Value::Null => Some(String::new()),
        _ => None,
    }
}

fn mapping_string_value(mapping: &serde_yaml::Mapping, key: &str) -> Result<Option<String>> {
    let Some(value) = mapping.get(serde_yaml::Value::String(key.to_string())) else {
        return Ok(None);
    };
    let value = value
        .as_str()
        .ok_or_else(|| anyhow!("{key} must be a string"))?;
    Ok(Some(value.to_string()))
}

pub(super) fn write_yaml_mapping(out: &mut String, values: &BTreeMap<String, String>) {
    if values.is_empty() {
        out.push_str("  {}\n");
        return;
    }
    for (key, value) in values {
        if value.contains('\n') {
            let (indicator, body) = if let Some(stripped) = value.strip_suffix('\n') {
                ("|", stripped)
            } else {
                ("|-", value.as_str())
            };
            out.push_str(&format!("  {key}: {indicator}\n"));
            for line in body.split('\n') {
                out.push_str("    ");
                out.push_str(line);
                out.push('\n');
            }
        } else {
            out.push_str(&format!("  {key}: {}\n", yaml_scalar(value)));
        }
    }
}

fn yaml_scalar(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

pub(super) fn sample_values_document(kind: PromptCardKind, template_set: &str) -> String {
    let mut values = sample_values();
    if kind == PromptCardKind::Spp {
        values.insert("status".to_string(), "draft".to_string());
        values.insert("activation_state".to_string(), "draft".to_string());
    } else if kind == PromptCardKind::Vpp {
        values.insert("status".to_string(), "draft".to_string());
    }
    let editable_keys = form_fields(kind)
        .iter()
        .filter(|field| field.editable)
        .map(|field| field.key)
        .collect::<BTreeSet<_>>();
    let system_keys = PLACEHOLDERS
        .iter()
        .copied()
        .filter(|key| !editable_keys.contains(key))
        .collect::<BTreeSet<_>>();

    let mut out = String::new();
    out.push_str("schema: adl.csdlc.prompt_template_values.v1\n");
    out.push_str(&format!("template_set: {template_set}\n"));
    out.push_str(&format!("card_kind: {}\n", kind.key()));
    out.push_str("system:\n");
    for key in system_keys {
        let value = values.get(key).map(String::as_str).unwrap_or_default();
        out.push_str(&format!("  {}: {}\n", key, yaml_quote(value)));
    }
    out.push_str("values:\n");
    for key in editable_keys {
        let value = values.get(key).map(String::as_str).unwrap_or_default();
        out.push_str(&format!("  {}: {}\n", key, yaml_quote(value)));
    }
    out
}

fn yaml_quote(value: &str) -> String {
    if value.contains('\n') {
        let mut out = String::from("|-\n");
        for line in value.lines() {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        return out.trim_end().to_string();
    }
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\t', "\\t")
    )
}
