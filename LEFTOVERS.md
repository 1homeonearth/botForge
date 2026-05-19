## Remaining work (2026-05-19)
- Complete full setup UI/config engine implementation across all requested scopes/profiles, including pagination navigation state, confirm/cancel interaction flow wiring, staging validation/test execution hooks, and profile propagation semantics in storage/runtime.
- Implement and test botforge-python-wasm-sdk plus bot template repositories (`botforge-python-wasm-sdk`, `botforge-bot-template`) with host wrappers, formatting utilities, harness, manifests, and full docs.
- Implement Squire core modules/features and civic suite migration to SQLite-backed BotForge intents/events.
- Implement Sentry moderation/verification/evidence vault/security scaffolding and required schemas/tables.
- Continue unresolved platform-module/Wasmtime leftovers from prior sessions.

## Completed this session
- Added setup schema model support in `crates/botforge-spec` for page-based setup flows and interactive field types.
- Added runtime `SetupEngine` scaffolding for stage/diff/promote/rollback and secret masking.
- Added runtime tests for staged diff/promotion/rollback, secret masking, and audit entries.

## Continuation commands
- `cargo test`
- `sed -n '1,260p' crates/botforge-spec/src/lib.rs`
- `sed -n '1,320p' crates/botforge-runtime/src/lib.rs`
