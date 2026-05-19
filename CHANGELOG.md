# CHANGELOG

## 2026-05-19
- Rebuilt root `AGENTS.md` to codify BotForge architecture law, migration law, and ISSUES/LEFTOVERS continuity workflow.
- Added canonical design docs under `docs/design/` for runtime/platform/Rampart model and migration law.
- Expanded `crates/botforge-spec` into schema crate with manifests/envelopes/capability structures and manifest examples.
- Added schema validation fixtures for missing fields, invalid versions, missing platform, unknown capability, forbidden permissions, unsupported build target, invalid setup, and placeholder hash/signature failures.
- Replaced runtime library with Gate/Court/Chamber skeleton and typed intent/event flow boundaries.
- Added runtime architecture tests proving raw platform events are rejected by Chamber, intents require Court validation before Gate execution, and impossible lifecycle activation jumps are blocked.
- Added storage scaffold migration `0002_core_storage.sql` with required logical tables for config/state/audit/evidence/verification/moderation/analytics domains.
- Security/migration note: legacy Squire/Bard/Cryer/Sentry transport assumptions remain behavior-only; runtime trust requires manifest+policy validation and Court-mediated execution.
