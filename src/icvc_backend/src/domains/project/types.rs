use std::fmt;

use candid::{CandidType, Deserialize};

use crate::{domains::step::types::StepPhaseId, StepPhase, UserId};

pub type ProjectId = u64;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectCreate {
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub categories: Vec<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectUpdate {
    pub title: Option<String>,
    pub moto: Option<String>,
    pub description: Option<String>,
    pub team_members: Option<Vec<TeamMember>>,
    pub links: Option<Vec<Link>>,
    pub categories: Vec<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: ProjectId,
    pub user_id: UserId,
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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectAndStepPhase {
    pub id: ProjectId,
    pub user_id: UserId,
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub current_phase: StepPhaseId,
    pub status: ProjectStatus,
    pub created_at: u64,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
    pub step_phase: StepPhase,
}

#[derive(CandidType, Deserialize, PartialEq, Clone, Debug)]
pub enum ProjectStatus {
    Open,
    Funded,
    NotFunded,
    NotSubmitted,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProjectStatus::Open => write!(f, "Open"),
            ProjectStatus::NotFunded => write!(f, "Not Funded"),
            ProjectStatus::Funded => write!(f, "Funded"),
            ProjectStatus::NotSubmitted => write!(f, "Not Submitted"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Link {
    pub kind: String,
    pub url: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TeamMember {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub previous_experience: String,
    pub links: Vec<Link>,
    pub profile_picture: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UploadFile {
    pub filename: Option<String>,
    pub document_type: DocumentType,
    pub s3_key: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum DocumentType {
    PitchDeck,
    Logo,
    CoverPhoto,
    FinancialModels,
    ProductDemo,
    ExpenditurePlan,
}

impl ToString for DocumentType {
    fn to_string(&self) -> String {
        match self {
            DocumentType::PitchDeck => "PitchDeck".to_string(),
            DocumentType::Logo => "Logo".to_string(),
            DocumentType::CoverPhoto => "CoverPhoto".to_string(),
            DocumentType::FinancialModels => "FinancialModels".to_string(),
            DocumentType::ProductDemo => "ProductDemo".to_string(),
            DocumentType::ExpenditurePlan => "ExpenditurePlan".to_string(),
        }
    }
}
