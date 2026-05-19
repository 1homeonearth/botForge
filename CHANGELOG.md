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
