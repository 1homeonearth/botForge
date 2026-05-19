# Repository-Informed Migration Law (Canonical)

Squire/Bard/Cryer/Sentry legacy implementations are migration references for feature behavior only.

## Mandatory migration transformations
- Queue-based transports -> typed BotForge intents/events.
- File/JSON persistence -> SQLite via BotForge APIs.
- Local HTTP/process bridges -> Court-mediated calls + platform modules.
- Direct platform SDK/API use -> Rust platform module contracts.
- Setup scripts/manual config -> setup schema + rendered setup UI.
- Bot-local logging -> centralized BotForge audit/log stream.
- Enforcement features -> Sentry policy-owned domain.

## Compatibility principle
Preserve user-visible behavior where appropriate, but replace transport/storage/authority with BotForge runtime primitives.

## Release discipline
Every behavior migration requires:
- Updated manifests/spec compatibility notes.
- Tests proving behavior survived migration boundaries.
- Changelog entries including security implications.
