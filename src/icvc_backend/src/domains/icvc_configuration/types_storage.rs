use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use crate::domains::step::types::AssessmentMethod;
use crate::DocumentType;

use super::types::{CheckBoxConfig, DecimalValueConfig, QuestionConfig};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ICVCConfig {
    pub open_duration: u64,
    pub assessment_duration: u64,
    pub grade_min_value: u32,
    pub grade_max_value: u32,
    pub projects_update_timer_interval: u64,
}

impl Storable for ICVCConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for ICVCConfig {
    fn default() -> Self {
        Self {
            open_duration: 14 * 24 * 60 * 60, // 14 days in seconds
            assessment_duration: 60,          // 2 * 24 * 60 * 60, // 2 days in seconds
            grade_min_value: 0,
            grade_max_value: 10,
            projects_update_timer_interval: 3600,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseConfigModel {
    pub assessement_method: AssessmentMethod,
}

impl Storable for StepPhaseConfigModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepConfigModel {
    pub questions: Vec<QuestionConfig>,
    pub required_upload_files: Vec<DocumentType>,
    pub checkboxes: Vec<CheckBoxConfig>,
    pub numeric_values: Vec<DecimalValueConfig>,
}

impl Storable for StepConfigModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CategoryModel {
    pub name: String,
    pub active: bool,
}

impl Storable for CategoryModel {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
