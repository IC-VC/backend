use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{ProjectId, StepPhaseId};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectProposalPayload {
    pub project_id: ProjectId,
    pub phase_id: StepPhaseId,
}
