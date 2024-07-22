use candid::{CandidType, Deserialize};

use crate::{AssessmentMethod, DocumentType, StepId, StepPhaseId};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ICVCConfigUpdate {
    pub open_duration: Option<u64>,
    pub assessment_duration: Option<u64>,
    pub grade_min_value: Option<u32>,
    pub grade_max_value: Option<u32>,
    pub projects_update_timer_interval: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseConfigCreate {
    pub assessment_method: AssessmentMethod,
    pub steps: Vec<StepConfigCreateDefault>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepPhaseConfig {
    pub id: StepPhaseId,
    pub assessment_method: AssessmentMethod,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepConfigCreateDefault {
    pub questions: Vec<QuestionConfigCreate>,
    pub checkboxes: Vec<CheckBoxConfigCreate>,
    pub decimal_values: Vec<DecimalValueConfigCreate>,
    pub required_upload_files: Vec<DocumentType>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepConfigCreate {
    pub questions: Vec<QuestionConfig>,
    pub checkboxes: Vec<CheckBoxConfig>,
    pub decimal_values: Vec<DecimalValueConfig>,
    pub required_upload_files: Vec<DocumentType>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct StepConfig {
    pub id: StepPhaseId,
    pub step_id: StepId,
    pub questions: Vec<QuestionConfig>,
    pub checkboxes: Vec<CheckBoxConfig>,
    pub decimal_values: Vec<DecimalValueConfig>,
    pub required_upload_files: Vec<DocumentType>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct QuestionConfigCreate {
    pub max_num_bytes: usize,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct QuestionConfig {
    pub id: String,
    pub max_num_bytes: usize,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CheckBoxConfigCreate {
    pub default_value: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CheckBoxConfig {
    pub id: String,
    pub default_value: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DecimalValueConfigCreate {
    pub default_value: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DecimalValueConfig {
    pub id: String,
    pub default_value: f64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CategoryCreate {
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub active: bool,
}
