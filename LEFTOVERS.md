## Remaining work (2026-05-19)
- Implement real Rust crates for each platform module (`discord`, `reddit`, `spotify`, `youtube`, `rss`, `webhook`) with trait implementations, manifests (`botforge.platform.toml`), and comprehensive mock tests.
- Extend Chamber into real Wasmtime host bindings with capability-scoped host functions (`botforge.log`, `botforge.get_config`, state APIs, intent emit, timer, context, metrics, open_case, attach_evidence_ref) and resource limits/circuit breaker behavior.
- Add full docs updates under `docs/design/` for typed routing, platform contract, and migration implications.
- Add setup schema docs and dependency/compatibility report behavior in registry.
