use botforge_runtime::{Court, Gate, Intent, PlatformError, PlatformManifest, PlatformModule, PlatformRegistry, RouteDecision, RuntimeAudit};
use botforge_spec::{Actor, CapabilityGrant, EventEnvelope, EventSource, EventTarget, IntentEnvelope, SPEC_VERSION};

fn source() -> EventSource { EventSource { platform: "discord".into(), guild: Some("g1".into()), channel: Some("c1".into()) } }
fn target() -> EventTarget { EventTarget { bot: "squire".into(), feature: Some("mod".into()) } }
fn intent(cap: &str) -> Intent { Intent(IntentEnvelope { spec: SPEC_VERSION.into(), intent_id: "i1".into(), r#type: "discord.message.send".into(), timestamp: "2026-05-19T00:00:00Z".into(), from: target(), requested_by_event: Some("e1".into()), capabilities_used: vec![cap.into()], payload: serde_json::json!({"content":"ok"}) }) }

#[test]
fn unknown_capability_denied() {
    let grants = vec![];
    let decision = Court::validate_intent(&intent("discord.message.send"), &grants, &source());
    assert_eq!(decision, RouteDecision::Denied("capability denied: discord.message.send".into()));
}

#[test]
fn scope_mismatch_denied() {
    let grants = vec![CapabilityGrant { capability: "discord.message.send".into(), effect: "allow".into(), bot: None, feature: None, guild: Some("other".into()), channel: None, role: None, user: None, platform_module: Some("discord".into()), environment: None, time_window: None, rate_limit_per_minute: None, setup_profile: None }];
    let decision = Court::validate_intent(&intent("discord.message.send"), &grants, &source());
    assert!(matches!(decision, RouteDecision::Denied(_)));
}

#[test]
fn valid_intent_routed_and_audited() {
    let grants = vec![CapabilityGrant { capability: "discord.message.send".into(), effect: "allow".into(), bot: None, feature: None, guild: Some("g1".into()), channel: Some("c1".into()), role: None, user: None, platform_module: Some("discord".into()), environment: None, time_window: None, rate_limit_per_minute: None, setup_profile: None }];
    let mut audit = RuntimeAudit { records: vec![] };
    let routed = Court::route_intent(&intent("discord.message.send"), &grants, &source(), &mut audit).unwrap();
    assert_eq!(routed.r#type, "court.discord.message.send");
    assert!(audit.records[0].starts_with("route:i1"));
}

#[test]
fn raw_platform_event_sanitized_by_gate() {
    let gate = Gate;
    let raw = serde_json::to_string(&EventEnvelope { spec: "bad".into(), event_id: "e1".into(), r#type: "discord.message.created".into(), timestamp: "2026-05-19T00:00:00Z".into(), source: source(), target: target(), actor: Actor { actor_type: "user".into(), actor_id: "u1".into() }, payload: serde_json::json!({}), context: serde_json::json!({}), correlation_id: None }).unwrap();
    let normalized = gate.normalize_platform_event(&raw, "discord", target()).unwrap();
    assert_eq!(normalized.0.spec, SPEC_VERSION);
}

struct MockPlatform { started: bool }
impl PlatformModule for MockPlatform {
    fn manifest(&self) -> PlatformManifest { PlatformManifest { runtime_id: "discord".into(), capabilities: vec!["discord.message.send".into()], required_secrets: vec!["secret.discord.token".into()], allow_raw_network: true } }
    fn setup_schema(&self) -> String { "setup".into() }
    fn start_listener(&mut self) -> Result<(), PlatformError> { self.started = true; Ok(()) }
    fn stop_listener(&mut self) -> Result<(), PlatformError> { Ok(()) }
    fn execute_intent(&self, _intent: &Intent) -> Result<serde_json::Value, PlatformError> { Ok(serde_json::json!({"ok":true})) }
    fn normalize_event(&self, _raw: serde_json::Value) -> Result<EventEnvelope, PlatformError> { Err(PlatformError::ExecuteFailed("not used".into())) }
}

#[test]
fn inter_bot_mediation_only_through_court_and_registry_activation() {
    let mut registry = PlatformRegistry::default();
    registry.register(Box::new(MockPlatform { started: false })).unwrap();
    assert!(registry.activate("discord", &["secret.discord.token"]).is_ok());
    let mut registry2 = PlatformRegistry::default();
    registry2.register(Box::new(MockPlatform { started: false })).unwrap();
    let err = registry2.activate("discord", &[]).unwrap_err();
    assert!(matches!(err, PlatformError::MissingSecret(_)));
}
