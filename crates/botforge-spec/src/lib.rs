use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Envelope {
    pub spec: String,
    pub id: String,
    pub r#type: String,
    pub timestamp: String,
    pub source: String,
    pub target: Option<String>,
    pub actor: String,
    pub payload: serde_json::Value,
    pub context: serde_json::Value,
    pub correlation_id: Option<String>,
    pub request_id: Option<String>,
    pub capabilities_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityGrant {
    pub capability: String,
    pub scope: String,
    pub effect: String,
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
    pub setup: serde_json::Value,
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
    pub setup: serde_json::Value,
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
    pub setup: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamedVersion { pub name: String, pub version: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Build { pub target: String, pub crate_path: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Security { pub signatures_required: bool, pub hash_required: bool }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Platforms { pub required: Vec<String>, pub optional: Vec<String> }

pub fn validate_platform_name(name: &str) -> bool {
    matches!(name, "discord"|"reddit"|"spotify"|"youtube"|"rss"|"webhook"|"telegram"|"matrix"|"mattermost"|"session"|"email")
}
