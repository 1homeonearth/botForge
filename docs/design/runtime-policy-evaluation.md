# Runtime Policy Evaluation and Enforcement Order

## Canonical scope evaluation order

BotForge policy enforcement evaluates rules from least-specific to most-specific scope, with higher-specificity scopes taking final precedence:

1. global
2. platform
3. bot
4. feature
5. guild
6. channel
7. role
8. user

At decision time, the engine sorts applicable rules by descending scope precedence rank so user-scoped decisions can override broader defaults.

## Deny/allow precedence and exception rules

When two rules target the same effective scope, `deny` always outranks `allow`. This guarantees safe defaults for ambiguous or conflicting policy profiles.

Exception rules are modeled as higher-specificity allows (for example a user allow exception inside a broader guild deny). Because specificity rank is evaluated first, a user-level allow can override a guild deny. At the same scope, deny remains dominant.

## Tie-breaking for overlapping profiles and staged configurations

For deterministic outcomes when multiple rules overlap:

1. Higher scope specificity wins.
2. At equal scope, `deny` wins over `allow`.
3. At equal scope/effect, higher explicit `priority` wins.
4. Non-staged rules win over staged rules unless staged evaluation mode is explicitly enabled.
5. At equal rank after all prior criteria, lexicographically smaller `profile_id` wins.
6. Final tie-breaker is lexicographically smaller `rule_id`.

## Audit record rule path requirements

Every policy decision writes an audit record containing:

- final decision (allow/deny)
- winning rule id
- winning profile id
- exact ordered rule path string in canonical format:

`scope=<scope> > effect=<allow|deny> > priority=<n> > profile=<profile_id> > rule=<rule_id>`

When no rule applies, runtime emits a default deny with rule path `default=deny (no applicable rules)`.
