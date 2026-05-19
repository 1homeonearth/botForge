# Retention Governance and Security Policy

## 1) Default permanence policy and rationale

BotForge defaults to **permanent retention** for evidence, chain-of-custody records, and retention policy decisions.

Rationale:
- Evidence integrity for moderation, safety, and abuse investigations requires complete historical traceability.
- Appeals, legal disputes, and incident-response postmortems depend on durable records.
- Security events (tampering, override attempts, unauthorized access) must remain analyzable across long time horizons.
- Policy provenance and accountability require stable, immutable references from each exception back to approving records.

Default behavior:
- Evidence metadata and custody history are retained indefinitely.
- Policy overrides are exceptional and require explicit approval artifacts.
- No destructive retention edit is applied without an auditable approval chain.

## 2) Authorized roles/process for policy overrides

Retention overrides are permitted only through a controlled process.

Authorized roles:
- **Owner** (required approver)
- **Compliance Officer** (required co-approver)
- **Security Lead** (required co-approver)

Process requirements:
1. Open a formal retention override request with legal/security rationale.
2. Link all impacted record scopes and required time window.
3. Record structured risk assessment and compensating controls.
4. Gather approvals meeting threshold policy (see section 5).
5. Apply override through governed workflow that writes immutable audit links.
6. Run post-change verification and archive resulting evidence.

## 3) Legal hold and removal exception procedures

### Legal hold
- A legal hold supersedes normal retention or deletion pathways.
- Hold request must include case identifier, jurisdictional basis, scope, and initiating authority.
- Hold activation must block deletion/expiry jobs for impacted scope.
- Hold release requires equivalent approval rigor and explicit release record.

### Removal exceptions
- Removal exceptions are permitted only for legally compelled deletion, court order, or validated rights-based obligations.
- Each removal exception must include legal basis, scope, method, execution timestamp, and verifier attestations.
- When source evidence cannot remain, BotForge preserves non-content audit metadata sufficient to prove why and how removal occurred.

## 4) Chain-of-custody metadata changes for exceptions

When an exception is applied, chain-of-custody metadata must append:
- Exception type (`legal_hold`, `legal_hold_release`, `removal_exception`, `retention_override`)
- Affected record/evidence scope
- Reason and legal authority references
- Approver identities and approval threshold satisfied
- Immutable link to initiating policy audit record
- Execution actor, timestamp, and integrity hash

Chain-of-custody metadata is append-only; existing entries cannot be modified or deleted.

## 5) Required audit entries and approval thresholds for retention policy edits

Required audit entry fields for every retention policy edit:
- Unique edit identifier and policy key
- Old/new retention values
- Reason code and structured rationale
- Requestor identity
- Approval set (role, principal, timestamp)
- Applied timestamp and executor identity
- Immutable parent audit link (for override/exception chains)
- Integrity digest/signature metadata

Approval thresholds:
- **Default policy updates**: minimum 2 approvals including Owner + one of Compliance Officer/Security Lead.
- **Override/removal exception edits**: minimum 3 approvals including Owner + Compliance Officer + Security Lead.
- **Legal hold release**: minimum 3 approvals including Owner + Compliance Officer + Legal delegate.

Any edit lacking threshold approvals must be denied and logged as a failed governance attempt.
