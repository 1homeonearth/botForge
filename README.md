# BotForge

BotForge is a Rust-first runtime ecosystem for policy-contained WASM bots and Rust platform modules.

## Core model
- Gate / Court / Chamber runtime layering.
- Rust platform modules for all external communication.
- WASM bot/features with typed events/intents.
- SQLite-backed configuration, state, audit, evidence metadata, analytics.

## Rules snapshot
- No JavaScript/Node/discord.js/Node-based Cryer.
- No direct bot network/secrets/env/fs/shell/platform SDK access.
- Setup is BotForge-rendered, not ad-hoc transport scripts.

## Schema + runtime coverage in this repo
- `crates/botforge-spec` is the stable schema crate for bot/feature/platform/policy manifests, capability taxonomy, and event/intent envelopes.
- `crates/botforge-runtime` is the Gate/Court/Chamber skeleton with explicit lifecycle state transitions and boundary tests.
- SQLite scaffolding and migrations live under `crates/botforge-runtime/migrations` and represent BotForge-owned state paths.

See `AGENTS.md` and canonical docs under `docs/design/` before coding.
