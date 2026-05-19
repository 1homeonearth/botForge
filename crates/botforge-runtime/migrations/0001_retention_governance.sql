PRAGMA foreign_keys = ON;

BEGIN;

CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    occurred_at TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_id TEXT NOT NULL,
    details_json TEXT NOT NULL,
    integrity_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS retention_policies (
    policy_id TEXT PRIMARY KEY,
    policy_key TEXT NOT NULL UNIQUE,
    scope_kind TEXT NOT NULL,
    scope_id TEXT,
    retention_mode TEXT NOT NULL CHECK (retention_mode IN ('permanent','duration','legal_hold')),
    retention_days INTEGER,
    legal_basis TEXT,
    is_default INTEGER NOT NULL CHECK (is_default IN (0,1)) DEFAULT 0,
    effective_from TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    CHECK ((retention_mode = 'duration' AND retention_days IS NOT NULL AND retention_days > 0)
        OR (retention_mode IN ('permanent','legal_hold') AND retention_days IS NULL))
);

CREATE TABLE IF NOT EXISTS retention_policy_edits (
    edit_id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL,
    requested_by TEXT NOT NULL,
    reason_code TEXT NOT NULL,
    rationale TEXT NOT NULL,
    old_value_json TEXT NOT NULL,
    new_value_json TEXT NOT NULL,
    approval_threshold INTEGER NOT NULL CHECK (approval_threshold >= 1),
    approved_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('pending','approved','denied','applied')),
    parent_audit_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    applied_at TEXT,
    applied_by TEXT,
    FOREIGN KEY(policy_id) REFERENCES retention_policies(policy_id),
    FOREIGN KEY(parent_audit_id) REFERENCES audit_log(id)
);

CREATE TABLE IF NOT EXISTS retention_policy_approvals (
    approval_id TEXT PRIMARY KEY,
    edit_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approver_role TEXT NOT NULL,
    approved_at TEXT NOT NULL,
    decision TEXT NOT NULL CHECK (decision IN ('approve','deny')),
    notes TEXT,
    FOREIGN KEY(edit_id) REFERENCES retention_policy_edits(edit_id)
);

CREATE TABLE IF NOT EXISTS legal_holds (
    hold_id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL,
    case_reference TEXT NOT NULL,
    jurisdiction TEXT NOT NULL,
    authority_reference TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('active','released')),
    imposed_at TEXT NOT NULL,
    imposed_by TEXT NOT NULL,
    released_at TEXT,
    released_by TEXT,
    parent_audit_id TEXT NOT NULL,
    release_audit_id TEXT,
    FOREIGN KEY(policy_id) REFERENCES retention_policies(policy_id),
    FOREIGN KEY(parent_audit_id) REFERENCES audit_log(id),
    FOREIGN KEY(release_audit_id) REFERENCES audit_log(id)
);

CREATE TABLE IF NOT EXISTS custody_exception_events (
    event_id TEXT PRIMARY KEY,
    exception_type TEXT NOT NULL CHECK (exception_type IN ('legal_hold','legal_hold_release','removal_exception','retention_override')),
    affected_kind TEXT NOT NULL,
    affected_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    legal_authority_ref TEXT,
    policy_edit_id TEXT,
    parent_audit_id TEXT NOT NULL,
    executed_by TEXT NOT NULL,
    executed_at TEXT NOT NULL,
    integrity_hash TEXT NOT NULL,
    FOREIGN KEY(policy_edit_id) REFERENCES retention_policy_edits(edit_id),
    FOREIGN KEY(parent_audit_id) REFERENCES audit_log(id)
);

CREATE TABLE IF NOT EXISTS custody_exception_approvals (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approver_role TEXT NOT NULL,
    approved_at TEXT NOT NULL,
    FOREIGN KEY(event_id) REFERENCES custody_exception_events(event_id)
);

CREATE TRIGGER IF NOT EXISTS trg_retention_policy_edits_parent_audit_immutable
BEFORE UPDATE OF parent_audit_id ON retention_policy_edits
FOR EACH ROW
WHEN NEW.parent_audit_id <> OLD.parent_audit_id
BEGIN
    SELECT RAISE(ABORT, 'parent_audit_id is immutable');
END;

CREATE TRIGGER IF NOT EXISTS trg_legal_holds_parent_audit_immutable
BEFORE UPDATE OF parent_audit_id ON legal_holds
FOR EACH ROW
WHEN NEW.parent_audit_id <> OLD.parent_audit_id
BEGIN
    SELECT RAISE(ABORT, 'parent_audit_id is immutable');
END;

CREATE TRIGGER IF NOT EXISTS trg_custody_exception_events_parent_audit_immutable
BEFORE UPDATE OF parent_audit_id ON custody_exception_events
FOR EACH ROW
WHEN NEW.parent_audit_id <> OLD.parent_audit_id
BEGIN
    SELECT RAISE(ABORT, 'parent_audit_id is immutable');
END;

COMMIT;
