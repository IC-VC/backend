use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use crate::{StepGradeResult, UploadFile, UserId};

use super::types::{
    AssessmentMethod, CheckBoxSubmission, DecimalSubmission, QuestionSubmission, StepPhaseStatus,
};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepModel {
    pub questions_submission: Vec<QuestionSubmission>,
    pub checkbox_submission: Vec<CheckBoxSubmission>,
    pub numeric_submission: Vec<DecimalSubmission>,
    pub upload_files: Vec<UploadFile>,
    pub grade_end_date: Option<u64>,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
}

impl Storable for StepModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseModel {
    pub status: StepPhaseStatus,
    pub start_open_date: u64,
    pub end_open_date: u64,
    pub submit_date: Option<u64>,
    pub start_assessment_date: u64,
    pub end_assessment_date: u64,
    pub assessment_method: AssessmentMethod,
}

impl Storable for StepPhaseModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseGradeResultModel {
    pub avg_result: f64,
    pub total_steps_grades_count: u64,
    pub steps_grade_results: Vec<StepGradeResult>,
}

impl Storable for StepPhaseGradeResultModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseVoteResultModel {
    pub yes: u64,
    pub no: u64,
    pub total: u64,
    pub approved: bool,
}

impl Storable for StepPhaseVoteResultModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
