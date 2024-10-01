use candid::{CandidType, Deserialize, Principal};

pub type UserId = Principal;
pub type UserNeuronId = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserCreate {
    pub name: String,
    pub user_id: UserId,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserUpdate {
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: UserId,
    pub name: String,
    pub is_admin: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserNeuron {
    pub user_id: UserId,
    pub neuron_id: UserNeuronId,
}
