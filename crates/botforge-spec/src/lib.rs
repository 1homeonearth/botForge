use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const SPEC_VERSION: &str = "botforge.spec.v1";
pub const SUPPORTED_WASM_TARGETS: &[&str] = &["wasm32-wasi", "wasm32-unknown-unknown"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventEnvelope {
    pub spec: String,
    pub event_id: String,
    pub r#type: String,
    pub timestamp: String,
    pub source: EventSource,
    pub target: EventTarget,
    pub actor: Actor,
    pub payload: serde_json::Value,
    pub context: serde_json::Value,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentEnvelope {
    pub spec: String,
    pub intent_id: String,
    pub r#type: String,
    pub timestamp: String,
    pub from: EventTarget,
    pub requested_by_event: Option<String>,
    pub capabilities_used: Vec<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventSource {
    pub platform: String,
    pub guild: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventTarget {
    pub bot: String,
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Actor {
    pub actor_type: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityGrant {
    pub capability: String,
    pub effect: String,
    pub bot: Option<String>,
    pub feature: Option<String>,
    pub guild: Option<String>,
    pub channel: Option<String>,
    pub role: Option<String>,
    pub user: Option<String>,
    pub platform_module: Option<String>,
    pub environment: Option<String>,
    pub time_window: Option<String>,
    pub rate_limit_per_minute: Option<u32>,
    pub setup_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamedVersion { pub name: String, pub version: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Build { pub language: String, pub target: String, pub crate_path: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Security { pub signatures_required: bool, pub hash_required: bool, pub forbidden_permissions: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Platforms { pub required: Vec<String>, pub optional: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupSchema { pub fields: Vec<SetupField> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupField { pub key: String, pub field_type: String, pub required: bool }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotManifest { pub bot: NamedVersion, pub build: Build, pub security: Security, pub platforms: Platforms, pub capabilities: Vec<String>, pub events: Vec<String>, pub intents: Vec<String>, pub setup: SetupSchema }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError { MissingField(&'static str), InvalidVersion, MissingPlatform(String), UnknownCapability(String), ForbiddenPermission(String), UnsupportedBuildTarget(String), InvalidSetup(String), HashMismatchPlaceholder, InvalidSignaturePlaceholder }

pub fn validate_platform_name(name: &str) -> bool {
    matches!(name,"discord"|"reddit"|"spotify"|"youtube"|"rss"|"webhook"|"telegram"|"matrix"|"mattermost"|"session"|"email")
}

pub fn known_capability(capability: &str) -> bool {
    const EXACT: &[&str] = &[
        "discord.message.send","discord.message.edit","discord.message.delete","discord.embed.send","discord.thread.create","discord.channel.create","discord.channel.edit","discord.channel.delete","discord.role.create","discord.role.edit","discord.role.assign","discord.role.remove","discord.member.timeout","discord.member.kick","discord.member.ban","discord.member.unban","discord.auditlog.read","discord.interaction.respond","discord.command.register","discord.voice.observe","discord.webhook.manage",
        "reddit.post.submit","reddit.post.status.read","reddit.subreddit.flair.read","reddit.subreddit.rules.read","reddit.comment.submit","reddit.account.identity.use","reddit.user.submitted.read",
        "spotify.track.search","spotify.playlist.read","spotify.playlist.write",
        "youtube.video.search","youtube.video.resolve","youtube.playlist.read","youtube.playlist.write",
        "botforge.state.read","botforge.state.write","botforge.config.read","botforge.audit.write","botforge.metric.emit","botforge.job.schedule","botforge.setup.read","botforge.update.stage",
        "sentry.case.open","sentry.evidence.attach","sentry.verification.run","sentry.moderation.execute","sentry.lockdown.activate"
    ];
    EXACT.contains(&capability)
}

fn version_ok(v: &str) -> bool { let mut it=v.split('.'); it.next().and_then(|x|x.parse::<u64>().ok()).is_some()&&it.next().and_then(|x|x.parse::<u64>().ok()).is_some()&&it.next().and_then(|x|x.parse::<u64>().ok()).is_some() }

pub fn validate_bot_manifest(m:&BotManifest,installed_platforms:&[&str])->Result<(),ValidationError>{
    if m.bot.name.is_empty(){return Err(ValidationError::MissingField("bot.name"));}
    if !version_ok(&m.bot.version){return Err(ValidationError::InvalidVersion);}
    if !SUPPORTED_WASM_TARGETS.contains(&m.build.target.as_str()){return Err(ValidationError::UnsupportedBuildTarget(m.build.target.clone()));}
    let installed:BTreeSet<&str>=installed_platforms.iter().copied().collect();
    for p in &m.platforms.required{if !validate_platform_name(p)||!installed.contains(p.as_str()){return Err(ValidationError::MissingPlatform(p.clone()));}}
    for cap in &m.capabilities{if !known_capability(cap){return Err(ValidationError::UnknownCapability(cap.clone()));}}
    for perm in &m.security.forbidden_permissions{if ["network","filesystem","shell","env","platform_sdk_live_calls"].contains(&perm.as_str()){return Err(ValidationError::ForbiddenPermission(perm.clone()));}}
    if m.setup.fields.iter().any(|f|f.key.is_empty()||f.field_type.is_empty()){return Err(ValidationError::InvalidSetup("empty setup field".into()));}
    Ok(())
}
