use botforge_runtime::{Chamber, Court, Gate, Intent};
use botforge_spec::Envelope;

fn intent_with_caps() -> Intent {
    Intent(Envelope {
        spec: "botforge.spec.v1".into(), id: "1".into(), r#type: "discord.message.send".into(),
        timestamp: "2026-05-19T00:00:00Z".into(), source: "bot.squire".into(), target: Some("discord".into()),
        actor: "bot".into(), payload: serde_json::json!({}), context: serde_json::json!({}),
        correlation_id: None, request_id: None, capabilities_used: vec!["discord.message.send".into()]
    })
}

#[test]
fn raw_platform_event_cannot_enter_chamber() {
    let chamber = Chamber;
    assert!(!chamber.reject_raw_platform_event("{raw:true}"));
}

#[test]
fn intent_must_pass_court_before_gate_execution() {
    let gate = Gate;
    let court = Court;
    let intent = intent_with_caps();
    let approved = court.validate_intent(&intent);
    assert!(gate.execute_intent(&intent, approved));
}
