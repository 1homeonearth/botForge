use std::collections::BTreeMap;
use botforge_runtime::{SetupEngine, SetupScope};

#[test]
fn stage_diff_promote_and_rollback() {
    let mut engine = SetupEngine::default();
    let mut staged = BTreeMap::new();
    staged.insert("channel_id".into(), "123".into());
    staged.insert("api_secret".into(), "top-secret".into());

    engine.stage(SetupScope::ThisServer, staged);
    let diff = engine.preview_diff(&SetupScope::ThisServer);
    assert_eq!(diff.get("channel_id").unwrap().1.as_deref(), Some("123"));

    engine.promote(&SetupScope::ThisServer).unwrap();
    assert_eq!(engine.active.get(&SetupScope::ThisServer).unwrap().revision, 1);

    let mut staged2 = BTreeMap::new();
    staged2.insert("channel_id".into(), "999".into());
    engine.stage(SetupScope::ThisServer, staged2);
    engine.promote(&SetupScope::ThisServer).unwrap();
    assert_eq!(engine.active.get(&SetupScope::ThisServer).unwrap().revision, 2);

    engine.rollback(&SetupScope::ThisServer).unwrap();
    assert_eq!(engine.active.get(&SetupScope::ThisServer).unwrap().values.get("channel_id").map(String::as_str), Some("123"));
}

#[test]
fn secret_masking_and_audit_entries() {
    let mut values = BTreeMap::new();
    values.insert("discord_token".into(), "abc".into());
    values.insert("name".into(), "squire".into());
    let masked = SetupEngine::masked_values(&values);
    assert_eq!(masked.get("discord_token").map(String::as_str), Some("********"));
    assert_eq!(masked.get("name").map(String::as_str), Some("squire"));

    let mut engine = SetupEngine::default();
    engine.stage(SetupScope::Bot, values);
    engine.promote(&SetupScope::Bot).unwrap();
    assert!(engine.audit.iter().any(|a| a.contains("setup.stage")));
    assert!(engine.audit.iter().any(|a| a.contains("setup.promote")));
}
