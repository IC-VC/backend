use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};

use crate::{domains::step::types::StepPhaseId, UserId};

use super::types::{Link, ProjectStatus, TeamMember};

pub type ProjectId = u64;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectModel {
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub categories: Vec<u64>,
    pub current_phase: StepPhaseId,
    pub status: ProjectStatus,
    pub created_at: u64,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
}

impl Storable for ProjectModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
