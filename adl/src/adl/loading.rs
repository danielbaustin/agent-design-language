use anyhow::{anyhow, Context, Result};
use serde_yaml::{Mapping as YamlMapping, Value as YamlValue};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub(super) fn load_yaml_with_includes(path: &Path, stack: &mut Vec<PathBuf>) -> Result<YamlValue> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("resolve include path: {}", path.display()))?;
    if let Some(idx) = stack.iter().position(|p| p == &canonical) {
        let mut cycle = stack[idx..]
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();
        cycle.push(canonical.display().to_string());
        return Err(anyhow!("include cycle detected: {}", cycle.join(" -> ")));
    }

    stack.push(canonical.clone());
    let result = (|| -> Result<YamlValue> {
        let text = fs::read_to_string(&canonical)
            .with_context(|| format!("read adl include file: {}", canonical.display()))?;
        let mut doc: YamlValue = serde_yaml::from_str(&text)
            .with_context(|| format!("parse adl yaml: {}", canonical.display()))?;
        let map = doc.as_mapping_mut().ok_or_else(|| {
            anyhow!(
                "top-level ADL document must be a mapping: {}",
                canonical.display()
            )
        })?;

        let include_key = YamlValue::String("include".to_string());
        let include_list = map.remove(&include_key);

        let mut merged = YamlMapping::new();

        if let Some(include_list) = include_list {
            let includes = include_list.as_sequence().ok_or_else(|| {
                anyhow!(
                    "include must be a YAML sequence of relative file paths: {}",
                    canonical.display()
                )
            })?;
            let base_dir = canonical.parent().unwrap_or(Path::new("."));
            for include_item in includes {
                let include_rel = include_item.as_str().ok_or_else(|| {
                    anyhow!("include entries must be strings in {}", canonical.display())
                })?;
                let include_path = Path::new(include_rel);
                if include_path.is_absolute()
                    || include_path
                        .components()
                        .any(|c| matches!(c, Component::ParentDir))
                {
                    return Err(anyhow!(
                        "include path must be relative and must not contain '..': '{}' in {}",
                        include_rel,
                        canonical.display()
                    ));
                }
                let include_file = base_dir.join(include_path);
                let include_doc = load_yaml_with_includes(&include_file, stack)?;
                let include_map = include_doc.as_mapping().ok_or_else(|| {
                    anyhow!(
                        "included ADL document must be a mapping: {}",
                        include_file.display()
                    )
                })?;
                merge_top_level_map(&mut merged, include_map, &include_file)?;
            }
        }

        merge_top_level_map(&mut merged, map, &canonical)?;
        Ok(YamlValue::Mapping(merged))
    })();
    let _ = stack.pop();
    result
}

fn merge_top_level_map(dst: &mut YamlMapping, src: &YamlMapping, src_path: &Path) -> Result<()> {
    for (key, value) in src {
        let Some(key_name) = key.as_str() else {
            return Err(anyhow!(
                "top-level ADL keys must be strings in {}",
                src_path.display()
            ));
        };

        match key_name {
            "providers" | "tools" | "agents" | "tasks" | "workflows" => {
                let src_map = value.as_mapping().ok_or_else(|| {
                    anyhow!(
                        "top-level '{}' must be a mapping in {}",
                        key_name,
                        src_path.display()
                    )
                })?;
                let dst_entry = dst
                    .entry(key.clone())
                    .or_insert_with(|| YamlValue::Mapping(YamlMapping::new()));
                let dst_map = dst_entry.as_mapping_mut().ok_or_else(|| {
                    anyhow!("top-level '{}' merge target is not a mapping", key_name)
                })?;
                for (item_key, item_val) in src_map {
                    if dst_map.contains_key(item_key) {
                        let item_id = item_key
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "<non-string>".to_string());
                        return Err(anyhow!(
                            "duplicate {} id '{}' while processing includes (source: {})",
                            key_name,
                            item_id,
                            src_path.display()
                        ));
                    }
                    dst_map.insert(item_key.clone(), item_val.clone());
                }
            }
            "patterns" => {
                let src_seq = value.as_sequence().ok_or_else(|| {
                    anyhow!(
                        "top-level '{}' must be a sequence in {}",
                        key_name,
                        src_path.display()
                    )
                })?;
                let dst_entry = dst
                    .entry(key.clone())
                    .or_insert_with(|| YamlValue::Sequence(Vec::new()));
                let dst_seq = dst_entry.as_sequence_mut().ok_or_else(|| {
                    anyhow!("top-level '{}' merge target is not a sequence", key_name)
                })?;
                dst_seq.extend(src_seq.iter().cloned());
            }
            _ => {
                if dst.contains_key(key) {
                    return Err(anyhow!(
                        "duplicate top-level key '{}' while processing includes (source: {})",
                        key_name,
                        src_path.display()
                    ));
                }
                dst.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}
