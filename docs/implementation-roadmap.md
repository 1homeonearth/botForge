# BotForge Implementation Roadmap and Repository Split Gates

This roadmap defines explicit extraction gates for splitting the current workspace into standalone repositories. No crate is extracted until all gates below are met and the extraction checklist is complete.

## 1) `botforge-spec` API/Schema Stability Thresholds

`botforge-spec` is the contract root for runtime, platform modules, and bot SDKs, so split readiness requires a measurable stability window.

### Stability thresholds
- **Schema freeze window:** no breaking change to exported structs/enums/field semantics for **two consecutive release cycles**.
- **Versioning discipline:**
  - Breaking schema/API changes require a **major** bump.
  - Additive backward-compatible fields/capabilities require a **minor** bump.
  - Clarifications/bugfixes require a **patch** bump.
- **Compatibility tests:** all golden schema fixtures and compatibility tests must pass for current and previous supported minor versions.
- **Deprecation runway:** any field or enum variant planned for removal must be marked deprecated for at least **one minor release** before removal.
- **Migration metadata:** each change affecting serialized artifacts must include migration notes and version mapping table updates in docs.

### Split gate decision for `botforge-spec`
`botforge-spec` can be extracted only when all thresholds above are true for the latest release candidate and verified in CI artifacts.

## 2) Minimum Test Coverage and CI Pass History per Crate

Every crate proposed for extraction must demonstrate baseline reliability before becoming independently versioned.

### Required minimums
- **Line coverage:** at least **80%** per crate.
- **Critical-path coverage:** at least **90%** for security/policy/authz/intent validation modules.
- **Test layers present:**
  - Unit tests for core logic.
  - Integration tests for cross-crate contract usage.
  - Regression tests for previously fixed defects.
- **CI pass streak:** at least **20 consecutive successful CI runs** on the default branch with no flaky-test quarantine exceptions.
- **Matrix health:** Linux stable toolchain and minimum supported Rust version jobs must pass for the same 20-run window.

### Evidence required
- Coverage report artifact link.
- CI history snapshot proving pass streak.
- List of known flaky tests (must be empty to pass split gate).

## 3) Release Cadence and Changelog Discipline Requirements

A repository split is blocked unless release operations are predictable and auditable.

### Release cadence expectations
- Maintain a documented release train cadence (recommended: **biweekly** stable cut).
- Emergency patch releases allowed but must follow normal changelog and tagging rules.

### Changelog discipline
- Every merged behavior change must include changelog entries covering:
  - Date.
  - Changed files/modules.
  - Behavior impact.
  - Security impact.
  - Migration notes.
  - Tests executed.
  - Docs updated.
- Changelog entries must be grouped by release tag and link back to merged PRs/commits.
- No extraction if changelog entries are missing for any merged change in the freeze window.

## 4) Ownership and Dependency Boundary Checks

Extraction candidates must not leak forbidden coupling across crate boundaries.

### Ownership requirements
- Each crate must have at least **one primary owner** and **one backup owner** listed in CODEOWNERS or equivalent ownership manifest.
- Owners must approve extraction checklist sign-off.

### Dependency boundary policy
- `botforge-spec` must remain dependency-light and must not import runtime/platform implementation crates.
- `botforge-runtime` may depend on `botforge-spec` but must not import bot-specific logic crates.
- Platform crates may depend on `botforge-spec` and runtime extension contracts only; no direct coupling to unrelated platform crates.
- Bot template/SDK crates must consume public contracts only; no private internal module imports.

### Boundary enforcement checks
- Run dependency graph checks (e.g., `cargo tree` policy scripts) to ensure no forbidden edge exists.
- Enforce lint/policy checks in CI for disallowed crate-to-crate imports.
- No extraction if any forbidden cross-crate edge is detected.

## 5) Migration Runbook: Tags/History Preservation and `botforge-suite` Lockfile Pinning

When a crate passes split gates, use this runbook to preserve provenance and keep ecosystem pinning deterministic.

### Repository extraction runbook
1. **Freeze window start**
   - Announce extraction freeze branch and stop non-critical feature merges.
2. **History-preserving split**
   - Use history-preserving extraction strategy (`git subtree split` or equivalent) for target crate paths.
   - Verify commit ancestry continuity and authored-attribution integrity.
3. **Tag migration**
   - Recreate or map release tags in new repo with documented old->new tag correspondence.
   - Verify signed tags where policy requires signatures.
4. **CI bootstrap in new repo**
   - Port required CI jobs, coverage gates, lint rules, and policy checks.
5. **Workspace dependency rewiring**
   - Replace local path dependencies with versioned git/crates.io refs according to release policy.
6. **`botforge-suite` lockfile pinning**
   - Pin extracted repos by immutable refs (tag + commit SHA) in `botforge-suite` lockfile.
   - Record compatibility matrix between runtime/spec/platform versions.
7. **Post-extraction verification**
   - Run full integration test matrix from `botforge-suite` against pinned refs.
   - Confirm reproducible checkout/build from lockfile only.

---

## Pre-Extraction Checklist Template (Use Before Every Repo Split)

Copy this template into the extraction PR description and complete all items.

### Metadata
- [ ] Target crate/repo:
- [ ] Planned new repo name:
- [ ] Proposed version/tag:
- [ ] Primary owner:
- [ ] Backup owner:
- [ ] Freeze window start date:

### Gate 1 — API/Spec Stability
- [ ] No breaking `botforge-spec` contract changes in the last 2 release cycles.
- [ ] Versioning rule compliance verified (major/minor/patch semantics).
- [ ] Compatibility/golden tests pass for supported minor versions.
- [ ] Deprecation runway satisfied for removals.
- [ ] Migration metadata/docs updated.

### Gate 2 — Quality and CI
- [ ] Coverage >= 80% for crate.
- [ ] Critical modules coverage >= 90%.
- [ ] Unit + integration + regression tests present.
- [ ] 20 consecutive successful CI runs verified.
- [ ] No known flaky tests.

### Gate 3 — Release Hygiene
- [ ] Release cadence followed during freeze window.
- [ ] Changelog complete for every merged behavior change.
- [ ] Security impact and migration notes present where applicable.

### Gate 4 — Ownership and Boundaries
- [ ] Primary + backup owners assigned.
- [ ] Dependency graph check run and archived.
- [ ] No forbidden cross-crate coupling found.
- [ ] CI boundary policy checks enabled and passing.

### Gate 5 — Extraction/Migration Runbook
- [ ] History-preserving split command and output logged.
- [ ] Tag mapping old->new documented.
- [ ] CI bootstrapped in new repo.
- [ ] Parent workspace dependencies rewired.
- [ ] `botforge-suite` lockfile pinned to immutable refs.
- [ ] Full integration matrix passed using pinned refs.

### Final Sign-off
- [ ] Owners approved extraction.
- [ ] Runtime/platform maintainers approved extraction.
- [ ] Rollback plan documented.
- [ ] Extraction date scheduled.
