# BotForge

BotForge is a Rust-first runtime ecosystem for policy-contained WASM bots and Rust platform modules.

## Core model
- Gate / Court / Chamber runtime layering.
- Rust platform modules for all external communication.
- WASM bot/features with typed events/intents.
- SQLite-backed configuration, state, audit, evidence metadata, analytics.

## Implemented in this scaffold
- Typed `EventEnvelope` and `IntentEnvelope` schemas with source/target/actor/correlation fields.
- Court capability checks across capability + scope (platform/guild/channel) with audit records for route/deny.
- Inter-bot mediation pattern: Court converts approved intents into sanitized events.
- Platform module Rust contract + registry activation flow with required-secret checks.
- Skeleton platform-module documentation and setup placeholders for discord/reddit/spotify/youtube/rss/webhook.
