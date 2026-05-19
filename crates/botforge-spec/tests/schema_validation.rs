use botforge_spec::{
    validate_bot_manifest, validate_platform_name, BotManifest, ValidationError,
};

#[test]
fn parses_valid_bot_manifest() {
    let raw = include_str!("../examples/bot-manifest.json");
    let parsed: BotManifest = serde_json::from_str(raw).unwrap();
    assert_eq!(parsed.bot.name, "squire");
    assert!(validate_platform_name("discord"));
    assert!(validate_bot_manifest(&parsed, &["discord", "reddit"]).is_ok());
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

#[test]
fn rejects_invalid_version() {
    let mut parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    parsed.bot.version = "x.y.z".into();
    assert_eq!(validate_bot_manifest(&parsed, &["discord", "reddit"]), Err(ValidationError::InvalidVersion));
}

#[test]
fn rejects_missing_platform() {
    let parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    assert_eq!(validate_bot_manifest(&parsed, &["reddit"]), Err(ValidationError::MissingPlatform("discord".into())));
}

#[test]
fn rejects_unknown_capability() {
    let mut parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    parsed.capabilities.push("unknown.cap".into());
    assert_eq!(validate_bot_manifest(&parsed, &["discord", "reddit"]), Err(ValidationError::UnknownCapability("unknown.cap".into())));
}

#[test]
fn rejects_forbidden_permission() {
    let mut parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    parsed.security.forbidden_permissions.push("network".into());
    assert_eq!(validate_bot_manifest(&parsed, &["discord", "reddit"]), Err(ValidationError::ForbiddenPermission("network".into())));
}

#[test]
fn rejects_unsupported_build_target() {
    let mut parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    parsed.build.target = "x86_64-unknown-linux-gnu".into();
    assert_eq!(
        validate_bot_manifest(&parsed, &["discord", "reddit"]),
        Err(ValidationError::UnsupportedBuildTarget("x86_64-unknown-linux-gnu".into()))
    );
}

#[test]
fn rejects_invalid_setup() {
    let mut parsed: BotManifest = serde_json::from_str(include_str!("../examples/bot-manifest.json")).unwrap();
    parsed.setup.pages[0].fields[0].field_type = "unknown".into();
    assert_eq!(
        validate_bot_manifest(&parsed, &["discord", "reddit"]),
        Err(ValidationError::InvalidSetup("invalid field setup_command".into()))
    );
}

#[test]
fn placeholder_hash_and_signature_errors_constructible() {
    let hash = ValidationError::HashMismatchPlaceholder;
    let sig = ValidationError::InvalidSignaturePlaceholder;
    assert!(matches!(hash, ValidationError::HashMismatchPlaceholder));
    assert!(matches!(sig, ValidationError::InvalidSignaturePlaceholder));
}
