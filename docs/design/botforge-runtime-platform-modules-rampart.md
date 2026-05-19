# BotForge Runtime + Platform Modules + Rampart (Canonical)

This document is canonical project law for architecture.

## Architecture
- Runtime (Rust/WASM) owns discovery, manifests, packaging, WASM execution, capability brokerage, event routing, intent validation, setup UI, state, audit, module/platform registries, activation lifecycle, update staging, private dashboard.
- Platform modules (Rust) own every external API interaction and must expose normalized events/intents.
- Rampart owns host hardening, isolation, integrity, backups, recovery, signing, incident response.

## Hard rules
- No JavaScript/Node/discord.js/Node-based Cryer.
- Bots and features are WASM modules.
- Python bot logic is allowed only via Python/WASM SDK.
- Bots must not access raw network/fs/secrets/env/shell/process/platform clients/direct bot-to-bot channels.
- All egress through short-name platform modules: discord, reddit, spotify, youtube, rss, webhook, telegram, matrix, mattermost, session, email.

## Gate / Court / Chamber
- **Gate:** platform ingress/egress, replay/rate/payload/signature checks, normalization, error mapping, health status.
- **Court:** capability checks, policy decisions, routing, validation, audit, setup/config/state governance, lifecycle/jobs/staging.
- **Chamber:** WASM execution sandbox with constrained host functions; no raw host escape.

## State + setup
- SQLite is canonical for config/state/audit/evidence metadata/analytics/registries/jobs.
- Raw evidence objects are encrypted object storage; SQLite retains hashes/metadata/chain-of-custody.
- Setup flows are BotForge-rendered UI with staged rollout + rollback.

## First-party ownership
- Squire: civic utility + customization.
- Bard: social/fun culture features.
- Cryer: outbound campaigns and bulletin workflows.
- Sentry: verification/enforcement/evidence moderation.
- Sentry Omega: integrity/signature/verification stack.
