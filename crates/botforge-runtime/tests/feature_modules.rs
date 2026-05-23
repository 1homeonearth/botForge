use botforge_runtime::features::bard::{LoggingForwarder, ModerationLogging, Starboard, WelcomeCard};
use botforge_runtime::features::squire::{
    ban, interactive_mock, mute, prepare_setup_summary, summary, validate_channel_id, AutobanDecider, EmbedBuilder,
    ExperienceTracker, RainbowBridge,
};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn autoban_recommends_when_threshold_hit() {
    let mut decider = AutobanDecider::new(2);
    assert!(decider.record_violation("u1", "spam").is_none());
    assert!(decider.record_violation("u1", "abuse").is_some());
}

#[test]
fn embed_builder_build_save_load_and_color_masking() {
    let path = std::env::temp_dir().join("botforge_embed_builder_test.json");
    let mut builder = EmbedBuilder::new(Some(path.clone()));
    builder.set_title("hello");
    builder.set_description("world");
    builder.set_color(0xFFAA_BBCC);
    builder.add_field("n", "v", true);
    builder.set_footer("f");
    let built = builder.build();
    assert_eq!(built.color, 0x00AA_BBCC);
    builder.save().expect("save");

    let mut reloaded = EmbedBuilder::new(Some(path));
    reloaded.load().expect("load");
    assert_eq!(reloaded.template.title, "hello");
}

#[test]
fn experience_tracker_award_summary_and_persistence() {
    let path = std::env::temp_dir().join("botforge_xp_test.json");
    let mut tracker = ExperienceTracker::new(100, Some(path.clone()));
    let p = tracker.award_xp("u1", -20);
    assert_eq!(p.xp, 0);
    let p = tracker.award_xp("u1", 210);
    assert_eq!(p.level, 3);
    tracker.save().expect("save xp");

    let mut loaded = ExperienceTracker::new(1, Some(path));
    loaded.load().expect("load xp");
    assert_eq!(loaded.summary("u1").level, 3);
}

#[test]
fn moderation_and_setup_helpers_work() {
    let records = vec![mute("u1", "caps", 30), ban("u2", "raid", 7)];
    let text = summary(&records);
    assert!(text.contains("duration_minutes=30"));
    assert!(text.contains("delete_message_days=7"));

    assert!(validate_channel_id("12345"));
    assert!(!validate_channel_id("abc"));
    let features = vec!["moderation".to_string(), "logging".to_string()];
    let mut channels = HashMap::new();
    channels.insert("moderation".to_string(), "7777".to_string());
    let setup = prepare_setup_summary(&features, &channels);
    assert!(setup.contains("feature=moderation channel=7777"));
    assert!(interactive_mock(&features).contains("10000"));
}

#[test]
fn rainbow_bridge_tracks_relays() {
    let mut bridge = RainbowBridge::default();
    bridge.add_bridge("a", "b", "x");
    let relays = bridge.relay_message("a", "alice", "hello");
    assert_eq!(relays.len(), 1);
    assert_eq!(bridge.last_relays(Some(1)).len(), 1);
    assert!(bridge.remove_bridge("x"));
}

#[test]
fn bard_modules_log_and_queue_payloads() {
    let root = std::env::temp_dir().join("botforge_bard_test");
    std::fs::create_dir_all(root.join("logs")).expect("logs dir");

    let lf = LoggingForwarder {
        event_log: root.join("logs/logging_forwarder.log"),
    };
    let p = lf.record_server_event("s1", "c1", "hello").expect("log event");
    assert_eq!(p.kind, "log");

    let ml = ModerationLogging {
        mod_log: root.join("logs/moderation.log"),
    };
    let mp = ml.log_action("ban", "mod", "u1", "raid").expect("mod log");
    assert_eq!(mp["kind"], "moderation_log");

    let sb = Starboard {
        starboard_log: root.join("logs/starboard.log"),
    };
    assert!(sb
        .record_reaction("m1", "a", "c", &["x".to_string(), "y".to_string(), "z".to_string()], 3)
        .expect("starboard")
        .is_some());

    let wc = WelcomeCard { template_path: PathBuf::from("/no/such/template") };
    let payload = wc.build_welcome_card("alice", "guild", Some("be nice"));
    assert_eq!(payload["kind"], "welcome_card");
}

#[test]
fn experience_tracker_guards_zero_level_scale() {
    let mut tracker = ExperienceTracker::new(0, None);
    let progress = tracker.award_xp("u1", 10);
    assert_eq!(progress.level, 11);
    assert_eq!(tracker.summary("u1").level, 11);
}
