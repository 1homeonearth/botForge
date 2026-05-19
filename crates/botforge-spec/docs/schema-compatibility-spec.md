# BotForge Spec: Event/Intent Schema Compatibility (Normative)

## 1. Scope and status

This section is **normative** for all `event/*` and `intent/*` payload schemas published by `botforge-spec`.
Implementations **MUST** follow these rules for versioning, decoding, validation, and migration signaling.

## 2. Semantic version rules for payload schemas

Each payload schema **MUST** declare a semantic version `MAJOR.MINOR.PATCH`.

- `PATCH` changes **MUST NOT** change structural shape or validation requirements.
- `MINOR` changes **MUST** be backward-compatible for decoders built against earlier minor versions in the same major line.
- `MAJOR` changes **MAY** break compatibility and **MUST** be treated as a new compatibility line.

A schema version applies to the payload contract itself, not transport wrappers.

## 3. Compatibility guarantees by bump type

### 3.1 Patch bump (`x.y.z -> x.y.z+1`)

Producers and consumers **MUST** remain mutually compatible without behavioral changes. Permitted patch changes include:

- Clarifying descriptions/comments.
- Tightening non-semantic metadata (examples, docs).
- Bug fixes that do not alter required fields, field types, enum members, or validation boundaries.

### 3.2 Minor bump (`x.y.z -> x.y+1.0`)

Minor changes **MUST** be non-breaking to existing consumers in the same major line. Permitted minor changes include:

- Adding optional fields.
- Adding enum members only when consumers are required to safely ignore unknown members, or when fallback behavior is defined.
- Expanding validation in a way that accepts all previously valid payloads.

Minor changes **MUST NOT** remove required fields, narrow accepted value domains, or change field types for existing fields.

### 3.3 Major bump (`x.y.z -> x+1.0.0`)

Major bumps are required for any breaking change, including:

- Removing fields.
- Converting optional fields to required.
- Changing field types or semantic meaning.
- Narrowing validation so previously valid payloads become invalid.

## 4. Runtime behavior on schema-version mismatch

For any event/intent exchange, runtime behavior is:

1. **Accept directly** when producer and consumer schema major versions match and payload validates.
2. **Shim path** MAY be used only when a registered, deterministic shim exists for `(from_version -> to_version)` and shim validation succeeds.
3. **Staged compatibility mode** MAY be used when policy enables temporary dual-read/dual-validate operation for controlled rollout.
4. **Reject** in all other cases.

### 4.1 Required rejection cases

Runtime **MUST reject** with an auditable error code when:

- Major versions differ and no approved shim exists.
- Payload fails target schema validation.
- Manifest compatibility bounds are not satisfied.

### 4.2 Staged compatibility mode constraints

When staged mode is enabled, runtime **MUST**:

- Emit audit records for every shimmed/dual-validated payload.
- Enforce an expiration window for staged mode.
- Support immediate rollback to strict mode.

## 5. Manifest migration metadata requirements

Every bot/module manifest that declares event or intent dependencies **MUST** include:

- `spec_version`: the BotForge spec version used at build time.
- `runtime_version_min`: minimum runtime version required.
- `schema_compat` block with:
  - `events` and `intents` entries keyed by schema id.
  - per-entry constraints: `min_supported`, `max_supported`.
  - optional `requires_shim` list for explicit migration edges.

Example:

```toml
spec_version = "1.4.0"
runtime_version_min = "0.9.0"

[schema_compat.events."event.guild.member_joined"]
min_supported = "1.2.0"
max_supported = "1.5.x"

[schema_compat.intents."intent.moderation.warn_user"]
min_supported = "2.0.0"
max_supported = "2.x"
requires_shim = ["2.0.0->2.1.0"]
```

Runtime **MUST** block activation when required metadata is absent or version constraints are unsatisfied.

## 6. Golden compatibility tests (required)

`botforge-spec` and runtime integration suites **MUST** maintain golden fixtures covering both decode and validation behavior.

Required golden sets:

- Old producer -> new consumer (forward compatibility where allowed).
- New producer -> old consumer (backward compatibility expectations).
- Shimmed decode path fixtures.
- Rejection fixtures for unsupported major mismatches.

Each fixture **MUST** assert:

- Decode result (`ok`/`error`).
- Validation result (`valid`/`invalid`).
- Runtime action (`accept`/`shim`/`staged`/`reject`).

## 7. Change examples

### 7.1 Non-breaking example (minor)

`event.guild.member_joined` v`1.2.0` adds optional field:

- Added `invite_code: string | null` (optional, default `null`).

Why non-breaking:

- Existing payloads remain valid.
- Older consumers can ignore unknown optional field.

Version bump required: `1.1.3 -> 1.2.0`.

### 7.2 Breaking example (major)

`intent.moderation.warn_user` changes:

- `reason` type from `string` to structured object `{ code: string, detail: string }`.

Why breaking:

- Existing consumers expecting string cannot decode new object shape.

Version bump required: `2.4.1 -> 3.0.0`.
Runtime action without shim: **reject**.
