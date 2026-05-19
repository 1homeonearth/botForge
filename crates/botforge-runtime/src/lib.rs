use std::collections::HashSet;

use botforge_spec::{PolicyRule, PolicyScope, RuleEffect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionAudit {
    pub decision: PolicyDecision,
    pub rule_path: String,
    pub winning_rule_id: String,
    pub winning_profile_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostCall {
    LogWrite,
    StateGet,
    StateSet,
    StateDelete,
    EmitIntent,
    EventAck,
    MetricsEmit,
}

impl HostCall {
    pub fn required_capability(self) -> &'static str {
        match self {
            HostCall::LogWrite => "runtime.log.write",
            HostCall::StateGet => "state.read",
            HostCall::StateSet => "state.write",
            HostCall::StateDelete => "state.write",
            HostCall::EmitIntent => "intent.emit",
            HostCall::EventAck => "event.ack",
            HostCall::MetricsEmit => "metrics.emit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBudget {
    pub max_host_calls: u32,
    pub max_denied_calls: u32,
    pub max_state_reads: u32,
    pub max_state_writes: u32,
    pub max_intents_emitted: u32,
    pub max_log_bytes: u32,
    pub max_payload_bytes: u32,
    pub cpu_ms_budget: u64,
    pub memory_bytes_budget: u64,
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HostUsage {
    pub host_calls: u32,
    pub denied_calls: u32,
    pub state_reads: u32,
    pub state_writes: u32,
    pub intents_emitted: u32,
    pub log_bytes: u32,
    pub cpu_ms_used: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationGuard {
    pub budget: HostBudget,
    pub usage: HostUsage,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardError {
    Cancelled,
    TimedOut,
    Backpressure,
    Unauthorized,
    PayloadTooLarge,
    AbuseDetected,
}

impl InvocationGuard {
    pub fn new(budget: HostBudget) -> Self {
        Self {
            budget,
            usage: HostUsage::default(),
            cancelled: false,
        }
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn authorize_and_track_call(
        &mut self,
        call: HostCall,
        granted_capabilities: &HashSet<String>,
        payload_bytes: u32,
        now_ms: u64,
    ) -> Result<(), GuardError> {
        if self.cancelled {
            return Err(GuardError::Cancelled);
        }

        if now_ms > self.budget.deadline_ms || self.usage.cpu_ms_used > self.budget.cpu_ms_budget {
            self.cancelled = true;
            return Err(GuardError::TimedOut);
        }

        if payload_bytes > self.budget.max_payload_bytes {
            self.note_denial();
            return Err(self.denial_result(GuardError::PayloadTooLarge));
        }

        let capability = call.required_capability();
        if !granted_capabilities.contains(capability) {
            self.note_denial();
            return Err(self.denial_result(GuardError::Unauthorized));
        }

        if self.usage.host_calls >= self.budget.max_host_calls {
            return Err(GuardError::Backpressure);
        }

        match call {
            HostCall::StateGet if self.usage.state_reads >= self.budget.max_state_reads => {
                return Err(GuardError::Backpressure)
            }
            HostCall::StateSet | HostCall::StateDelete
                if self.usage.state_writes >= self.budget.max_state_writes =>
            {
                return Err(GuardError::Backpressure)
            }
            HostCall::EmitIntent if self.usage.intents_emitted >= self.budget.max_intents_emitted => {
                return Err(GuardError::Backpressure)
            }
            HostCall::LogWrite if self.usage.log_bytes.saturating_add(payload_bytes) > self.budget.max_log_bytes => {
                return Err(GuardError::Backpressure)
            }
            _ => {}
        }

        self.usage.host_calls += 1;
        match call {
            HostCall::StateGet => self.usage.state_reads += 1,
            HostCall::StateSet | HostCall::StateDelete => self.usage.state_writes += 1,
            HostCall::EmitIntent => self.usage.intents_emitted += 1,
            HostCall::LogWrite => self.usage.log_bytes = self.usage.log_bytes.saturating_add(payload_bytes),
            HostCall::EventAck | HostCall::MetricsEmit => {}
        }

        Ok(())
    }

    fn note_denial(&mut self) {
        self.usage.denied_calls = self.usage.denied_calls.saturating_add(1);
    }

    fn denial_result(&self, default_error: GuardError) -> GuardError {
        if self.usage.denied_calls > self.budget.max_denied_calls {
            GuardError::AbuseDetected
        } else {
            default_error
        }
    }
}

pub fn evaluate_policy(rules: &[PolicyRule], include_staged: bool) -> DecisionAudit {
    let mut candidates: Vec<&PolicyRule> = rules
        .iter()
        .filter(|rule| include_staged || !rule.staged)
        .collect();

    candidates.sort_by(|a, b| {
        b.scope
            .precedence_rank()
            .cmp(&a.scope.precedence_rank())
            .then_with(|| rule_effect_rank(b.effect).cmp(&rule_effect_rank(a.effect)))
            .then_with(|| b.priority.cmp(&a.priority))
            .then_with(|| (!b.staged).cmp(&(!a.staged)))
            .then_with(|| a.profile_id.cmp(&b.profile_id))
            .then_with(|| a.id.cmp(&b.id))
    });

    let winner = candidates.first().copied();

    match winner {
        Some(rule) => {
            let decision = match rule.effect {
                RuleEffect::Allow => PolicyDecision::Allow,
                RuleEffect::Deny => PolicyDecision::Deny,
            };

            DecisionAudit {
                decision,
                rule_path: format!(
                    "scope={} > effect={} > priority={} > profile={} > rule={}",
                    scope_label(rule.scope),
                    effect_label(rule.effect),
                    rule.priority,
                    rule.profile_id,
                    rule.id
                ),
                winning_rule_id: rule.id.clone(),
                winning_profile_id: rule.profile_id.clone(),
            }
        }
        None => DecisionAudit {
            decision: PolicyDecision::Deny,
            rule_path: "default=deny (no applicable rules)".to_string(),
            winning_rule_id: "<none>".to_string(),
            winning_profile_id: "<none>".to_string(),
        },
    }
}

fn rule_effect_rank(effect: RuleEffect) -> u8 {
    match effect {
        RuleEffect::Deny => 1,
        RuleEffect::Allow => 0,
    }
}

fn effect_label(effect: RuleEffect) -> &'static str {
    match effect {
        RuleEffect::Allow => "allow",
        RuleEffect::Deny => "deny",
    }
}

fn scope_label(scope: PolicyScope) -> &'static str {
    scope.as_str()
}
