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
