use botforge_spec::Envelope;

#[derive(Debug, Clone)]
pub struct Gate;
#[derive(Debug, Clone)]
pub struct Court;
#[derive(Debug, Clone)]
pub struct Chamber;

#[derive(Debug, Clone)]
pub struct NormalizedEvent(pub Envelope);
#[derive(Debug, Clone)]
pub struct Intent(pub Envelope);

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
        !intent.0.capabilities_used.is_empty()
    }
}

impl Chamber {
    pub fn accept_event(&self, _event: &NormalizedEvent) -> bool { true }

    pub fn reject_raw_platform_event(&self, _raw: &str) -> bool { false }
}

pub struct Storage;
impl Storage {
    pub fn config_write(&self, _k: &str, _v: &str) -> bool { true }
    pub fn state_write(&self, _k: &str, _v: &str) -> bool { true }
    pub fn audit_write(&self, _entry: &str) -> bool { true }
    pub fn platform_status_write(&self, _platform: &str, _status: &str) -> bool { true }
    pub fn evidence_metadata_write(&self, _case_id: &str, _metadata: &str) -> bool { true }
}
