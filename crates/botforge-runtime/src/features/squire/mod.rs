use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AutobanDecider {
    pub state: HashMap<String, u32>,
    pub threshold: u32,
    pub audit_log: Vec<String>,
}

impl AutobanDecider {
    pub fn new(threshold: u32) -> Self {
        Self { state: HashMap::new(), threshold, audit_log: vec![] }
    }

    pub fn record_violation(&mut self, user_id: &str, reason: &str) -> Option<String> {
        let count = self.state.entry(user_id.to_string()).and_modify(|v| *v += 1).or_insert(1);
        self.audit_log.push(format!("Violation for user={user_id}: {reason} (count={count})"));
        if *count >= self.threshold {
            Some(format!("Ban recommended for user {user_id}: threshold {} reached", self.threshold))
        } else {
            None
        }
    }

    pub fn reset_user(&mut self, user_id: &str) {
        self.state.remove(user_id);
        self.audit_log.push(format!("Reset violations for user={user_id}"));
    }
    pub fn export_state(&self) -> HashMap<String, u32> { self.state.clone() }
    pub fn import_state(&mut self, snapshot: HashMap<String, u32>) { self.state = snapshot; }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbedTemplate {
    pub title: String,
    pub description: String,
    pub color: u32,
    pub fields: Vec<EmbedField>,
    pub footer: String,
}

impl Default for EmbedTemplate {
    fn default() -> Self {
        Self { title: String::new(), description: String::new(), color: 0x5865F2, fields: vec![], footer: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct EmbedBuilder {
    pub template: EmbedTemplate,
    pub history: Vec<EmbedTemplate>,
    pub persistence_path: Option<PathBuf>,
}

impl EmbedBuilder {
    pub fn new(persistence_path: Option<PathBuf>) -> Self {
        Self { template: EmbedTemplate::default(), history: vec![], persistence_path }
    }
    pub fn set_title(&mut self, title: &str) { self.template.title = title.into(); }
    pub fn set_description(&mut self, description: &str) { self.template.description = description.into(); }
    pub fn set_color(&mut self, color: u32) { self.template.color = color & 0x00FF_FFFF; }
    pub fn add_field(&mut self, name: &str, value: &str, inline: bool) {
        self.template.fields.push(EmbedField { name: name.into(), value: value.into(), inline });
    }
    pub fn set_footer(&mut self, footer: &str) { self.template.footer = footer.into(); }

    pub fn build(&mut self) -> EmbedTemplate {
        let built = self.template.clone();
        self.history.push(built.clone());
        built
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(path) = &self.persistence_path {
            let serialized = serde_json::to_string_pretty(&self.template).expect("embed serialization cannot fail");
            fs::write(path, serialized)?;
        }
        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.persistence_path {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                self.template = serde_json::from_str(&content).unwrap_or_default();
            } else {
                self.template = EmbedTemplate::default();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExperienceTracker {
    pub xp: HashMap<String, u64>,
    pub level_scale: u64,
    pub persistence_path: Option<PathBuf>,
}

impl ExperienceTracker {
    pub fn new(level_scale: u64, persistence_path: Option<PathBuf>) -> Self {
        Self { xp: HashMap::new(), level_scale, persistence_path }
    }
    pub fn award_xp(&mut self, user_id: &str, amount: i64) -> UserProgress {
        let amount = amount.max(0) as u64;
        let total = self.xp.entry(user_id.into()).and_modify(|v| *v += amount).or_insert(amount);
        UserProgress { xp: *total, level: (*total / self.level_scale) + 1 }
    }
    pub fn summary(&self, user_id: &str) -> UserProgress {
        let xp = *self.xp.get(user_id).unwrap_or(&0);
        UserProgress { xp, level: (xp / self.level_scale) + 1 }
    }
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(path) = &self.persistence_path {
            let mut obj = Map::new();
            obj.insert("xp".into(), serde_json::to_value(&self.xp).expect("serialize xp"));
            obj.insert("level_scale".into(), Value::from(self.level_scale));
            fs::write(path, serde_json::to_string_pretty(&Value::Object(obj)).expect("serialize snapshot"))?;
        }
        Ok(())
    }
    pub fn load(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.persistence_path {
            if !path.exists() { return Ok(()); }
            let content = fs::read_to_string(path)?;
            let parsed: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
            if let Some(scale) = parsed.get("level_scale").and_then(|v| v.as_u64()) { self.level_scale = scale.max(1); }
            if let Some(xp) = parsed.get("xp") {
                self.xp = serde_json::from_value(xp.clone()).unwrap_or_default();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserProgress {
    pub xp: u64,
    pub level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModerationActionRecord {
    pub action: String,
    pub user_id: String,
    pub reason: String,
    pub duration_minutes: Option<u64>,
    pub delete_message_days: Option<u64>,
}

fn action_record(action: &str, user_id: &str, reason: &str) -> ModerationActionRecord {
    ModerationActionRecord { action: action.to_string(), user_id: user_id.to_string(), reason: reason.to_string(), duration_minutes: None, delete_message_days: None }
}

pub fn warn(user_id: &str, reason: &str) -> ModerationActionRecord { action_record("warn", user_id, reason) }
pub fn unmute(user_id: &str, reason: &str) -> ModerationActionRecord { action_record("unmute", user_id, reason) }
pub fn kick(user_id: &str, reason: &str) -> ModerationActionRecord { action_record("kick", user_id, reason) }
pub fn mute(user_id: &str, reason: &str, duration_minutes: u64) -> ModerationActionRecord { let mut record = action_record("mute", user_id, reason); record.duration_minutes = Some(duration_minutes); record }
pub fn ban(user_id: &str, reason: &str, delete_message_days: u64) -> ModerationActionRecord { let mut record = action_record("ban", user_id, reason); record.delete_message_days = Some(delete_message_days); record }

pub fn summary(actions: &[ModerationActionRecord]) -> String {
    actions.iter().map(|a| {
        let mut line = format!("{} user={} reason={}", a.action, a.user_id, a.reason);
        if let Some(minutes) = a.duration_minutes { line.push_str(&format!(" duration_minutes={minutes}")); }
        if let Some(days) = a.delete_message_days { line.push_str(&format!(" delete_message_days={days}")); }
        line
    }).collect::<Vec<_>>().join("\n")
}

pub fn validate_channel_id(channel_id: &str) -> bool { !channel_id.is_empty() && channel_id.chars().all(|c| c.is_ascii_digit()) }

pub fn prepare_setup_summary(features: &[String], channels: &HashMap<String, String>) -> String {
    features.iter().map(|feature| {
        if let Some(channel) = channels.get(feature) {
            format!("feature={feature} channel={channel}")
        } else {
            format!("feature={feature} channel=<missing>")
        }
    }).collect::<Vec<_>>().join("\n")
}

pub fn interactive_mock(features: &[String]) -> String {
    let mut channels = HashMap::new();
    for (idx, feature) in features.iter().enumerate() {
        channels.insert(feature.clone(), (10_000 + idx as u32).to_string());
    }
    prepare_setup_summary(features, &channels)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelayRecord {
    pub timestamp_utc: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct RainbowBridge {
    pub bridges: Vec<(String, String, String)>,
    pub ledger: Vec<RelayRecord>,
}

impl RainbowBridge {
    pub fn add_bridge(&mut self, source: &str, target: &str, label: &str) {
        self.bridges.push((source.into(), target.into(), label.into()));
    }

    pub fn remove_bridge(&mut self, label: &str) -> bool {
        let before = self.bridges.len();
        self.bridges.retain(|(_, _, l)| l != label);
        before != self.bridges.len()
    }

    pub fn relay_message(&mut self, channel: &str, author: &str, content: &str) -> Vec<RelayRecord> {
        let timestamp_utc = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let reports = self
            .bridges
            .iter()
            .filter(|(source, _, _)| source == channel)
            .map(|(source, target, label)| RelayRecord {
                timestamp_utc: timestamp_utc.clone(),
                source: source.clone(),
                target: target.clone(),
                label: label.clone(),
                author: author.into(),
                content: content.into(),
            })
            .collect::<Vec<_>>();
        self.ledger.extend(reports.clone());
        reports
    }

    pub fn last_relays(&self, limit: Option<usize>) -> Vec<RelayRecord> {
        match limit {
            None => self.ledger.clone(),
            Some(n) => self.ledger.iter().rev().take(n).cloned().collect::<Vec<_>>().into_iter().rev().collect(),
        }
    }
}

pub fn demo_bridge_flow() -> Vec<RelayRecord> {
    let mut default_bridge = RainbowBridge::default();
    default_bridge.add_bridge("general", "ops", "default");
    default_bridge.relay_message("general", "demo", "hello from demo")
}

pub fn path_exists(path: &Path) -> bool { path.exists() }
