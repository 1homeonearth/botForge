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

See `AGENTS.md` and canonical docs under `docs/design/` before coding.
