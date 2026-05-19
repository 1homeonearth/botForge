## Remaining work (2026-05-19)
- Implement Cryer Python/WASM bot module runtime handlers:
  - `cryer.server.register_from_squire`
  - `cryer.campaign.schedule`
  - `cryer.campaign.preview`
  - `cryer.campaign.run`
  - `cryer.post.monitor`
  - `cryer.campaign.report`
- Wire intents/events for Reddit post submit/status/flair, BotForge jobs, Discord embeds, and audit writes.
- Add setup UI schema pages + authorization checks and staged confirmation flows in runtime.
- Implement campaign runner logic (dry-run/live-run, throttle auto-scheduling, all-server mode, idempotency, removal monitoring, analytics aggregation).
- Add Rust runtime/service APIs that operate on `0003_cryer_campaigns.sql` tables.
- Add targeted tests for scheduling, cooldown enforcement, profile copy/promotion, monitor classification, and run reports.

## Completed this session
- Added Cryer SQLite migration `crates/botforge-runtime/migrations/0003_cryer_campaigns.sql`.
- Added Cryer contract constants in `crates/botforge-spec/src/lib.rs`.
- Added Cryer constants tests in `crates/botforge-spec/tests/schema_validation.rs`.
- Added design doc `docs/design/cryer-campaign-storage-and-setup.md`.
- Updated `README.md` and `CHANGELOG.md`.

## Continuation commands
- `sqlite3 :memory: ".read crates/botforge-runtime/migrations/0003_cryer_campaigns.sql"`
- `cargo test`
- `sed -n '1,260p' crates/botforge-runtime/migrations/0003_cryer_campaigns.sql`
- `sed -n '1,260p' docs/design/cryer-campaign-storage-and-setup.md`
