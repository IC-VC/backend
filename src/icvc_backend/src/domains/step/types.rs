use std::fmt;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{DocumentType, ProjectId, UploadFile, UserId, UserNeuronId};

pub type StepId = u64;
pub type StepPhaseId = u64;
pub type ProposalId = u64;

#[derive(CandidType, Deserialize, Debug)]
pub struct StepPhaseCreate {
    pub status: StepPhaseStatus,
    pub start_open_date: u64,
    pub end_open_date: u64,
    pub submit_date: Option<u64>,
    pub start_assessment_date: u64,
    pub end_assessment_date: u64,
    pub assessment_method: AssessmentMethod,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StepPhaseUpdate {
    pub status: Option<StepPhaseStatus>,
    pub start_open_date: Option<u64>,
    pub end_open_date: Option<u64>,
    pub submit_date: Option<u64>,
    pub start_assessment_date: Option<u64>,
    pub end_assessment_date: Option<u64>,
}

impl Default for StepPhaseUpdate {
    fn default() -> Self {
        StepPhaseUpdate {
            status: None,
            start_open_date: None,
            end_open_date: None,
            submit_date: None,
            start_assessment_date: None,
            end_assessment_date: None,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhase {
    pub id: StepPhaseId,
    pub project_id: ProjectId,
    pub status: StepPhaseStatus,
    pub start_open_date: u64,
    pub end_open_date: u64,
    pub submit_date: Option<u64>,
    pub start_assessment_date: u64,
    pub end_assessment_date: u64,
    pub assessment_method: AssessmentMethod,
}

#[derive(CandidType, Deserialize, PartialEq, Clone, Debug)]
pub enum AssessmentMethod {
    None,
    Vote,
    Grade,
}

impl fmt::Display for AssessmentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AssessmentMethod::None => write!(f, "None"),
            AssessmentMethod::Vote => write!(f, "Vote"),
            AssessmentMethod::Grade => write!(f, "Grade"),
        }
    }
}

#[derive(CandidType, Deserialize, PartialEq, Clone)]
pub enum StepPhaseStatus {
    Open,
    NotSubmitted,
    Submitted,
    Approved,
    NotApproved,
}

impl fmt::Display for StepPhaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StepPhaseStatus::Open => write!(f, "Open"),
            StepPhaseStatus::NotSubmitted => write!(f, "Not Submitted"),
            StepPhaseStatus::Submitted => write!(f, "Submitted"),
            StepPhaseStatus::Approved => write!(f, "Approved"),
            StepPhaseStatus::NotApproved => write!(f, "Not Approved"),
        }
    }
}

impl fmt::Debug for StepPhaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::NotSubmitted => write!(f, "Not Submitted"),
            Self::Submitted => write!(f, "Submitted"),
            Self::Approved => write!(f, "Approved"),
            Self::NotApproved => write!(f, "Not Approved"),
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StepPhaseVoteResultCreate {
    pub yes: u64,
    pub no: u64,
    pub total: u64,
    pub approved: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StepPhaseVoteResult {
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub yes: u64,
    pub no: u64,
    pub total: u64,
    pub approved: bool,
}

//Steps
#[derive(CandidType, Deserialize, Debug)]
pub struct StepCreate {
    pub questions_submission: Vec<QuestionSubmission>,
    pub upload_files: Vec<UploadFile>,
    pub checkbox_submission: Vec<CheckBoxSubmission>,
    pub numeric_submission: Vec<DecimalSubmission>,
}

#[derive(CandidType, Deserialize, Default, Debug)]
pub struct StepUpdate {
    pub questions_submission: Option<Vec<QuestionSubmission>>,
    pub checkbox_submission: Option<Vec<CheckBoxSubmission>>,
    pub numeric_submission: Option<Vec<DecimalSubmission>>,
    pub upload_files: Option<Vec<UploadFile>>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StepQuestionSubmission {
    pub questions_submission: Vec<QuestionSubmission>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Step {
    pub id: StepId,
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub question_submission: Vec<QuestionSubmission>,
    pub checkbox_submission: Vec<CheckBoxSubmission>,
    pub decimal_submission: Vec<DecimalSubmission>,
    pub upload_files: Vec<UploadFile>,
    pub grade_end_date: Option<u64>,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct QuestionSubmission {
    pub id: String,
    pub response: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CheckBoxSubmission {
    pub id: String,
    pub value: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DecimalSubmission {
    pub id: String,
    pub value: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepGrade {
    pub neuron_id: UserNeuronId,
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub step_id: StepId,
    pub grade: u32,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseGradeResultCreate {
    pub avg_result: f64,
    pub total_steps_grades_count: u64,
    pub steps_grade_results: Vec<StepGradeResult>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseGradeResult {
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub avg_result: f64,
    pub total_steps_grades_count: u64,
    pub steps_grade_results: Vec<StepGradeResult>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepGradeResult {
    pub step_id: StepId,
    pub grades_count: u64,
    pub grade_avg: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseProposal {
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub proposal_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UploadUrlRequest {
    pub filename: String,
    pub document_type: DocumentType,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UploadUrlResponse {
    pub project_id: ProjectId,
    pub step_phase_id: StepPhaseId,
    pub step_id: StepId,
    pub url: String,
    pub document_type: DocumentType,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UploadPreSignedUrlRequest {
    pub bucket: String,
    pub keys: Vec<String>,
    pub method: S3Method, // "PUT" or "GET"
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum S3Method {
    PUT,
    GET,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultipleUploadUrlResponse {
    pub urls: Vec<PresignedUrlResponse>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct PresignedUrlResponse {
    pub key: String,
    pub url: String,
}
#[derive(Serialize, Deserialize)]
pub struct Context {
    pub bucket_start_time_index: usize,
    pub closing_price_index: usize,
}
