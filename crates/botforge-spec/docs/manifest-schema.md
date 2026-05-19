# BotForge Manifest Schema: Compatibility and Migration Fields (Normative)

This section defines required manifest fields for schema compatibility governance.

## Required fields

- `spec_version` (semver string, required)
- `runtime_version_min` (semver string, required)
- `schema_compat` (table, required when events/intents are declared)

### `schema_compat` structure

- `schema_compat.events.<schema_id>.min_supported` (required)
- `schema_compat.events.<schema_id>.max_supported` (required)
- `schema_compat.events.<schema_id>.requires_shim` (optional array)
- `schema_compat.intents.<schema_id>.min_supported` (required)
- `schema_compat.intents.<schema_id>.max_supported` (required)
- `schema_compat.intents.<schema_id>.requires_shim` (optional array)

## Validation rules

1. `spec_version` and `runtime_version_min` **MUST** parse as semantic versions.
2. `min_supported` **MUST** be less than or equal to `max_supported`.
3. `requires_shim` entries **MUST** use `from->to` format where both sides are semantic versions.
4. Major-range wildcards (for example `2.x`) are allowed only for `max_supported`.
5. Missing required compatibility fields **MUST** fail manifest validation.

## Runtime activation outcomes

- If manifest compatibility validates and versions intersect runtime capabilities: activation may proceed.
- If only shimmed compatibility is possible and shim is installed: activation enters staged compatibility mode or direct shim mode per policy.
- If no valid path exists: activation is rejected with migration guidance.
