use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Datelike, FixedOffset, Offset, Timelike, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const IDENTITY_PROFILE_SCHEMA: &str = "identity_profile.v1";
pub const TEMPORAL_CONTEXT_SCHEMA: &str = "temporal_context.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityProfile {
    pub schema_version: String,
    pub agent_id: String,
    pub display_name: String,
    pub birthday_rfc3339: String,
    pub birth_date_local: String,
    pub birth_weekday_local: String,
    pub birth_timezone: String,
    pub created_by: String,
    pub continuity_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalContext {
    pub schema_version: String,
    pub captured_at_rfc3339: String,
    pub local_timestamp_rfc3339: String,
    pub local_date: String,
    pub local_time: String,
    pub local_weekday: String,
    pub timezone: String,
    pub utc_offset: String,
    pub identity_agent_id: Option<String>,
    pub identity_display_name: Option<String>,
    pub age_days_since_birthday: Option<i64>,
}

impl IdentityProfile {
    pub fn from_birthday(
        agent_id: impl Into<String>,
        display_name: impl Into<String>,
        birthday_rfc3339: &str,
        timezone_name: &str,
        created_by: impl Into<String>,
    ) -> Result<Self> {
        let birthday = parse_rfc3339(birthday_rfc3339)?;
        let timezone = parse_timezone(timezone_name)?;
        let local_birthday = birthday.with_timezone(&timezone);

        Ok(Self {
            schema_version: IDENTITY_PROFILE_SCHEMA.to_string(),
            agent_id: normalize_nonempty(agent_id.into(), "agent_id")?,
            display_name: normalize_nonempty(display_name.into(), "display_name")?,
            birthday_rfc3339: birthday.to_rfc3339(),
            birth_date_local: local_birthday.format("%Y-%m-%d").to_string(),
            birth_weekday_local: local_birthday.format("%A").to_string(),
            birth_timezone: timezone.name().to_string(),
            created_by: normalize_nonempty(created_by.into(), "created_by")?,
            continuity_mode: "repo_local_persistent".to_string(),
        })
    }
}

impl TemporalContext {
    pub fn from_now(
        now_utc: DateTime<Utc>,
        timezone_name: &str,
        identity: Option<&IdentityProfile>,
    ) -> Result<Self> {
        let timezone = parse_timezone(timezone_name)?;
        let local_now = now_utc.with_timezone(&timezone);
        let offset = local_now.offset().fix();
        let age_days_since_birthday = match identity {
            Some(profile) => {
                let birthday = parse_rfc3339(&profile.birthday_rfc3339)?;
                let birthday_local = birthday.with_timezone(&timezone);
                Some(
                    local_now
                        .date_naive()
                        .signed_duration_since(birthday_local.date_naive())
                        .num_days(),
                )
            }
            None => None,
        };

        Ok(Self {
            schema_version: TEMPORAL_CONTEXT_SCHEMA.to_string(),
            captured_at_rfc3339: now_utc.to_rfc3339(),
            local_timestamp_rfc3339: local_now.to_rfc3339(),
            local_date: format!(
                "{:04}-{:02}-{:02}",
                local_now.year(),
                local_now.month(),
                local_now.day()
            ),
            local_time: format!(
                "{:02}:{:02}:{:02}",
                local_now.hour(),
                local_now.minute(),
                local_now.second()
            ),
            local_weekday: local_now.format("%A").to_string(),
            timezone: timezone.name().to_string(),
            utc_offset: format_offset(offset),
            identity_agent_id: identity.map(|profile| profile.agent_id.clone()),
            identity_display_name: identity.map(|profile| profile.display_name.clone()),
            age_days_since_birthday,
        })
    }
}

pub fn default_identity_profile_path(repo_root: &Path) -> PathBuf {
    repo_root.join("identity").join("identity_profile.v1.json")
}

pub fn write_identity_profile(path: &Path, profile: &IdentityProfile) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Err(anyhow!(
            "identity profile path must have a parent directory"
        ));
    };
    fs::create_dir_all(parent).with_context(|| {
        format!(
            "failed to create identity profile parent directory {}",
            parent.display()
        )
    })?;
    let bytes = serde_json::to_vec_pretty(profile)?;
    fs::write(path, bytes)
        .with_context(|| format!("failed to write identity profile to {}", path.display()))
}

pub fn load_identity_profile(path: &Path) -> Result<IdentityProfile> {
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read identity profile from {}", path.display()))?;
    let profile: IdentityProfile = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse identity profile {}", path.display()))?;
    if profile.schema_version != IDENTITY_PROFILE_SCHEMA {
        return Err(anyhow!(
            "unsupported identity profile schema version '{}'",
            profile.schema_version
        ));
    }
    Ok(profile)
}

fn parse_rfc3339(value: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("invalid RFC3339 datetime '{}'", value))
}

fn parse_timezone(value: &str) -> Result<Tz> {
    value
        .parse::<Tz>()
        .with_context(|| format!("unsupported timezone '{}'", value))
}

fn normalize_nonempty(value: String, field: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(trimmed.to_string())
}

fn format_offset(offset: FixedOffset) -> String {
    let total = offset.local_minus_utc();
    let sign = if total >= 0 { '+' } else { '-' };
    let absolute = total.abs();
    let hours = absolute / 3600;
    let minutes = (absolute % 3600) / 60;
    format!("{sign}{hours:02}:{minutes:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn identity_profile_derives_local_birth_fields() {
        let profile = IdentityProfile::from_birthday(
            "codex",
            "Codex",
            "2026-03-30T13:34:00-07:00",
            "America/Los_Angeles",
            "daniel",
        )
        .expect("profile");

        assert_eq!(profile.schema_version, IDENTITY_PROFILE_SCHEMA);
        assert_eq!(profile.birth_date_local, "2026-03-30");
        assert_eq!(profile.birth_weekday_local, "Monday");
        assert_eq!(profile.birth_timezone, "America/Los_Angeles");
        assert_eq!(profile.continuity_mode, "repo_local_persistent");
    }

    #[test]
    fn temporal_context_includes_identity_and_age() {
        let profile = IdentityProfile::from_birthday(
            "codex",
            "Codex",
            "2026-03-30T13:34:00-07:00",
            "America/Los_Angeles",
            "daniel",
        )
        .expect("profile");
        let now_utc = Utc.with_ymd_and_hms(2026, 3, 31, 20, 0, 0).unwrap();

        let context = TemporalContext::from_now(now_utc, "America/Los_Angeles", Some(&profile))
            .expect("context");

        assert_eq!(context.schema_version, TEMPORAL_CONTEXT_SCHEMA);
        assert_eq!(context.local_date, "2026-03-31");
        assert_eq!(context.local_weekday, "Tuesday");
        assert_eq!(context.utc_offset, "-07:00");
        assert_eq!(context.identity_agent_id.as_deref(), Some("codex"));
        assert_eq!(context.age_days_since_birthday, Some(1));
    }

    #[test]
    fn default_identity_profile_path_is_repo_relative() {
        let path = default_identity_profile_path(Path::new("/repo"));
        assert_eq!(
            path,
            PathBuf::from("/repo/identity/identity_profile.v1.json")
        );
    }
}
