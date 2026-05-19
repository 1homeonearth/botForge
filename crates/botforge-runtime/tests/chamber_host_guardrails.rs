use std::collections::HashSet;

use botforge_runtime::{GuardError, HostBudget, HostCall, InvocationGuard};

fn budget() -> HostBudget {
    HostBudget {
        max_host_calls: 16,
        max_denied_calls: 2,
        max_state_reads: 8,
        max_state_writes: 8,
        max_intents_emitted: 2,
        max_log_bytes: 64,
        max_payload_bytes: 32,
        cpu_ms_budget: 1_000,
        memory_bytes_budget: 4 * 1024 * 1024,
        deadline_ms: 1_000,
    }
}

fn caps(list: &[&str]) -> HashSet<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn spam_intents_hits_backpressure() {
    let mut guard = InvocationGuard::new(budget());
    let capabilities = caps(&["intent.emit"]);

    assert_eq!(
        guard.authorize_and_track_call(HostCall::EmitIntent, &capabilities, 8, 1),
        Ok(())
    );
    assert_eq!(
        guard.authorize_and_track_call(HostCall::EmitIntent, &capabilities, 8, 2),
        Ok(())
    );
    assert_eq!(
        guard.authorize_and_track_call(HostCall::EmitIntent, &capabilities, 8, 3),
        Err(GuardError::Backpressure)
    );
}

#[test]
fn oversized_payload_is_rejected() {
    let mut guard = InvocationGuard::new(budget());
    let capabilities = caps(&["runtime.log.write"]);

    assert_eq!(
        guard.authorize_and_track_call(HostCall::LogWrite, &capabilities, 33, 1),
        Err(GuardError::PayloadTooLarge)
    );
}

#[test]
fn repeated_denied_calls_trigger_abuse_detected() {
    let mut guard = InvocationGuard::new(budget());
    let capabilities = caps(&[]);

    assert_eq!(
        guard.authorize_and_track_call(HostCall::StateSet, &capabilities, 4, 1),
        Err(GuardError::Unauthorized)
    );
    assert_eq!(
        guard.authorize_and_track_call(HostCall::StateSet, &capabilities, 4, 2),
        Err(GuardError::Unauthorized)
    );
    assert_eq!(
        guard.authorize_and_track_call(HostCall::StateSet, &capabilities, 4, 3),
        Err(GuardError::AbuseDetected)
    );
}

#[test]
fn timeout_cancels_invocation() {
    let mut guard = InvocationGuard::new(budget());
    let capabilities = caps(&["metrics.emit"]);

    assert_eq!(
        guard.authorize_and_track_call(HostCall::MetricsEmit, &capabilities, 4, 1_001),
        Err(GuardError::TimedOut)
    );
    assert_eq!(
        guard.authorize_and_track_call(HostCall::MetricsEmit, &capabilities, 4, 1_002),
        Err(GuardError::Cancelled)
    );
}

#[test]
fn cancelled_invocation_rejects_all_calls() {
    let mut guard = InvocationGuard::new(budget());
    let capabilities = caps(&["event.ack"]);

    guard.cancel();
    assert_eq!(
        guard.authorize_and_track_call(HostCall::EventAck, &capabilities, 1, 1),
        Err(GuardError::Cancelled)
    );
}
