# CHANGELOG

## 2026-05-19
- Implemented typed event/intent envelopes in `botforge-spec` including event_id/intent_id, source/target/actor, requested_by_event, and capabilities_used.
- Added capability taxonomy coverage and expanded `CapabilityGrant` scopes for bot/feature/guild/channel/role/user/platform/environment/time/rate/setup profile.
- Implemented Court intent validation, scope-based authorization, route/deny auditing, and inter-bot mediation into sanitized Court events.
- Added Gate sanitization helper for normalized platform events and Chamber forbidden-import guard for raw WASI surfaces.
- Added platform module contract trait and registry activation flow with secret validation.
- Added runtime tests for unknown capability deny, scope mismatch deny, valid route + audit, raw event sanitization, and registry activation/missing secret denial.
- Added `docs/platform_modules.md` plus skeleton platform module docs and setup schema placeholders under `platforms/`.

## 2026-05-19
- Added setup schema pages/field controls and runtime setup staging/promotion/rollback scaffolding with audit logging and tests.

## 2026-05-19
- Added Cryer migration design doc for event/intents boundary, setup-panel replacement for CLI, and security implications.
- Added runtime SQLite migration `0003_cryer_campaigns.sql` with campaign storage, templates/queue, cooldowns, schedules, removal checks, and run/results analytics tables.
- Added `botforge-spec` exported Cryer handler/intent/setup constants and tests asserting canonical contract names.

## 2026-05-22
- Added `botforge-runtime::features` scaffold modules for Squire/Bard migration behavior shaping (autoban, moderation actions, and deterministic queue payload constructors) without direct network calls.
- Added runtime tests in `crates/botforge-runtime/tests/feature_modules.rs` covering threshold recommendations, moderation summaries, and Bard payload shaping.

## 2026-05-22
- Expanded `botforge-runtime::features` with full Squire and Bard module behavior parity from migration specs, including embed builder, experience tracker, rainbow bridge ledger, setup summary utilities, and file-backed Bard module queue/log flows.
- Added focused tests for embed persistence, XP progression/persistence, setup helpers, rainbow relay ledgers, and Bard queue/log file outputs.

## 2026-05-22
- Folderized runtime feature modules into per-feature directories (`features/squire/`, `features/bard/`) to make drop-in/drop-out adoption explicit and low-friction for ecosystem portability.
