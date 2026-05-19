use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const SPEC_VERSION: &str = "botforge.spec.v1";
pub const SUPPORTED_WASM_TARGETS: &[&str] = &["wasm32-wasi", "wasm32-unknown-unknown"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Envelope {
    pub spec: String,
    pub id: String,
    pub r#type: String,
    pub timestamp: String,
    pub source: String,
    pub from: Option<String>,
    pub target: Option<String>,
    pub actor: String,
    pub payload: serde_json::Value,
    pub context: serde_json::Value,
    pub correlation_id: Option<String>,
    pub request_id: Option<String>,
    pub capabilities_used: Vec<String>,
}

pub type EventEnvelope = Envelope;
pub type IntentEnvelope = Envelope;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityGrant {
    pub capability: String,
    pub scope: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamedVersion {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Build {
    pub language: String,
    pub target: String,
    pub crate_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Security {
    pub signatures_required: bool,
    pub hash_required: bool,
    pub forbidden_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Platforms {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupSchema {
    pub fields: Vec<SetupField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupField {
    pub key: String,
    pub field_type: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotManifest {
    pub bot: NamedVersion,
    pub build: Build,
    pub security: Security,
    pub platforms: Platforms,
    pub capabilities: Vec<String>,
    pub events: Vec<String>,
    pub intents: Vec<String>,
    pub setup: SetupSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureManifest {
    pub feature: NamedVersion,
    pub owner_bot: String,
    pub required_platforms: Vec<String>,
    pub optional_platforms: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub subscribed_events: Vec<String>,
    pub emitted_intents: Vec<String>,
    pub setup: SetupSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformManifest {
    pub platform: NamedVersion,
    pub build: Build,
    pub security: Security,
    pub allowed_secrets: Vec<String>,
    pub provided_capabilities: Vec<String>,
    pub emitted_events: Vec<String>,
    pub executed_intents: Vec<String>,
    pub setup: SetupSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyManifest {
    pub policy: NamedVersion,
    pub capabilities: Vec<CapabilityGrant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    MissingField(&'static str),
    InvalidVersion,
    MissingPlatform(String),
    UnknownCapability(String),
    ForbiddenPermission(String),
    UnsupportedBuildTarget(String),
    InvalidSetup(String),
    HashMismatchPlaceholder,
    InvalidSignaturePlaceholder,
}

pub fn validate_platform_name(name: &str) -> bool {
    matches!(
        name,
        "discord"
            | "reddit"
            | "spotify"
            | "youtube"
            | "rss"
            | "webhook"
            | "telegram"
            | "matrix"
            | "mattermost"
            | "session"
            | "email"
    )
}

pub fn known_capability(capability: &str) -> bool {
    capability.starts_with("discord.")
        || capability.starts_with("reddit.")
        || capability.starts_with("spotify.")
        || capability.starts_with("youtube.")
        || capability.starts_with("botforge.")
        || capability.starts_with("sentry.")
}

fn version_ok(v: &str) -> bool {
    let mut it = v.split('.');
    it.next().and_then(|x| x.parse::<u64>().ok()).is_some()
        && it.next().and_then(|x| x.parse::<u64>().ok()).is_some()
        && it.next().and_then(|x| x.parse::<u64>().ok()).is_some()
}

pub fn validate_bot_manifest(m: &BotManifest, installed_platforms: &[&str]) -> Result<(), ValidationError> {
    if m.bot.name.is_empty() {
        return Err(ValidationError::MissingField("bot.name"));
    }
    if !version_ok(&m.bot.version) { return Err(ValidationError::InvalidVersion); }
    if !SUPPORTED_WASM_TARGETS.contains(&m.build.target.as_str()) {
        return Err(ValidationError::UnsupportedBuildTarget(m.build.target.clone()));
    }
    let installed: BTreeSet<&str> = installed_platforms.iter().copied().collect();
    for p in &m.platforms.required {
        if !validate_platform_name(p) || !installed.contains(p.as_str()) {
            return Err(ValidationError::MissingPlatform(p.clone()));
        }
    }
    for cap in &m.capabilities {
        if !known_capability(cap) {
            return Err(ValidationError::UnknownCapability(cap.clone()));
        }
    }
    for perm in &m.security.forbidden_permissions {
        if ["network", "filesystem", "shell", "env", "platform_sdk_live_calls"].contains(&perm.as_str()) {
            return Err(ValidationError::ForbiddenPermission(perm.clone()));
        }
    }
    if m.setup.fields.iter().any(|f| f.key.is_empty() || f.field_type.is_empty()) {
        return Err(ValidationError::InvalidSetup("empty setup field".into()));
    }
    Ok(())
}
