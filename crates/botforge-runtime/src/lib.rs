pub mod features;
use std::collections::{BTreeMap, BTreeSet};
use botforge_spec::{CapabilityGrant, EventEnvelope, EventSource, EventTarget, IntentEnvelope, SPEC_VERSION};

#[derive(Debug, Clone, PartialEq, Eq)] pub enum RouteDecision { Routed, Denied(String) }
#[derive(Debug, Clone)] pub struct Gate; #[derive(Debug, Clone)] pub struct Court; #[derive(Debug, Clone)] pub struct Chamber;
#[derive(Debug, Clone)] pub struct RuntimeAudit { pub records: Vec<String> }
#[derive(Debug, Clone)] pub struct NormalizedEvent(pub EventEnvelope); #[derive(Debug, Clone)] pub struct Intent(pub IntentEnvelope);
#[derive(Debug, Clone, PartialEq, Eq)] pub enum ModuleLifecycleState { Discovered, Staged, Active, Suspended, Deactivated }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetupScope { ThisServer, SelectedServers, AllServers, FutureServers, Channel, ChannelGroup, RoleGroup, Bot, Feature, PlatformModule, Profile }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSnapshot { pub values: BTreeMap<String,String>, pub revision:u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedConfig { pub values:BTreeMap<String,String>, pub base_revision:u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupEngine { pub active:BTreeMap<SetupScope, ConfigSnapshot>, pub staged:BTreeMap<SetupScope, StagedConfig>, pub rollback:BTreeMap<SetupScope, Vec<ConfigSnapshot>>, pub audit:Vec<String> }
impl Default for SetupEngine { fn default()->Self{Self{active:BTreeMap::new(),staged:BTreeMap::new(),rollback:BTreeMap::new(),audit:vec![]}} }
impl SetupEngine {
    pub fn stage(&mut self, scope: SetupScope, values: BTreeMap<String,String>) { let base=self.active.get(&scope).map(|s|s.revision).unwrap_or(0); self.staged.insert(scope.clone(), StagedConfig{values,base_revision:base}); self.audit.push(format!("setup.stage:{scope:?}:r{base}")); }
    pub fn preview_diff(&self, scope:&SetupScope)->BTreeMap<String,(Option<String>,Option<String>)>{ let mut out=BTreeMap::new(); let cur=self.active.get(scope).map(|c|&c.values); let stg=self.staged.get(scope).map(|s|&s.values); let keys: BTreeSet<String> = cur.into_iter().flat_map(|m|m.keys().cloned()).chain(stg.into_iter().flat_map(|m|m.keys().cloned())).collect(); for k in keys { let l=cur.and_then(|m|m.get(&k).cloned()); let r=stg.and_then(|m|m.get(&k).cloned()); if l!=r { out.insert(k,(l,r)); } } out }
    pub fn promote(&mut self, scope:&SetupScope)->Result<(),String>{ let staged=self.staged.remove(scope).ok_or_else(||"no staged config".to_string())?; let prev=self.active.get(scope).cloned().unwrap_or(ConfigSnapshot{values:BTreeMap::new(),revision:0}); self.rollback.entry(scope.clone()).or_default().push(prev.clone()); self.active.insert(scope.clone(), ConfigSnapshot{values:staged.values, revision: prev.revision+1}); self.audit.push(format!("setup.promote:{scope:?}:r{}", prev.revision+1)); Ok(()) }
    pub fn rollback(&mut self, scope:&SetupScope)->Result<(),String>{ let hist=self.rollback.get_mut(scope).ok_or_else(||"no rollback history".to_string())?; let snap=hist.pop().ok_or_else(||"no rollback history".to_string())?; self.active.insert(scope.clone(), snap.clone()); self.audit.push(format!("setup.rollback:{scope:?}:r{}", snap.revision)); Ok(()) }
    pub fn masked_values(map:&BTreeMap<String,String>)->BTreeMap<String,String>{ map.iter().map(|(k,v)|(k.clone(), if k.contains("secret")||k.contains("token") {"********".into()} else {v.clone()})).collect() }
}

impl Gate { pub fn normalize_platform_event(&self, raw:&str, source_platform:&str, target:EventTarget)->Option<NormalizedEvent>{ let mut e=serde_json::from_str::<EventEnvelope>(raw).ok()?; e.spec=SPEC_VERSION.to_string(); e.source.platform=source_platform.to_string(); e.target=target; Some(NormalizedEvent(e)) }}
impl Court { pub fn validate_intent(intent:&Intent, grants:&[CapabilityGrant], source:&EventSource)->RouteDecision{ if intent.0.spec!=SPEC_VERSION{return RouteDecision::Denied("invalid spec".into());} for cap in &intent.0.capabilities_used { let allowed=grants.iter().any(|g|g.capability==*cap&&g.effect=="allow"&&g.platform_module.as_deref().map(|p|p==source.platform).unwrap_or(true)&&g.guild.as_deref().map(|x|Some(x)==source.guild.as_deref()).unwrap_or(true)&&g.channel.as_deref().map(|x|Some(x)==source.channel.as_deref()).unwrap_or(true)); if !allowed{return RouteDecision::Denied(format!("capability denied: {cap}"));}} RouteDecision::Routed }
pub fn route_intent(intent:&Intent,grants:&[CapabilityGrant],source:&EventSource,audit:&mut RuntimeAudit)->Result<EventEnvelope,String>{match Self::validate_intent(intent,grants,source){RouteDecision::Denied(r)=>{audit.records.push(format!("deny:{}:{}",intent.0.intent_id,r));Err(r)}RouteDecision::Routed=>{audit.records.push(format!("route:{}:{}",intent.0.intent_id,intent.0.r#type));Ok(EventEnvelope{spec:SPEC_VERSION.into(),event_id:format!("ev-from-{}",intent.0.intent_id),r#type:format!("court.{}",intent.0.r#type),timestamp:intent.0.timestamp.clone(),source:source.clone(),target:intent.0.from.clone(),actor:botforge_spec::Actor{actor_type:"court".into(),actor_id:"court".into()},payload:intent.0.payload.clone(),context:serde_json::json!({"inter_bot":true}),correlation_id:intent.0.requested_by_event.clone()})}}}}
impl Chamber { pub fn reject_forbidden_imports(imports:&[&str])->bool{let forbidden=["wasi:sockets","wasi:filesystem","wasi:clocks","wasi:cli","wasi:environment"];imports.iter().all(|i|!forbidden.iter().any(|f|i.contains(f)))}}

#[derive(Debug, Clone)] pub struct PlatformManifest { pub runtime_id:String,pub capabilities:Vec<String>,pub required_secrets:Vec<String>,pub allow_raw_network:bool }
#[derive(Debug, Clone, PartialEq, Eq)] pub enum PlatformError { MissingSecret(String), CapabilityUnknown(String), ExecuteFailed(String) }
pub trait PlatformModule { fn manifest(&self)->PlatformManifest; fn setup_schema(&self)->String; fn start_listener(&mut self)->Result<(),PlatformError>; fn stop_listener(&mut self)->Result<(),PlatformError>; fn execute_intent(&self,intent:&Intent)->Result<serde_json::Value,PlatformError>; fn normalize_event(&self,raw:serde_json::Value)->Result<EventEnvelope,PlatformError>; }
pub struct PlatformRegistry { pub modules:BTreeMap<String,Box<dyn PlatformModule>>, pub active:BTreeSet<String> }
impl Default for PlatformRegistry { fn default()->Self{Self{modules:BTreeMap::new(),active:BTreeSet::new()}} }
impl PlatformRegistry { pub fn register(&mut self,module:Box<dyn PlatformModule>)->Result<(),PlatformError>{ let manifest=module.manifest(); if !botforge_spec::validate_platform_name(&manifest.runtime_id){return Err(PlatformError::CapabilityUnknown(manifest.runtime_id));} self.modules.insert(manifest.runtime_id,module); Ok(()) }
 pub fn activate(&mut self,runtime_id:&str,configured_secrets:&[&str])->Result<(),PlatformError>{ let module=self.modules.get_mut(runtime_id).ok_or_else(||PlatformError::ExecuteFailed("missing module".into()))?; let manifest=module.manifest(); for secret in manifest.required_secrets { if !configured_secrets.contains(&secret.as_str()){return Err(PlatformError::MissingSecret(secret));}} module.start_listener()?; self.active.insert(runtime_id.into()); Ok(()) }}
