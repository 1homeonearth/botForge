use botforge_spec::{validate_platform_name, BotManifest};

#[test]
fn parses_valid_bot_manifest() {
    let raw = include_str!("../examples/bot-manifest.json");
    let parsed: BotManifest = serde_json::from_str(raw).unwrap();
    assert_eq!(parsed.bot.name, "squire");
    assert!(validate_platform_name("discord"));
}

#[test]
fn rejects_missing_required_fields() {
    let raw = r#"{"bot":{"name":"x"}}"#;
    let parsed = serde_json::from_str::<BotManifest>(raw);
    assert!(parsed.is_err());
}

#[test]
fn rejects_unknown_platform() {
    assert!(!validate_platform_name("myspace"));
}
