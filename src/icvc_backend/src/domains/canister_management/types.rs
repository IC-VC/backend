use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub sns_governance_id: Option<Principal>,
    pub subaccount: Option<String>,
    pub max_stable_memory_size: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CanisterConfigUpdate {
    pub sns_governance_id: Option<Principal>,
    pub subaccount: Option<String>,
    pub max_stable_memory_size: Option<u64>,
}
