use botforge_spec::{Envelope, IntentEnvelope};

#[derive(Debug, Clone)]
pub struct Gate;
#[derive(Debug, Clone)]
pub struct Court;
#[derive(Debug, Clone)]
pub struct Chamber;

#[derive(Debug, Clone)]
pub struct NormalizedEvent(pub Envelope);
#[derive(Debug, Clone)]
pub struct Intent(pub IntentEnvelope);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleLifecycleState {
    Discovered,
    ManifestInvalid,
    ManifestValid,
    BuildPending,
    BuildFailed,
    Built,
    ImportInspectionFailed,
    TestPending,
    TestFailed,
    CapabilityReviewPending,
    SetupPending,
    Staged,
    Active,
    Suspended,
    Deactivated,
    OrphanedState,
    Archived,
}

impl Gate {
    pub fn normalize_platform_event(&self, raw: &str) -> Option<NormalizedEvent> {
        serde_json::from_str::<Envelope>(raw).ok().map(NormalizedEvent)
    }

    pub fn execute_intent(&self, intent: &Intent, approved: bool) -> bool {
        approved && !intent.0.r#type.is_empty()
    }
}

impl Court {
    pub fn validate_intent(&self, intent: &Intent) -> bool {
        !intent.0.capabilities_used.is_empty() && intent.0.spec == botforge_spec::SPEC_VERSION
    }

    pub fn transition_allowed(from: ModuleLifecycleState, to: ModuleLifecycleState) -> bool {
        use ModuleLifecycleState::*;
        matches!((from, to),
            (Discovered, ManifestInvalid) | (Discovered, ManifestValid) |
            (ManifestValid, BuildPending) | (BuildPending, BuildFailed) | (BuildPending, Built) |
            (Built, TestPending) | (TestPending, TestFailed) | (TestPending, CapabilityReviewPending) |
            (CapabilityReviewPending, SetupPending) | (SetupPending, Staged) | (Staged, Active) |
            (Active, Suspended) | (Suspended, Active) | (Active, Deactivated) | (Deactivated, Archived)
        )
    }
}

impl Chamber {
    pub fn accept_event(&self, _event: &NormalizedEvent) -> bool { true }
    pub fn reject_raw_platform_event(&self, _raw: &str) -> bool { false }
}
