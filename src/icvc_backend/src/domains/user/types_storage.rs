use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use crate::{UserId, UserNeuronId};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserModel {
    pub name: String,
    pub is_admin: bool,
}

impl Storable for UserModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserNeuronModel {
    pub user_id: UserId,
    pub neuron_id: UserNeuronId,
}

impl Storable for UserNeuronModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
