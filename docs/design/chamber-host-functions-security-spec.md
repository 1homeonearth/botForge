# Chamber Host Functions Security Spec and Implementation Checklist

## Scope
This specification defines the Chamber-facing host API that WebAssembly bots can call through BotForge Runtime. The goals are strict capability mediation, deterministic denial behavior, and bounded resource usage.

## 1) Enumerated host calls exposed to WASM modules

| Host call | Category | Description |
|---|---|---|
| `log_write` | Logging | Write structured bot logs into BotForge audit pipeline. |
| `state_get` | State I/O | Read namespaced bot state values from runtime-managed state store. |
| `state_set` | State I/O | Write namespaced bot state values into runtime-managed state store. |
| `state_delete` | State I/O | Delete namespaced bot state values. |
| `emit_intent` | Intent routing | Submit typed intents to Court for validation/authorization/execution. |
| `event_ack` | Event flow | Acknowledge completion of delivered event processing. |
| `metrics_emit` | Observability | Emit constrained custom metrics/counters for dashboards. |

No networking, filesystem, process execution, secret retrieval, environment variable access, or raw platform SDK calls are exposed.

## 2) Per-call authorization checks and capability bindings

All host calls run through `authorize_and_track_call` before execution.

| Host call | Required capability | Authorization checks |
|---|---|---|
| `log_write` | `runtime.log.write` | Module active; not cancelled; per-invocation budget remaining; payload bytes <= max payload. |
| `state_get` | `state.read` | Module active; namespace ownership check; key length and payload budget check. |
| `state_set` | `state.write` | Module active; namespace ownership check; value size <= max payload; write-op budget remaining. |
| `state_delete` | `state.write` | Module active; namespace ownership check; delete-op budget remaining. |
| `emit_intent` | `intent.emit` | Module active; intent schema id allowlisted in module manifest; payload size <= max payload; emission rate budget remaining. |
| `event_ack` | `event.ack` | Event delivery token exists and belongs to module invocation; ack budget remaining. |
| `metrics_emit` | `metrics.emit` | Metric name allowlisted; cardinality within bounds; payload bytes <= max payload. |

Denials are explicit, deterministic, and count toward denied-call abuse protections.

## 3) Resource budgets

Chamber enforces invocation-scoped budgets:

- **CPU budget**: `cpu_ms_budget` (wall-clock proxy) across one invocation.
- **Memory budget**: `memory_bytes_budget` for guest memory allocation ceiling.
- **Operation budget**: `max_host_calls` total host calls.
- **Denied-call budget**: `max_denied_calls` repeated denial threshold.
- **State I/O budgets**:
  - `max_state_reads`
  - `max_state_writes`
- **Intent budget**: `max_intents_emitted`
- **Log volume budget**: `max_log_bytes`
- **Per-call payload limit**: `max_payload_bytes`

Budget exhaustion transitions to fail-closed deny/cancel behavior.

## 4) Timeout, cancellation, and backpressure semantics

- **Timeout**:
  - Invocation has hard deadline (`deadline_ms`).
  - Any host call after deadline returns `TimedOut` and marks invocation cancelled.
- **Cancellation**:
  - Runtime or owner-triggered cancellation sets `cancelled=true`.
  - All subsequent calls return `Cancelled` with no side effects.
- **Backpressure**:
  - When host call budget, intent budget, or log volume budget is exhausted, Chamber returns `Backpressure`.
  - Callers should treat backpressure as non-retryable within the same invocation.
  - Denied repeats are counted and trigger abuse cut-off.

## 5) Abuse-case tests

Required test coverage:

1. **Spam intents**: repeated `emit_intent` beyond `max_intents_emitted` results in `Backpressure` and no additional accepted emissions.
2. **Oversized payloads**: any call with payload > `max_payload_bytes` returns `PayloadTooLarge`.
3. **Repeated denied calls**: capability-missing calls increment denied counter and eventually return `AbuseDetected` once `max_denied_calls` is crossed.
4. **Post-timeout calls**: after timeout, calls return `TimedOut` and invocation remains cancelled.
5. **Cancelled invocation**: all calls return `Cancelled` and consume no mutable budget except denial accounting.

## Implementation checklist

- [x] Define canonical host-call enum in runtime.
- [x] Bind each host call to a canonical capability string.
- [x] Add deterministic authorization + budget guard function.
- [x] Add invocation budget/accounting state model.
- [x] Implement timeout/cancellation checks in shared guard.
- [x] Implement abuse counters for repeated denied calls.
- [x] Add tests for spam intents, oversized payloads, repeated denied calls, timeout, and cancellation.
- [ ] Wire this guard into actual Wasmtime host bindings as each host function is implemented.
