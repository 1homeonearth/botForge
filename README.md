# BotForge

BotForge is a Rust-first runtime ecosystem for policy-contained WASM bots and Rust platform modules.

## Core model
- Gate / Court / Chamber runtime layering.
- Rust platform modules for all external communication.
- WASM bot/features with typed events/intents.
- SQLite-backed configuration, state, audit, evidence metadata, analytics.

## Implemented in this scaffold
- Runtime feature modules are isolated in per-feature folders (`features/squire/`, `features/bard/`) so feature bundles are easy to pull in, remove, or swap across bots; they implement deterministic migration behaviors (autoban, embed builder, XP/levels, moderation action records, rainbow bridge relay ledger, setup summary helpers, and Bard logging/moderation/starboard/welcome queue shaping with local file-backed logs).
- Typed `EventEnvelope` and `IntentEnvelope` schemas with source/target/actor/correlation fields.
- Court capability checks across capability + scope (platform/guild/channel) with audit records for route/deny.
- Inter-bot mediation pattern: Court converts approved intents into sanitized events.
- Platform module Rust contract + registry activation flow with required-secret checks.
- Skeleton platform-module documentation and setup placeholders for discord/reddit/spotify/youtube/rss/webhook.

- Setup engine now supports staged config diff preview, promotion, rollback, and secret-masked value presentation in runtime scaffolding.

- Cryer campaign migration scaffolding now defines canonical handlers/intents/setup panels and SQLite campaign storage tables for BotForge-native scheduling/reporting.

## Merge protection policy
- GitHub Actions workflow `.github/workflows/compile.yml` runs `cargo check --workspace --all-targets` for pull requests and pushes to `main`.
- Configure branch protection on `main` to require the `Rust compile gate` status check before merge so uncompiled changes cannot be merged.
