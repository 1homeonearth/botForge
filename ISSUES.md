# ISSUES

## 2026-05-19 00:00 UTC — Missing project tracking files and spec docs scaffold (Complete)
- **Status:** Complete
- **Context:** Session started without required `ISSUES.md` and `LEFTOVERS.md`; user task requires adding normative schema compatibility docs under `crates/botforge-spec` but repository only contained root metadata files.
- **Attempt log:**
  - 2026-05-19 00:01 UTC: Verified file inventory and confirmed missing files/paths.
  - 2026-05-19 00:02 UTC: Created `ISSUES.md` and `LEFTOVERS.md` placeholders to satisfy repo workflow.
  - 2026-05-19 00:03 UTC: Planned new docs under `crates/botforge-spec/docs/` for semantic versioning, compatibility behavior, migration metadata, and golden tests.
- **Commands run:**
  - `rg --files -g 'AGENTS.md' -g 'AGENTS.override.md'`
  - `rg --files`
- **Result:** Issue resolved by creating required tracking files and proceeding with requested documentation additions.

## 2026-05-19 00:10 UTC — Runtime policy precedence/enforcement and deterministic conflict handling (Complete)
- **Status:** Complete
- **Context:** Required implementation and documentation of canonical scope order, deny/allow precedence, profile/staged tie-breaking, rule-path audit recording, and deterministic conflict tests across `crates/botforge-runtime` and `crates/botforge-spec`.
- **Attempt log:**
  - 2026-05-19 00:10 UTC: Inspected repository and identified missing `crates/botforge-runtime` source and test scaffolding.
  - 2026-05-19 00:12 UTC: Added workspace and crate manifests for `botforge-spec` and `botforge-runtime`.
  - 2026-05-19 00:14 UTC: Implemented policy scope/effect/rule schema definitions in `crates/botforge-spec/src/lib.rs`.
  - 2026-05-19 00:16 UTC: Implemented runtime policy evaluator with deterministic sorting, staged filtering, and audit rule-path output in `crates/botforge-runtime/src/lib.rs`.
  - 2026-05-19 00:18 UTC: Added conflict-focused tests in `crates/botforge-runtime/tests/policy_engine.rs`.
  - 2026-05-19 00:19 UTC: Added runtime policy docs in `docs/design/runtime-policy-evaluation.md`.
  - 2026-05-19 00:20 UTC: Ran `cargo test` and confirmed all tests passed.
- **Commands run:**
  - `rg --files -g 'AGENTS.md' -g 'AGENTS.override.md' && rg --files ISSUES.md LEFTOVERS.md`
  - `cat AGENTS.md && cat ISSUES.md && cat LEFTOVERS.md`
  - `rg --files crates/botforge-runtime crates/botforge-spec docs`
  - `find . -maxdepth 4 -type f`
  - `cargo test`
- **Result:** All requested policy semantics were documented and implemented with deterministic tests and passing verification.

## 2026-05-19 07:52 UTC — Chamber host function security spec and runtime guardrails (Complete)
- **Status:** Complete
- **Context:** Added a Chamber host-function security spec and implementation checklist; implemented host-call authorization, capability binding, budget/backpressure enforcement, timeout/cancellation semantics, and abuse-case tests in `crates/botforge-runtime`.
- **Attempt log:**
  - 2026-05-19 07:48 UTC: Reviewed runtime crate and existing tests to determine insertion points for Chamber guardrails.
  - 2026-05-19 07:50 UTC: Added host call model, capability bindings, shared guard function, budget accounting, and denial abuse tracking to `crates/botforge-runtime/src/lib.rs`.
  - 2026-05-19 07:51 UTC: Added abuse-case test suite for spam intents, oversized payloads, repeated denied calls, timeout, and cancellation.
  - 2026-05-19 07:52 UTC: Ran full `cargo test` and verified all tests pass.
- **Commands run:**
  - `rg --files`
  - `sed -n '1,260p' crates/botforge-runtime/src/lib.rs`
  - `sed -n '1,260p' crates/botforge-runtime/tests/policy_engine.rs`
  - `cargo test`
- **Result:** Runtime now has deterministic Chamber host-call guardrails and coverage for core abuse cases; spec/checklist published under `docs/design`.

## 2026-05-19 08:05 UTC — Retention governance policy and immutable audit-linked schema migration (Complete)
- **Status:** Complete
- **Context:** Added governance/security retention policy documentation and SQLite migration tables for policy metadata, legal holds, removal exceptions, approvals, and immutable parent-audit linkage.
- **Attempt log:**
  - 2026-05-19 08:05 UTC: Reviewed existing repository structure and identified absence of governance policy docs and SQL migrations.
  - 2026-05-19 08:08 UTC: Added `docs/governance/retention-governance-security-policy.md` covering default permanence, override authority/process, legal hold/removal procedures, custody metadata changes, and approval thresholds.
  - 2026-05-19 08:11 UTC: Added `crates/botforge-runtime/migrations/0001_retention_governance.sql` with retention policy, edit, approval, legal hold, and custody exception tables plus immutable parent audit triggers.
  - 2026-05-19 08:12 UTC: Verified SQL migration syntax with `sqlite3` in-memory execution and reran `cargo test` successfully.
- **Commands run:**
  - `sqlite3 :memory: ".read crates/botforge-runtime/migrations/0001_retention_governance.sql"`
  - `cargo test`
- **Result:** Governance policy doc and schema migration implemented with immutable audit-link enforcement and passing validation/tests.

## 2026-05-19 08:30 UTC — Missing implementation roadmap split gates (Complete)
- **Status:** Complete
- **Context:** User requested explicit repository extraction gates in `docs/implementation-roadmap.md`, including spec stability, coverage/CI thresholds, release/changelog discipline, ownership/boundary checks, migration runbook, and reusable checklist template.
- **Attempt log:**
  - 2026-05-19 08:27 UTC: Checked root guidance and continuity files (`AGENTS.md`, `ISSUES.md`, `LEFTOVERS.md`) and confirmed no unresolved blockers.
  - 2026-05-19 08:28 UTC: Verified that `docs/implementation-roadmap.md` did not exist.
  - 2026-05-19 08:30 UTC: Authored new roadmap file with five explicit split gates, runbook steps, and a pre-extraction checklist template.
- **Commands run:**
  - `rg --files -g 'AGENTS.md' -g 'AGENTS.override.md'`
  - `cat AGENTS.md`
  - `cat ISSUES.md && cat LEFTOVERS.md`
  - `sed -n '1,260p' docs/implementation-roadmap.md`
  - `rg --files`
- **Result:** Requested split-gate roadmap documentation added and ready for future extraction PRs.

## 2026-05-19 09:15 UTC — Root governance docs and runtime/spec/storage scaffolding alignment (Complete)
- **Status:** Complete
- **Context:** AGENTS.md was oversized/redundant and missing concise continuity instructions; runtime/spec/storage tasks required architecture-aligned scaffolds and tests.
- **Attempt log:**
  - 2026-05-19 09:16 UTC: Replaced root `AGENTS.md` with concise binding architecture and continuity workflow, plus mandatory canonical-doc reading rule.
  - 2026-05-19 09:18 UTC: Added canonical design docs for runtime/platform/Rampart and migration law under `docs/design/`.
  - 2026-05-19 09:21 UTC: Expanded `botforge-spec` with envelope/manifest schema types, examples, and validation tests.
  - 2026-05-19 09:24 UTC: Implemented `botforge-runtime` Gate/Court/Chamber skeleton and architecture boundary tests.
  - 2026-05-19 09:26 UTC: Added SQLite core storage migration scaffold with required logical tables.
  - 2026-05-19 09:27 UTC: Updated README/CHANGELOG/architecture docs and ran full cargo test suite.
- **Commands run:**
  - `cargo test`
- **Result:** Requested foundational scaffolding and documentation alignment completed with passing tests.

## 2026-05-19 10:05 UTC — BotForge schema/runtime/storage/doc audit and hardening sweep (Complete)
- **Status:** Complete
- **Context:** User requested full audit/implementation pass for AGENTS/design-law alignment, `botforge-spec` stabilization, runtime lifecycle/boundary scaffolding, and validation fixtures.
- **Attempt log:**
  - 2026-05-19 09:45 UTC: Reviewed AGENTS/ISSUES/LEFTOVERS plus README/CHANGELOG and canonical docs before editing.
  - 2026-05-19 09:50 UTC: Reworked `botforge-spec` types to include richer envelope/manifest/setup schema fields and validation error taxonomy.
  - 2026-05-19 09:58 UTC: Added success/failure schema validation tests and updated manifest examples.
  - 2026-05-19 10:02 UTC: Extended runtime skeleton with lifecycle transition guards and test coverage for impossible activation jumps.
  - 2026-05-19 10:04 UTC: Updated README/CHANGELOG and ran full `cargo test` with all tests passing.
- **Commands run:**
  - `cargo test`
- **Result:** Requested schema/runtime scaffolding and validation coverage are implemented and verified; no blockers remain.

## 2026-05-19 11:20 UTC — Capability routing + platform contract skeleton (Incomplete)
- **Status:** Incomplete
- **Context:** User requested broad implementation including capability system, typed routing, platform module registry/contracts, multiple platform module scaffolds, and Chamber runtime hardening.
- **Attempt log:**
  - 2026-05-19 11:21 UTC: Reworked `botforge-spec` to split typed event/intent envelopes and expanded scoped capability grants.
  - 2026-05-19 11:24 UTC: Implemented runtime Court validation/routing with audit logging and scope mismatch/unknown capability denial.
  - 2026-05-19 11:28 UTC: Added Rust `PlatformModule` trait and `PlatformRegistry` activation with required secret checks.
  - 2026-05-19 11:30 UTC: Added mock architecture tests and platform docs placeholders for discord/reddit/spotify/youtube/rss/webhook.
- **Commands run:**
  - `cargo test`
- **Result:** Core skeleton landed and tests pass, but full Wasmtime integration and complete platform module crates/tests remain pending in `LEFTOVERS.md`.

## 2026-05-19 08:57 UTC — Capability routing + platform contract skeleton (Incomplete)
- **Status:** Incomplete
- **Context:** Continued from existing incomplete chain; implemented initial setup/config staging engine scaffolding to unblock BotForge-rendered setup migration path.
- **Attempt log:**
  - 2026-05-19 08:57 UTC: Added setup schema pages and supported interactive field types in `botforge-spec`.
  - 2026-05-19 08:57 UTC: Added runtime `SetupEngine` for staged config, diff preview, promotion, rollback, and secret masking.
  - 2026-05-19 08:57 UTC: Added runtime tests for setup staging lifecycle and audit records.
- **Commands run:**
  - `cargo test`
- **Result:** Partial progress only; major requested tasks remain and are tracked in `LEFTOVERS.md`.

## 2026-05-19 12:10 UTC — Cryer campaign storage/setup migration scaffold (Incomplete)
- **Status:** Incomplete
- **Context:** Requested large Cryer migration from Node/Express/CLI/JSON into BotForge Python/WASM + Rust platform module boundaries. Session implemented storage/setup contract scaffolding only.
- **Attempt log:**
  - 2026-05-19 12:00 UTC: Reviewed AGENTS/ISSUES/LEFTOVERS and canonical design docs before edits.
  - 2026-05-19 12:03 UTC: Added SQLite migration `0003_cryer_campaigns.sql` with requested Cryer campaign/state tables.
  - 2026-05-19 12:06 UTC: Added Cryer canonical handler/intent/setup constants in `botforge-spec` and coverage tests.
  - 2026-05-19 12:08 UTC: Added design doc `docs/design/cryer-campaign-storage-and-setup.md` and updated README/CHANGELOG.
  - 2026-05-19 12:09 UTC: Verified with `cargo test` and SQLite migration parse checks.
- **Commands run:**
  - `sqlite3 :memory: ".read crates/botforge-runtime/migrations/0003_cryer_campaigns.sql"`
  - `cargo test`
- **Result:** Storage/setup foundation added; full Cryer runtime handlers, Python/WASM implementation, scheduler/runner/removal monitor/report flows remain pending.

## 2026-05-22 00:40 UTC — Squire/Bard feature modules lacked full behavior parity (Complete)
- **Status:** Complete
- **Context:** Prior commit only added minimal scaffolding; requested implementation needed full module functionality for individual migration behavior coverage.
- **Attempt log:**
  - 2026-05-22 00:32 UTC: Reviewed existing feature scaffolds and identified missing embed/experience/rainbow/setup helpers and Bard file-backed flows.
  - 2026-05-22 00:37 UTC: Implemented complete Squire feature set and Bard module structs with deterministic queue/log behavior and optional template support.
  - 2026-05-22 00:39 UTC: Added expanded tests for persistence, threshold logic, summaries, relay ledgers, and queue file outputs; ran `cargo test -p botforge-runtime`.
- **Commands run:**
  - `cargo test -p botforge-runtime`
- **Result:** Feature modules now provide concrete behavior implementations aligned with migration specification and pass runtime tests.

## 2026-05-23 00:30 UTC — PR #11 follow-up: Bard queue transport regression + Squire XP divide-by-zero (Complete)
- **Status:** Complete
- **Context:** Codex review flagged two P1 regressions in feature modules: Bard wrote transport payloads directly to `Discovery/gateway_queue.log`, and Squire `ExperienceTracker::new(0, ...)` allowed divide-by-zero in `award_xp`/`summary`.
- **Attempt log:**
  - 2026-05-23 00:24 UTC: Inspected `crates/botforge-runtime/src/features/bard/mod.rs`, `.../squire/mod.rs`, and `tests/feature_modules.rs` to confirm queue-file writes and unchecked `level_scale` constructor input.
  - 2026-05-23 00:27 UTC: Refactored Bard feature module to remove queue-file transport writes and keep outputs as runtime-intent-ready payload values while retaining deterministic local log traces.
  - 2026-05-23 00:28 UTC: Hardened `ExperienceTracker::new` to clamp level scale to `>= 1` and added a regression test for zero-scale construction.
  - 2026-05-23 00:29 UTC: Updated docs/changelog verbiage and executed runtime feature test suite to verify behavior.
- **Commands run:**
  - `sed -n '1,220p' crates/botforge-runtime/src/features/bard/mod.rs`
  - `sed -n '1,260p' crates/botforge-runtime/src/features/squire/mod.rs`
  - `sed -n '1,260p' crates/botforge-runtime/tests/feature_modules.rs`
  - `cargo test -p botforge-runtime --test feature_modules`
- **Result:** Both P1 issues resolved; no queue-file transport dependency remains in Bard feature helpers and zero-scale XP inputs no longer panic.
