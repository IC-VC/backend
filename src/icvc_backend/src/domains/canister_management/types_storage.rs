use candid::{CandidType, Deserialize, Principal};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

const CANISTER_CONFIG_MAX_SIZE: u32 = 64000;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CanisterConfig {
    pub owner: Option<Principal>,
    pub sns_governance_id: Option<Principal>,
    pub subaccount: Option<String>,
    pub max_stable_memory_size: Option<u64>,
}

impl Storable for CanisterConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: CANISTER_CONFIG_MAX_SIZE,
        is_fixed_size: false,
    };
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            owner: None,
            sns_governance_id: None,
            subaccount: None,
            max_stable_memory_size: Some(0),
        }
    }
}
