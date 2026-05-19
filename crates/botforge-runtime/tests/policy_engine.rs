use botforge_runtime::{evaluate_policy, PolicyDecision};
use botforge_spec::{PolicyRule, PolicyScope, RuleEffect};

fn rule(id: &str, scope: PolicyScope, effect: RuleEffect, priority: u16, profile: &str, staged: bool) -> PolicyRule {
    PolicyRule {
        id: id.to_string(),
        scope,
        effect,
        priority,
        profile_id: profile.to_string(),
        staged,
    }
}

#[test]
fn user_scope_overrides_global_scope() {
    let rules = vec![
        rule("global-allow", PolicyScope::Global, RuleEffect::Allow, 100, "default", false),
        rule("user-deny", PolicyScope::User, RuleEffect::Deny, 1, "default", false),
    ];

    let result = evaluate_policy(&rules, false);
    assert_eq!(result.decision, PolicyDecision::Deny);
    assert_eq!(result.winning_rule_id, "user-deny");
}

#[test]
fn deny_beats_allow_at_same_scope() {
    let rules = vec![
        rule("guild-allow", PolicyScope::Guild, RuleEffect::Allow, 999, "alpha", false),
        rule("guild-deny", PolicyScope::Guild, RuleEffect::Deny, 1, "beta", false),
    ];

    let result = evaluate_policy(&rules, false);
    assert_eq!(result.decision, PolicyDecision::Deny);
    assert_eq!(result.winning_rule_id, "guild-deny");
}

#[test]
fn profile_tie_break_is_stable() {
    let rules = vec![
        rule("r-2", PolicyScope::Feature, RuleEffect::Allow, 10, "profile-b", false),
        rule("r-1", PolicyScope::Feature, RuleEffect::Allow, 10, "profile-a", false),
    ];

    let result = evaluate_policy(&rules, false);
    assert_eq!(result.winning_profile_id, "profile-a");
    assert_eq!(result.winning_rule_id, "r-1");
}

#[test]
fn staged_rules_are_excluded_unless_requested() {
    let rules = vec![
        rule("active-deny", PolicyScope::Bot, RuleEffect::Deny, 5, "prod", false),
        rule("staged-allow", PolicyScope::User, RuleEffect::Allow, 50, "staging", true),
    ];

    let without_staged = evaluate_policy(&rules, false);
    assert_eq!(without_staged.winning_rule_id, "active-deny");

    let with_staged = evaluate_policy(&rules, true);
    assert_eq!(with_staged.winning_rule_id, "staged-allow");
}

#[test]
fn audit_record_includes_exact_rule_path() {
    let rules = vec![rule(
        "channel-deny",
        PolicyScope::Channel,
        RuleEffect::Deny,
        77,
        "profile-main",
        false,
    )];

    let result = evaluate_policy(&rules, false);
    assert_eq!(
        result.rule_path,
        "scope=channel > effect=deny > priority=77 > profile=profile-main > rule=channel-deny"
    );
}
