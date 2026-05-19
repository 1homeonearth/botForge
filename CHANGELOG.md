# CHANGELOG

## 2026-05-19
- Rebuilt root `AGENTS.md` to codify BotForge architecture law, migration law, and ISSUES/LEFTOVERS continuity workflow.
- Added canonical design docs under `docs/design/` for runtime/platform/Rampart model and migration law.
- Expanded `crates/botforge-spec` into schema crate with manifests/envelopes/capability structures and manifest examples.
- Added schema validation tests and platform name validation coverage.
- Replaced runtime library with Gate/Court/Chamber skeleton and typed intent/event flow boundaries.
- Added runtime architecture tests proving raw platform events are rejected by Chamber and intents require Court validation before Gate execution.
- Added storage scaffold migration `0002_core_storage.sql` with required logical tables for config/state/audit/evidence/verification/moderation/analytics domains.
