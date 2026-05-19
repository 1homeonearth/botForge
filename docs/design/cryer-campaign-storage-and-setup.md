# Cryer campaign storage + setup migration (BotForge)

This document defines the Cryer migration from Node/Express/CLI/JSON behavior to BotForge Python/WASM + Rust platform module boundaries.

## Runtime boundary
- Cryer is a first-party bot module, receives events, emits typed intents, and never calls Reddit directly.
- All Reddit operations route through the `reddit` platform module intents:
  - `reddit.post.submit.request`
  - `reddit.post.status.read.request`
  - `reddit.subreddit.flair.read.request`
- Reporting/audit paths use:
  - `discord.embed.send.request`
  - `botforge.audit.write`
  - `botforge.job.schedule.request`

## Handler surface
Cryer exposes these handlers as event-driven entrypoints:
- `cryer.server.register_from_squire`
- `cryer.campaign.schedule`
- `cryer.campaign.preview`
- `cryer.campaign.run`
- `cryer.post.monitor`
- `cryer.campaign.report`

## Setup panel surface
CLI commands are replaced with BotForge dashboard/setup panels:
- Campaign Home
- Server Campaign Profile
- Subreddit Profiles
- Template Queue
- Dry-run Preview
- Schedule Campaign
- Campaign History
- Removal Monitor
- Analytics

## SQLite storage model
Cryer campaign/state storage is migrated into SQLite tables via `0003_cryer_campaigns.sql`:
- `cryer_servers`
- `cryer_server_defaults`
- `cryer_subreddit_profiles`
- `cryer_campaign_templates`
- `cryer_campaign_queue`
- `cryer_cooldowns`
- `cryer_posted_records`
- `cryer_schedules`
- `cryer_removal_checks`
- `cryer_campaign_runs`
- `cryer_campaign_results`

## Security and migration implications
- Removes shared-key HTTP callback and Express attack surface.
- Replaces file/JSON transport with audited SQLite state + typed intents/events.
- Preserves campaign behavior while moving scheduling and monitoring to BotForge jobs and policy-governed platform requests.
