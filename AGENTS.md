BotForge Ultimate Implementation Tutorial
A practical, Codex-ready guide for building BotForge Runtime, platform modules, first-party bots, Sentry Omega, and Rampart.
Source manual SHA-256: d2e9442a5f5e9d4a27897c05…
 
How to Use This Document
This tutorial is organized for actual implementation work. Read the strategy sections first, create the new BotForge repo, place the AGENTS.md material, then feed the standalone Codex prompt boxes one at a time according to the build order. Each Codex prompt is self-contained enough to use in an individual Codex session.
•	Use Section 1 to decide where implementation begins.
•	Use Section 2 to create the first repository and workspace.
•	Use Section 3 to seed AGENTS.md and docs/design/.
•	Use Section 4 as the implementation map.
•	Use Section 5 for ordered Codex prompts.
•	Use the appendices as the full manual-ingestion packet and source reference.
1. Implementation Decision: Start a New BotForge Repo
Recommendation: start a whole new repo.
Use the current Squire and Cryer repositories as migration references, not as the implementation foundation. The new system is not a patch to one bot. It is a runtime/platform ecosystem centered on BotForge Runtime, Rust platform modules, WASM bot modules, centralized SQLite state, typed events/intents, and Rampart hardening.
Why not Squire first: Squire currently carries the older Discovery hub pattern: Rust presence markers, queue files, and Python modules that prepare records for Rust forwarding. That is useful project history, but the new architecture replaces it with manifests, capabilities, typed intents, WASM containment, and platform modules.
Why not Cryer first: Cryer currently carries the strongest old-architecture gravity: Node, Express, dotenv, prompts, JSON file storage, local HTTP endpoints, and direct Reddit logic. The new architecture has hard rules against JavaScript, Node, Express, direct Reddit calls, and bot-held credentials.
Best first implementation repo: botforge-runtime.
Start it as a Rust workspace at first, with local crates for botforge-spec, botforge-runtime, botforge-platform-discord, botforge-platform-reddit, botforge-python-wasm-sdk, and botforge-bot-template. This lets the spine evolve together while the contracts are still settling.
Once the spine is stable, split or mirror into the intended repo set: botforge-spec, botforge-runtime, botforge-platform-discord, botforge-platform-reddit, botforge-python-wasm-sdk, botforge-bot-template, squire, bard, cryer, sentry, sentry-omega, rampart, and botforge-suite.
Create botforge-suite later as the pinned out-of-the-box ecosystem repo. It can pin runtime, platform modules, first-party bots, Sentry Omega, and Rampart through lockfiles and/or submodules.
2. Recommended First Repository Layout
The first repo should be botforge-runtime as a Rust workspace. This keeps the spec, runtime, platform registry, first platform modules, Python/WASM SDK, and working template together while the interfaces are still forming.
TEXT BOX 2.1 — Recommended early Rust workspace layout
botforge-runtime/
  AGENTS.md
  README.md
  CHANGELOG.md
  docs/
    design/
      botforge-runtime-platform-modules-rampart.md
      repository-informed-migration.md
    architecture.md
    implementation-roadmap.md
  crates/
    botforge-spec/
    botforge-runtime/
    botforge-platform-discord/
    botforge-platform-reddit/
    botforge-python-wasm-sdk/
    botforge-bot-template/
  tests/
  Cargo.toml
  Cargo.lock

After the spine stabilizes, split or mirror into the full repo set:
TEXT BOX 2.2 — Target mature repo set
botforge-spec
botforge-runtime
botforge-platform-discord
botforge-platform-reddit
botforge-platform-spotify
botforge-platform-youtube
botforge-platform-rss
botforge-platform-webhook
botforge-python-wasm-sdk
botforge-bot-template
squire
bard
cryer
sentry
sentry-omega
rampart
botforge-suite

3. First Moves: AGENTS.md and docs/design/
Your first move in the new repo should be to place the manual-derived rules in AGENTS.md and docs/design/. You said you will personally rewrite AGENTS.md, so the two boxes below are written as inclusion material rather than a command to overwrite your final voice.
TEXT BOX A1 — AGENTS.md manual inclusion, part 1
# BotForge Runtime + Platform Modules + Rampart: Canonical Agent Instructions
 
This repository belongs to the BotForge ecosystem. Treat these instructions as binding project law unless Katie explicitly revises them.
 
BotForge Runtime is the Rust/WASM execution environment. It owns module discovery, manifests, build/package flow, WebAssembly execution, capability brokerage, event routing, intent validation, setup UI, state management, audit logging, update staging, private status dashboard, module registry, platform module registry, SQLite storage, and activation lifecycle.
 
Platform modules are short-named Rust modules that own all communication with external services. Runtime IDs must be simple lowercase names: discord, reddit, spotify, youtube, rss, webhook, telegram, matrix, mattermost, session, email. Repositories may use longer names such as botforge-platform-discord, but runtime IDs stay short. A platform module contains the Rust client, authentication handling, secret-reference handling, event normalization, rate limits, API call implementations, schemas, setup declarations, error mapping, retry/idempotency rules, tests, mock responses, and capability declarations for that service.
 
Rampart is the deployment hardening shield. It owns OS service isolation, read-only source mounts, writable state mounts, firewall policy, integrity monitoring, release verification, backup strategy, recovery playbooks, signing workflows, hardened production profiles, incident response, and owner emergency access.
 
Core rule: BotForge owns execution and authority. Platform modules own external communication. Bots emit typed intents and receive sanitized events. Rampart hardens the host.
 
No bot, feature module, Python logic, setup helper, template code, or WASM component may directly communicate with Discord, Reddit, Spotify, YouTube, Telegram, Matrix, Mattermost, Session, RSS, webhooks, email, or any external platform. All external calls go through installed Rust platform modules after BotForge authorization.
 
No JavaScript. No Node. No discord.js. No Node-based Cryer. BotForge Runtime is Rust. Platform modules are Rust. Security-sensitive bot modules may be Rust/WASM. Ordinary bot behavior may be authored in Python through the BotForge Python/WASM SDK and then packaged as WebAssembly. Deployed bots are WASM modules/components, not normal Python processes with host filesystem, network, environment, or process access.
 
The discord platform module uses Twilight for Discord gateway and HTTP protocol implementation. Twilight handles protocol details. The discord platform module maps Twilight events and operations into BotForge schemas. BotForge handles capability checks, routing, setup, state, audit, module activation, and policy.
 
Bots are policy-contained modules. Bots never directly call Discord, Reddit, Spotify, YouTube, Telegram, Matrix, Mattermost, Session, RSS, webhooks, email, or any external API. Bots never receive raw secrets, raw network access, arbitrary shell access, environment access, direct inter-bot calls, platform SDKs for live calls, or direct platform clients. Bots emit typed intents. BotForge validates, authorizes, audits, routes, executes, denies, stages, or asks for confirmation.
 
Bot, feature, platform, and policy source folders are read-only at runtime. BotForge writes no runtime data into source folders. Build artifacts go into BotForge-managed build/cache paths. Configuration, state, logs, caches, audit entries, evidence, and runtime data live under BotForge-managed state paths. Removing a bot source folder deactivates the bot and marks state orphaned. Removing a platform module disables its capabilities and intents only after BotForge shows dependent bots/features and receives confirmation.
 
Squire, Bard, Cryer, and Sentry are complete first-party bots. They must run as usable complete solutions when installed with required platform modules and configured through BotForge setup. The ecosystem also includes a real bot template, not a stub. The template must include a working event handler, setup schema, state access example, intent emission example, feature module example, tests, manifests, documentation, and teaching comments.
 
Drop-in folders plus manifests are approved. A folder placed into a module directory is discovered, evaluated, and never trusted merely because it exists. Activation requires manifest validation, version compatibility checks, source integrity checks, build/package success, import inspection, tests, capability review, setup completion, and owner/admin approval. Folder nesting is allowed only as packaging convenience. Runtime trust comes from manifests, namespaces, capabilities, and activation state.
 
Git submodules are approved where independent repos need nesting. Generated or vendored SDK code may live in clearly marked folders such as _vendor/botforge_py/. Production uses a BotForge lockfile. Scheduled checks may fetch allowlisted upstream changes. Updates must be signed, hashed, built, tested, staged, diffed, and approved before production promotion. First-party repos may auto-fetch and auto-stage. Auto-promotion requires narrow signed security policy.
 
Normal setup happens through BotForge-rendered UI. Manual JSON/TOML/YAML editing is developer/emergency path only. Setup uses Discord slash commands, buttons, select menus, modals, previews, confirmations, paginated panels, staging mode, profile propagation, and rollback. Bots, features, platform modules, and policy modules declare setup needs. BotForge renders UI, validates input, stores config centrally, and applies settings by scope.
 
SQLite is canonical local storage for BotForge configuration, bot state, platform module state, evidence metadata, audit records, job state, analytics, and registry data. Raw evidence/media objects live in encrypted object storage managed by BotForge/Sentry. SQLite stores metadata, hashes, permissions, case links, and chain-of-custody records.

BotForge Ultimate Implementation Tutorial
A practical, Codex-ready guide for building BotForge Runtime, platform modules, first-party bots, Sentry Omega, and Rampart.
Source manual SHA-256: d2e9442a5f5e9d4a27897c05…
 
How to Use This Document
This tutorial is organized for actual implementation work. Read the strategy sections first, create the new BotForge repo, place the AGENTS.md material, then feed the standalone Codex prompt boxes one at a time according to the build order. Each Codex prompt is self-contained enough to use in an individual Codex session.
•	Use Section 1 to decide where implementation begins.
•	Use Section 2 to create the first repository and workspace.
•	Use Section 3 to seed AGENTS.md and docs/design/.
•	Use Section 4 as the implementation map.
•	Use Section 5 for ordered Codex prompts.
•	Use the appendices as the full manual-ingestion packet and source reference.
1. Implementation Decision: Start a New BotForge Repo
Recommendation: start a whole new repo.
Use the current Squire and Cryer repositories as migration references, not as the implementation foundation. The new system is not a patch to one bot. It is a runtime/platform ecosystem centered on BotForge Runtime, Rust platform modules, WASM bot modules, centralized SQLite state, typed events/intents, and Rampart hardening.
Why not Squire first: Squire currently carries the older Discovery hub pattern: Rust presence markers, queue files, and Python modules that prepare records for Rust forwarding. That is useful project history, but the new architecture replaces it with manifests, capabilities, typed intents, WASM containment, and platform modules.
Why not Cryer first: Cryer currently carries the strongest old-architecture gravity: Node, Express, dotenv, prompts, JSON file storage, local HTTP endpoints, and direct Reddit logic. The new architecture has hard rules against JavaScript, Node, Express, direct Reddit calls, and bot-held credentials.
Best first implementation repo: botforge-runtime.
Start it as a Rust workspace at first, with local crates for botforge-spec, botforge-runtime, botforge-platform-discord, botforge-platform-reddit, botforge-python-wasm-sdk, and botforge-bot-template. This lets the spine evolve together while the contracts are still settling.
Once the spine is stable, split or mirror into the intended repo set: botforge-spec, botforge-runtime, botforge-platform-discord, botforge-platform-reddit, botforge-python-wasm-sdk, botforge-bot-template, squire, bard, cryer, sentry, sentry-omega, rampart, and botforge-suite.
Create botforge-suite later as the pinned out-of-the-box ecosystem repo. It can pin runtime, platform modules, first-party bots, Sentry Omega, and Rampart through lockfiles and/or submodules.
2. Recommended First Repository Layout
The first repo should be botforge-runtime as a Rust workspace. This keeps the spec, runtime, platform registry, first platform modules, Python/WASM SDK, and working template together while the interfaces are still forming.
TEXT BOX 2.1 — Recommended early Rust workspace layout
botforge-runtime/
  AGENTS.md
  README.md
  CHANGELOG.md
  docs/
    design/
      botforge-runtime-platform-modules-rampart.md
      repository-informed-migration.md
    architecture.md
    implementation-roadmap.md
  crates/
    botforge-spec/
    botforge-runtime/
    botforge-platform-discord/
    botforge-platform-reddit/
    botforge-python-wasm-sdk/
    botforge-bot-template/
  tests/
  Cargo.toml
  Cargo.lock

After the spine stabilizes, split or mirror into the full repo set:
TEXT BOX 2.2 — Target mature repo set
botforge-spec
botforge-runtime
botforge-platform-discord
botforge-platform-reddit
botforge-platform-spotify
botforge-platform-youtube
botforge-platform-rss
botforge-platform-webhook
botforge-python-wasm-sdk
botforge-bot-template
squire
bard
cryer
sentry
sentry-omega
rampart
botforge-suite

3. First Moves: AGENTS.md and docs/design/
Your first move in the new repo should be to place the manual-derived rules in AGENTS.md and docs/design/. You said you will personally rewrite AGENTS.md, so the two boxes below are written as inclusion material rather than a command to overwrite your final voice.
TEXT BOX A1 — AGENTS.md manual inclusion, part 1
# BotForge Runtime + Platform Modules + Rampart: Canonical Agent Instructions
 
This repository belongs to the BotForge ecosystem. Treat these instructions as binding project law unless Katie explicitly revises them.
 
BotForge Runtime is the Rust/WASM execution environment. It owns module discovery, manifests, build/package flow, WebAssembly execution, capability brokerage, event routing, intent validation, setup UI, state management, audit logging, update staging, private status dashboard, module registry, platform module registry, SQLite storage, and activation lifecycle.
 
Platform modules are short-named Rust modules that own all communication with external services. Runtime IDs must be simple lowercase names: discord, reddit, spotify, youtube, rss, webhook, telegram, matrix, mattermost, session, email. Repositories may use longer names such as botforge-platform-discord, but runtime IDs stay short. A platform module contains the Rust client, authentication handling, secret-reference handling, event normalization, rate limits, API call implementations, schemas, setup declarations, error mapping, retry/idempotency rules, tests, mock responses, and capability declarations for that service.
 
Rampart is the deployment hardening shield. It owns OS service isolation, read-only source mounts, writable state mounts, firewall policy, integrity monitoring, release verification, backup strategy, recovery playbooks, signing workflows, hardened production profiles, incident response, and owner emergency access.
 
Core rule: BotForge owns execution and authority. Platform modules own external communication. Bots emit typed intents and receive sanitized events. Rampart hardens the host.
 
No bot, feature module, Python logic, setup helper, template code, or WASM component may directly communicate with Discord, Reddit, Spotify, YouTube, Telegram, Matrix, Mattermost, Session, RSS, webhooks, email, or any external platform. All external calls go through installed Rust platform modules after BotForge authorization.
 
No JavaScript. No Node. No discord.js. No Node-based Cryer. BotForge Runtime is Rust. Platform modules are Rust. Security-sensitive bot modules may be Rust/WASM. Ordinary bot behavior may be authored in Python through the BotForge Python/WASM SDK and then packaged as WebAssembly. Deployed bots are WASM modules/components, not normal Python processes with host filesystem, network, environment, or process access.
 
The discord platform module uses Twilight for Discord gateway and HTTP protocol implementation. Twilight handles protocol details. The discord platform module maps Twilight events and operations into BotForge schemas. BotForge handles capability checks, routing, setup, state, audit, module activation, and policy.
 
Bots are policy-contained modules. Bots never directly call Discord, Reddit, Spotify, YouTube, Telegram, Matrix, Mattermost, Session, RSS, webhooks, email, or any external API. Bots never receive raw secrets, raw network access, arbitrary shell access, environment access, direct inter-bot calls, platform SDKs for live calls, or direct platform clients. Bots emit typed intents. BotForge validates, authorizes, audits, routes, executes, denies, stages, or asks for confirmation.
 
Bot, feature, platform, and policy source folders are read-only at runtime. BotForge writes no runtime data into source folders. Build artifacts go into BotForge-managed build/cache paths. Configuration, state, logs, caches, audit entries, evidence, and runtime data live under BotForge-managed state paths. Removing a bot source folder deactivates the bot and marks state orphaned. Removing a platform module disables its capabilities and intents only after BotForge shows dependent bots/features and receives confirmation.
 
Squire, Bard, Cryer, and Sentry are complete first-party bots. They must run as usable complete solutions when installed with required platform modules and configured through BotForge setup. The ecosystem also includes a real bot template, not a stub. The template must include a working event handler, setup schema, state access example, intent emission example, feature module example, tests, manifests, documentation, and teaching comments.
 
Drop-in folders plus manifests are approved. A folder placed into a module directory is discovered, evaluated, and never trusted merely because it exists. Activation requires manifest validation, version compatibility checks, source integrity checks, build/package success, import inspection, tests, capability review, setup completion, and owner/admin approval. Folder nesting is allowed only as packaging convenience. Runtime trust comes from manifests, namespaces, capabilities, and activation state.
 
Git submodules are approved where independent repos need nesting. Generated or vendored SDK code may live in clearly marked folders such as _vendor/botforge_py/. Production uses a BotForge lockfile. Scheduled checks may fetch allowlisted upstream changes. Updates must be signed, hashed, built, tested, staged, diffed, and approved before production promotion. First-party repos may auto-fetch and auto-stage. Auto-promotion requires narrow signed security policy.
 
Normal setup happens through BotForge-rendered UI. Manual JSON/TOML/YAML editing is developer/emergency path only. Setup uses Discord slash commands, buttons, select menus, modals, previews, confirmations, paginated panels, staging mode, profile propagation, and rollback. Bots, features, platform modules, and policy modules declare setup needs. BotForge renders UI, validates input, stores config centrally, and applies settings by scope.
 
SQLite is canonical local storage for BotForge configuration, bot state, platform module state, evidence metadata, audit records, job state, analytics, and registry data. Raw evidence/media objects live in encrypted object storage managed by BotForge/Sentry. SQLite stores metadata, hashes, permissions, case links, and chain-of-custody records.

TEXT BOX A2 — AGENTS.md manual inclusion, part 2
# BotForge Feature Specifications, Migration Rules, Testing, Deployment, and Build Order
 
Responsibilities: Squire owns non-punitive Discord utility, civic infrastructure, setup/customization, dashboards, role menus, templates, reminders, suggestions, analytics, backups, cross-server utility syncing, and general assistant behavior. Bard owns fun, immersive, playful, social, music, games, birthdays, confessions, QOTD, trivia, server culture, and vibe features. Cryer owns advertising, Reddit campaigns, formal outbound announcements, external bulletins, campaign templates, schedules, removal tracking, ad analytics, and formal news/bulletins. Sentry owns verification, enforcement, automod, tickets, modmail, appeals, infractions, moderation notes, evidence, anti-raid, anti-spam, anti-scam, emergency console, moderation analytics, owner logs, and enforcement-sensitive alerts. Rampart owns hardening, service isolation, source read-only policy, firewall profiles, integrity deployment, backups, recovery, signing, and incident response.
 
Sentry owns new-member verification. VPN/proxy risk is part of risk intake. Rule attestation alone is insufficient. Verification may include photos, video, audio, live sessions, randomized liveness challenges, surprise prompts, moderator review, evidence retention, vouching, progressive access, and post-entry monitoring. Automated gender classification is prohibited. The system verifies liveness, account control, seriousness, consistency, risk, and adherence to community boundaries. Evidence retention is permanent unless Katie changes policy. Evidence access is restricted, logged, encrypted, and auditable.
 
Required repos: botforge-runtime, botforge-spec, botforge-platform-discord, botforge-platform-reddit, botforge-platform-spotify, botforge-platform-youtube, botforge-platform-rss, botforge-platform-webhook, botforge-python-wasm-sdk, botforge-bot-template, squire, bard, cryer, sentry, sentry-omega, rampart. Optional future repos: botforge-platform-telegram, botforge-platform-matrix, botforge-platform-mattermost, botforge-platform-session, botforge-platform-email, botforge-examples, botforge-suite.
 
BotForge layers: Gate touches outside world through platform modules and never sends raw platform events to bots. Court is authority and routing: capability checks, policy, routing, validation, inter-bot mediation, audit, config, setup UI, registries, activation, jobs, update staging, dashboard, evidence, scopes, profiles, staging, rollback. Chamber runs WASM, exposes host functions, enforces limits, delivers events, collects intents, controls state/log/metric access, and never exposes raw network, filesystem, secrets, env vars, or process execution.
 
Capabilities are scoped by bot, feature, guild/server, channel, role, user, platform module, environment, time window, rate limit, and setup profile. Events describe things that happened. Intents describe actions requested. BotForge validates intents, checks capability scope, audits, then executes/denies/stages/confirms. Inter-bot communication happens only through mediated intents/events.
 
Required docs in every repo: README.md, CHANGELOG.md, AGENTS.md, docs/, tests/, manifest examples where applicable, and security notes where applicable. Every behavior change updates CHANGELOG with date, changed files, behavior, security impact, migration, tests, docs. Comments teach syntax, variables, control flow, purpose, data structures, security decisions, capability checks, error handling, WASM/platform boundaries, Rust ownership/lifetimes, and Python behavior.
 
Migration law: current modules that write files, queue gateway messages, call local HTTP services, or return action records become WASM modules emitting typed BotForge intents and using SQLite/state APIs. User-facing behavior survives when it belongs to the feature. Transport, storage, and authority move into BotForge and Rust platform modules. Queue files become typed intents. JSON persistence becomes SQLite. Direct Discord/Reddit/Spotify/YouTube/RSS/webhook calls become Rust platform calls. Setup helpers become BotForge setup schemas. Logging becomes BotForge audit/log service. Squire/Bard/Cryer/Sentry remain complete. botforge-bot-template is real.
 
Migration inventory: Squire autoban.py becomes sentry.autoban. moderation_commands.py becomes sentry.moderation_actions. experience.py becomes squire.experience. Thirst XP becomes squire.thirst_experience. embed_builder.py becomes squire.embed_builder plus SDK utility. rainbow_bridge.py becomes squire.cross_server_bridge. setup.py becomes BotForge setup engine plus Squire schemas. core/logger.py becomes BotForge audit/logging plus SDK wrapper. lib/display.py becomes SDK formatting helper. config_loader.py becomes BotForge config/vault. crypto/passwords.py becomes BotForge/Rampart auth crypto. crypto/integrity.py becomes Sentry Omega/Rampart integrity. crypto/secrets.py becomes BotForge/Rampart vault reference only. embedded_interpreter.py and local_repl.py become safe dev console/read-only production diagnostics. Rust setup_panel.rs becomes setup renderer. Rust discord_gateway.rs and central_comm.rs become BotForge Gate/Court foundation.
 
Bard old welcome_card.py moves to squire.welcome_cards. starboard.py moves to squire.starboard. logging_forwarder.py merges into BotForge audit/log forwarding. moderation_logging.py moves to sentry.moderation_log. Bard demo main becomes template tests/demo fixture. Production Bard focuses on confessions, QOTD, trivia, games, music, birthdays, relationships, phrase echo, lore, quotes, festivals, polls.
 
Cryer package.json/Node packaging retires. Express API in index.js becomes BotForge intent handlers and scheduler. store.js JSON registry becomes SQLite campaign storage. CLI becomes Discord setup panels. Subreddit add/edit/queue flow survives as cryer.subreddit_profiles. postNow/postAll/throttle/scheduling survives as campaign runner using BotForge scheduler. Sync from Squire becomes BotForge server registry events. Logs UI becomes BotForge dashboard. lib/reddit.js becomes Rust reddit platform. logger.js becomes audit/log service. network helper concept moves into adapter/network validation. Sentry Omega README/lib.rs survive as sentry-omega with real crypto, signatures, canonical manifests, BotForge lockfile verification, Rampart checks, dashboard reporting.
 
Implementation phases: 1 botforge-spec schemas; 2 runtime skeleton, SQLite, registries, manifest validation, Wasmtime prototype, host functions, audit; 3 discord platform with Twilight; 4 reddit platform; 5 Python/WASM SDK and working bot template; 6 Squire MVP; 7 Sentry MVP; 8 Rampart MVP; 9 Cryer MVP; 10 Bard MVP; 11 advanced audit-log diffing, webhook relay, backups, drift enforcement, templates, analytics, cross-server sync, appeals, emergency console.


## Session Continuity Files (ISSUES.md and LEFTOVERS.md)

- Before new implementation work, review `ISSUES.md` and `LEFTOVERS.md` at repository root.
- Resolve unresolved `ISSUES.md` items first; record each solution attempt in the relevant issue chain.
- Execute `LEFTOVERS.md` remaining tasks second, then clear completed handoff noise while keeping useful unresolved or final resolution context.
- Verify presumed fixes with concrete test/check commands before marking issues complete.
- Keep `ISSUES.md` updated with timestamps, commands, outcomes, and final resolutions.
