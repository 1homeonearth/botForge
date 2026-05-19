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
