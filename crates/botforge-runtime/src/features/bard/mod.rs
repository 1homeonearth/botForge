use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

fn bot_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(|p| p.parent()).unwrap_or(Path::new(".")).to_path_buf()
}

fn ensure_parent(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    Ok(())
}

fn append_line(path: &Path, line: &str) -> std::io::Result<()> {
    ensure_parent(path)?;
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{line}")?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueuePayload {
    pub kind: String,
    pub route_server_id: String,
    pub route_channel_id: String,
    pub body: String,
}

pub struct LoggingForwarder {
    pub event_log: PathBuf,
}

impl Default for LoggingForwarder {
    fn default() -> Self {
        let root = bot_root();
        Self { event_log: root.join("logs/logging_forwarder.log") }
    }
}

impl LoggingForwarder {
    pub fn record_server_event(&self, server_id: &str, channel_id: &str, message: &str) -> std::io::Result<QueuePayload> {
        let line = format!("server={server_id} channel={channel_id} message={message}");
        append_line(&self.event_log, &line)?;
        Ok(QueuePayload { kind: "log".into(), route_server_id: server_id.into(), route_channel_id: channel_id.into(), body: message.into() })
    }

    pub fn summarize_queue(&self) -> String {
        "dispatch through runtime intents".to_string()
    }
}

pub struct ModerationLogging {
    pub mod_log: PathBuf,
}

impl Default for ModerationLogging {
    fn default() -> Self {
        let root = bot_root();
        Self { mod_log: root.join("logs/moderation.log") }
    }
}

impl ModerationLogging {
    pub fn log_action(&self, action: &str, moderator: &str, subject: &str, reason: &str) -> std::io::Result<Value> {
        let payload = json!({
            "kind": "moderation_log",
            "action": action,
            "moderator": moderator,
            "subject": subject,
            "reason": reason,
        });
        append_line(&self.mod_log, &payload.to_string())?;
        Ok(payload)
    }
}

pub struct Starboard {
    pub starboard_log: PathBuf,
}

impl Default for Starboard {
    fn default() -> Self {
        let root = bot_root();
        Self { starboard_log: root.join("logs/starboard.log") }
    }
}

impl Starboard {
    pub fn record_reaction(&self, message_id: &str, author: &str, content: &str, reactors: &[String], threshold: usize) -> std::io::Result<Option<Value>> {
        let reaction_count = reactors.len();
        let trace = format!("message_id={message_id} reactions={reaction_count} threshold={threshold}");
        append_line(&self.starboard_log, &trace)?;

        if reaction_count < threshold {
            append_line(&self.starboard_log, "below threshold")?;
            return Ok(None);
        }

        let payload = json!({
            "kind": "starboard",
            "message_id": message_id.to_string(),
            "author": author.to_string(),
            "content": content.to_string(),
            "reactors": reactors.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
            "reaction_count": reaction_count.to_string(),
        });
        append_line(&self.starboard_log, "threshold reached")?;
        Ok(Some(payload))
    }
}

pub struct WelcomeCard {
    pub template_path: PathBuf,
}

impl Default for WelcomeCard {
    fn default() -> Self {
        let root = bot_root();
        Self { template_path: root.join("data/welcome_template.txt") }
    }
}

impl WelcomeCard {
    pub fn build_welcome_card(&self, member_name: &str, server_name: &str, extra_note: Option<&str>) -> Value {
        let mut body = format!("Welcome, {member_name}! You joined {server_name}.");
        if self.template_path.exists() {
            if let Ok(template) = fs::read_to_string(&self.template_path) {
                body.push_str("\n\n");
                body.push_str(template.trim());
            }
        }
        if let Some(note) = extra_note {
            body.push_str("\n\n");
            body.push_str(note);
        }

        json!({"kind":"welcome_card","title":format!("Welcome {member_name}"),"body":body,"member_name":member_name,"server_name":server_name})
    }
}
