#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyScope {
    Global,
    Platform,
    Bot,
    Feature,
    Guild,
    Channel,
    Role,
    User,
}

impl PolicyScope {
    pub const fn precedence_rank(self) -> u8 {
        match self {
            PolicyScope::Global => 0,
            PolicyScope::Platform => 1,
            PolicyScope::Bot => 2,
            PolicyScope::Feature => 3,
            PolicyScope::Guild => 4,
            PolicyScope::Channel => 5,
            PolicyScope::Role => 6,
            PolicyScope::User => 7,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            PolicyScope::Global => "global",
            PolicyScope::Platform => "platform",
            PolicyScope::Bot => "bot",
            PolicyScope::Feature => "feature",
            PolicyScope::Guild => "guild",
            PolicyScope::Channel => "channel",
            PolicyScope::Role => "role",
            PolicyScope::User => "user",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    pub id: String,
    pub scope: PolicyScope,
    pub effect: RuleEffect,
    pub priority: u16,
    pub profile_id: String,
    pub staged: bool,
}
