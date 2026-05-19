# BotForge Canonical Agent Instructions (Root)

Before any implementation work:
1. Read `ISSUES.md` and resolve unresolved items first.
2. Execute remaining `LEFTOVERS.md` tasks second.
3. Read this file, then read all canonical design docs in `docs/design/`.

## Binding architecture summary
- **Core law:** BotForge Runtime owns execution and authority. Platform modules own all external communication. Bots/features emit typed intents and receive sanitized events. Rampart hardens host/deployment.
- **Runtime stack:** Gate (external ingress/egress), Court (policy/authority/routing/audit), Chamber (WASM execution sandbox).
- **Storage:** SQLite is canonical for config, state, audit, evidence metadata, analytics, registry, jobs, staging.
- **Setup:** User setup is BotForge-rendered UI, not ad-hoc files.

## Hard prohibitions
- No JavaScript. No Node. No discord.js. No Node-based Cryer.
- Bots/features are deployed as WASM modules/components.
- Ordinary bot logic may be authored in Python **only** through the Python/WASM SDK.
- Bots/features never receive raw network, secrets, env vars, arbitrary filesystem, shell, platform SDK live calls, queue files, or direct inter-bot calls.
- No direct external API calls from bots/features.

## Platform module boundary
All external communication must pass through short-named Rust platform modules:
`discord`, `reddit`, `spotify`, `youtube`, `rss`, `webhook`, `telegram`, `matrix`, `mattermost`, `session`, `email`.

## Required domain definitions
AGENTS and design docs must encode BotForge Runtime, platform modules, Rampart, Squire, Bard, Cryer, Sentry, Sentry Omega, manifests, capabilities, events/intents, Gate/Court/Chamber, setup UI, SQLite, read-only source rules, lockfile update flow, evidence vault, verification policy, and migration law.

## Migration law (binding)
Old Squire/Bard/Cryer/Sentry behavior is a **behavior specification** only, not transport code to preserve. Migrate as follows:
- Queue files/local transport => typed intents/events.
- JSON/files => SQLite/state APIs.
- Manual setup scripts/forms => BotForge setup schemas + rendered setup UI.
- Direct Discord/Reddit/etc calls => Rust platform module contracts.
- Bot-local logs => BotForge audit/log service.
- Enforcement-sensitive behavior => Sentry ownership and Court policy.

## Docs/tests/changelog rules
- Any behavior or architecture change must update `README.md`, `CHANGELOG.md`, relevant docs under `docs/`, and tests.
- Include security and migration implications in docs/changelog entries.

## ISSUES.md / LEFTOVERS.md continuity
- Record each troubleshooting attempt in the appropriate issue chain in `ISSUES.md` with timestamp, commands, outcomes.
- Verify presumed fixes with concrete checks before marking complete.
- Keep only useful unresolved or final-resolution context.
- If work cannot be finished, update `LEFTOVERS.md` with exact continuation steps.
