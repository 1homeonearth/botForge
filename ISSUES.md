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
