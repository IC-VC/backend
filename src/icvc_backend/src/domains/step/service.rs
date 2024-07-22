use ic_cdk::api::{
    self,
    management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
    },
};
use serde_json::json;
use std::collections::{HashSet, VecDeque};

use crate::{
    domains::{
        icvc_configuration::{
            self,
            types::{CheckBoxConfig, DecimalValueConfig, QuestionConfig, StepConfig},
        },
        sns_integration,
    },
    repository, APIError, AssessmentMethod, CheckBoxSubmission, Context, DecimalSubmission,
    DocumentType, MultipleUploadUrlResponse, ProjectId, ProposalData, QuestionSubmission, S3Method,
    Step, StepCreate, StepGrade, StepGradeResult, StepId, StepPhase, StepPhaseCreate,
    StepPhaseGradeResult, StepPhaseGradeResultCreate, StepPhaseId, StepPhaseProposal,
    StepPhaseStatus, StepPhaseUpdate, StepPhaseVoteResult, StepPhaseVoteResultCreate, StepUpdate,
    UploadFile, UploadPreSignedUrlRequest, UploadUrlRequest, UploadUrlResponse, UserId,
};

pub fn create_step_phase(
    user_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhase, APIError> {
    match initialize_step_phase(project_id, step_phase_id) {
        Ok(step_phase_create) => {
            match repository::insert_step_phase(project_id, step_phase_id, step_phase_create) {
                Some(step_phase) => {
                    repository::update_project_current_phase(user_id, project_id, step_phase_id);
                    Ok(step_phase)
                }
                None => Err(APIError::BadRequest(
                    "Failed to save the project step phase. Project Step Phase already exists."
                        .to_string(),
                )),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_step_phase_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhase, APIError> {
    match repository::get_step_phase_by_id(project_id, step_phase_id) {
        Some(step_phase) => Ok(step_phase),
        None => Err(APIError::NotFound(format!(
            "Step phase with id: {} for project id: {}, not found!",
            step_phase_id, project_id
        ))),
    }
}

pub fn update_step(
    caller_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    step_update: StepUpdate,
) -> Result<Step, APIError> {
    let step_phase =
        repository::get_step_phase_by_id(project_id, step_phase_id).ok_or_else(|| {
            APIError::NotFound(format!(
                "Step phase with id: {} for project id: {}, not found.",
                step_phase_id, project_id
            ))
        })?;

    repository::get_step_by_id(project_id, step_phase_id, step_id).ok_or_else(|| {
        APIError::NotFound(format!(
            "Step with id: {} for project id: {} and step phase id: {}, not found.",
            step_id, project_id, step_phase_id
        ))
    })?;

    let step_config =
        repository::get_step_config_by_id(step_phase_id, step_id).ok_or_else(|| {
            APIError::NotFound(format!(
                "Step config with id: {} for step phase id: {}, not found.",
                step_id, step_phase_id
            ))
        })?;

    if step_phase.status != StepPhaseStatus::Open {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {}, can't be edited since the step phase status is: {} and should be Open.",
            step_id, project_id, step_phase.status
        )));
    }

    if api::time() >= step_phase.end_open_date {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {}, can't be edited as the open period has passed.",
            step_id, project_id
        )));
    }

    if let Some(question_submissions) = &step_update.questions_submission {
        check_submission_lengths(
            step_config.questions.len(),
            question_submissions.len(),
            "question",
        )?;
        check_unique_question_ids(question_submissions)?;
        check_valid_question_ids(question_submissions, &step_config.questions)?;
        check_response_length(question_submissions, &step_config.questions)?;
    }

    if let Some(checkbox_submissions) = &step_update.checkbox_submission {
        check_submission_lengths(
            step_config.checkboxes.len(),
            checkbox_submissions.len(),
            "checkbox",
        )?;
        check_unique_checkbox_ids(checkbox_submissions)?;
        check_valid_checkbox_ids(checkbox_submissions, &step_config.checkboxes)?;
    }

    if let Some(decimal_submissions) = &step_update.numeric_submission {
        check_numeric_values(decimal_submissions)?;
        check_submission_lengths(
            step_config.decimal_values.len(),
            decimal_submissions.len(),
            "numeric",
        )?;
        check_unique_numeric_ids(decimal_submissions)?;
        check_valid_numeric_ids(decimal_submissions, &step_config.decimal_values)?;
    }

    repository::update_step(caller_id, project_id, step_phase_id, step_id, step_update).ok_or_else(
        || {
            APIError::NotFound(format!(
                "Unable to update step with id: {} for project id: {}.",
                step_id, project_id
            ))
        },
    )
}

pub async fn submit_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhase, APIError> {
    let current_time = ic_cdk::api::time();

    let step_phase = repository::get_step_phase_by_id(project_id, step_phase_id).ok_or(
        APIError::NotFound(format!(
            "Step phase with id: {} for project id: {} not found.",
            step_phase_id, project_id
        )),
    )?;

    if step_phase.status != StepPhaseStatus::Open {
        return Err(APIError::BadRequest(format!(
            "Step phase with id: {} for project id: {} can't be submited since the step phase status is: {} and should be open.",
            step_phase_id, project_id, step_phase.status
        )));
    }

    if current_time > step_phase.end_open_date {
        return Err(APIError::BadRequest(format!(
            "Step phase with id: {} for project id: {}, can't be submited as the open period has ended.",
            step_phase_id, project_id
        )));
    }

    if let Some(_) = repository::get_step_phase_config_by_id(step_phase_id) {
        let get_assessment_duration = icvc_configuration::service::get_assessment_duration();
        let end_assessment_date_ns = current_time + get_assessment_duration * 1_000_000_000;

        let mut step_phase_update = StepPhaseUpdate {
            status: Some(StepPhaseStatus::Submitted),
            end_open_date: Some(current_time),
            submit_date: Some(current_time),
            start_assessment_date: Some(current_time),
            end_assessment_date: Some(end_assessment_date_ns),
            ..Default::default()
        };

        if step_phase.assessment_method == AssessmentMethod::Vote {
            match sns_integration::service::submit_project_vote_proposal(project_id, step_phase_id)
                .await
            {
                Ok(proposal_id) => {
                    match sns_integration::service::get_sns_proposal_by_id(proposal_id).await {
                        Ok(proposal) => {
                            let voting_end_time = calculate_voting_end_time(&proposal);
                            step_phase_update.end_assessment_date = Some(voting_end_time);
                        }
                        Err(e) => {
                            ic_cdk::println!(
                                "Proposal was submitted but couldn't update the end time: {}",
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    return Err(APIError::InternalServerError(format!(
                        "Failed to submit project vote proposal: {}",
                        e
                    )));
                }
            }
        }

        repository::update_step_phase(project_id, step_phase_id, step_phase_update).ok_or_else(
            || {
                APIError::NotFound(format!(
                    "Unable to submit step phase with id: {} for project id: {}, step phase not found!",
                    step_phase_id, project_id
                ))
            },
        )
    } else {
        Err(APIError::NotFound(format!(
            "Unable to submit Step phase with id: {} for project id: {}, step phase config not found!",
            step_phase_id, project_id
        )))
    }
}

pub fn get_project_step_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Result<Step, APIError> {
    match repository::get_step_by_id(project_id, step_phase_id, step_id) {
        Some(step) => Ok(step),
        None => Err(APIError::NotFound(format!(
            "Step with id: {} for project id: {}, not found!",
            step_id, project_id
        ))),
    }
}

pub fn get_all_phases_by_project(project_id: ProjectId) -> Vec<StepPhase> {
    repository::get_all_phases_by_project(project_id)
}

pub fn get_all_project_steps(project_id: ProjectId, step_phase_id: StepPhaseId) -> Vec<Step> {
    repository::get_all_steps_by_phase(project_id, step_phase_id)
}

//Grades
pub fn submit_step_grade(
    caller_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    grade: u32,
) -> Result<u32, APIError> {
    repository::retrieve_project_by_user_id_and_project_id(caller_id, project_id).ok_or(
        APIError::BadRequest(
            "As the owner of the project, you are not allowed to vote on it.".to_string(),
        ),
    )?;

    let step_phase = repository::get_step_phase_by_id(project_id, step_phase_id).ok_or(
        APIError::NotFound(format!(
            "Step phase with id: {} for project id: {} not found.",
            step_phase_id, project_id
        )),
    )?;

    if step_phase.status != StepPhaseStatus::Submitted {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {} can't be updated since the step phase status is: {} and should be Submitted.",
            step_id, project_id, step_phase.status
        )));
    }

    if step_phase.assessment_method != AssessmentMethod::Grade {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {} can't be graded since the assessment method for this step is: {}",
            step_id, project_id, step_phase.assessment_method
        )));
    }

    repository::get_step_by_id(project_id, step_phase_id, step_id).ok_or(APIError::NotFound(
        format!(
            "Step  with id: {} for project id: {} not found.",
            step_id, project_id
        ),
    ))?;

    let current_time = api::time();
    let end_assessment_date = step_phase.end_assessment_date;

    if current_time > end_assessment_date {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {} can't be edited as the assessment period has ended.",
            step_id, project_id
        )));
    };

    repository::put_step_grade(caller_id, project_id, step_phase_id, step_id, grade).ok_or(
        APIError::InternalServerError(format!(
            "Unable to update step grade for step_id: {} in project_id: {}, it doesn't exist.",
            step_id, project_id
        )),
    )
}

pub fn get_step_grade_by_id(
    user_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Result<StepGrade, APIError> {
    match repository::get_step_grade_by_id(user_id, project_id, step_phase_id, step_id) {
        Some(step) => Ok(step),
        None => Err(APIError::NotFound(format!(
            "Step with id: {} for project id: {}, not found!",
            step_id, project_id
        ))),
    }
}

pub fn get_all_user_phase_steps_grade(
    user_id: UserId,
    project_id: u64,
    phase_id: u64,
) -> Result<Vec<StepGrade>, APIError> {
    let step_grades = repository::get_all_phase_steps_grade(user_id, project_id, phase_id);
    Ok(step_grades)
}

pub fn get_step_phase_grade_result(
    project_id: u64,
    step_phase_id: u64,
) -> Result<StepPhaseGradeResult, APIError> {
    let step_phase = repository::get_step_phase_by_id(project_id, step_phase_id).ok_or(
        APIError::NotFound(format!(
            "Step phase with id: {} for project id: {} not found.",
            step_phase_id, project_id
        )),
    )?;
    /* */
    if step_phase.assessment_method != AssessmentMethod::Grade {
        return Err(APIError::BadRequest(format!(
            "Step phase with id: {} for project id: {} can't be graded since the assessment method for this step is: {}",
            step_phase_id, project_id, step_phase.assessment_method
        )));
    }

    let current_time = api::time();
    let end_assessment_date = step_phase.end_assessment_date;

    if current_time <= end_assessment_date {
        return Err(APIError::BadRequest(format!(
            "Step phase with id: {} for project id: {} assessment period has not ended yet.",
            step_phase_id, project_id
        )));
    };

    if let Some(step_phase_result) =
        repository::get_grade_result_by_step_phase_id(project_id, step_phase_id)
    {
        Ok(step_phase_result)
    } else {
        Err(APIError::BadRequest(format!(
            "Step phase with id: {} for project id: {} not found.",
            step_phase_id, project_id
        )))
    }
}

pub fn save_and_calculate_grade_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> StepPhaseGradeResult {
    let result = calculate_step_phase_grade_average(project_id, step_phase_id);

    repository::put_step_phase_grade_result(project_id, step_phase_id, result)
}

pub fn calculate_step_phase_grade_average(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> StepPhaseGradeResultCreate {
    let steps = repository::get_all_steps_by_phase(project_id, step_phase_id);
    let mut steps_grade_result: Vec<StepGradeResult> = Vec::new();
    let mut user_grade_count: u64 = 0;
    let mut overall_average: f64 = 0.0;

    for step in steps {
        let step_grade_result = calculate_step_grades_average(project_id, step_phase_id, step.id);
        user_grade_count += step_grade_result.grades_count;

        steps_grade_result.push(step_grade_result);
    }

    if !steps_grade_result.is_empty() {
        let total_grade_sum: f64 = steps_grade_result.iter().map(|step| step.grade_avg).sum();
        overall_average = total_grade_sum / steps_grade_result.len() as f64;
    }

    ic_cdk::println!(
        "Overall average grade for step phase id {} in project {} is {:.2}",
        step_phase_id,
        project_id,
        overall_average
    );

    StepPhaseGradeResultCreate {
        avg_result: overall_average,
        total_steps_grades_count: user_grade_count,
        steps_grade_results: steps_grade_result,
    }
}

pub fn calculate_step_grades_average(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> StepGradeResult {
    let step_grades = repository::get_users_step_grades(project_id, step_phase_id, step_id);
    let mut avg = 0.0;

    if !step_grades.is_empty() {
        avg = step_grades.iter().sum::<u32>() as f64 / step_grades.len() as f64;
    }

    StepGradeResult {
        step_id,
        grades_count: step_grades.len() as u64,
        grade_avg: avg,
    }
}

//Vote
pub fn put_phase_proposal(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    proposal_id: u64,
) -> u64 {
    repository::put_step_phase_proposal(project_id, step_phase_id, proposal_id)
}

pub fn get_proposal_by_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseProposal, APIError> {
    match repository::get_proposal_by_step_phase_id(project_id, step_phase_id) {
        Some(proposal_id) => Ok(proposal_id),
        None => Err(APIError::NotFound(format!(
            "Proposal for project id {} on step phase id: {}, not found",
            project_id, step_phase_id
        ))),
    }
}

pub fn get_all_proposals_by_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Vec<StepPhaseProposal> {
    repository::get_all_proposals_by_step_phase(project_id, step_phase_id)
}

pub fn put_step_phase_vote_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    vote_result_create: StepPhaseVoteResultCreate,
) -> StepPhaseVoteResult {
    repository::put_step_phase_vote_result(project_id, step_phase_id, vote_result_create)
}

pub fn get_vote_result_by_step_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseVoteResult, APIError> {
    match repository::get_vote_result_by_step_phase_id(project_id, step_phase_id) {
        Some(proposal_id) => Ok(proposal_id),
        None => Err(APIError::NotFound(format!(
            "Proposal for project id {} on step phase id: {}, not found",
            step_phase_id, project_id
        ))),
    }
}

//Private methods
fn initialize_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseCreate, APIError> {
    if let Some(step_phase_config) = repository::get_step_phase_config_by_id(step_phase_id) {
        let step_phases_config = repository::get_all_phase_steps_config(step_phase_config.id);
        let mut errors: VecDeque<APIError> = VecDeque::new();
        let mut step_count = 0;

        for (index, step_phase_config) in step_phases_config.iter().enumerate() {
            let step_id = index as StepId;
            let step = initialize_step(step_phase_config);

            if repository::insert_step(project_id, step_phase_id, step_id, step).is_none() {
                errors.push_back(APIError::BadRequest(format!(
                    "Failed to save project step {}. Project Step already exists.",
                    step_id
                )));
            }
            step_count += 1;
        }

        if !errors.is_empty() {
            return Err(APIError::MultipleErrors(errors.into()));
        }

        let end_open_date =
            ic_cdk::api::time() + icvc_configuration::service::get_open_duration() * 1_000_000_000;
        let start_assessment_date = end_open_date;

        let end_assessment_date = start_assessment_date
            + step_count * (icvc_configuration::service::get_assessment_duration() * 1_000_000_000);

        let step_phase_create = StepPhaseCreate {
            status: StepPhaseStatus::Open,
            start_open_date: ic_cdk::api::time(),
            end_open_date: end_open_date,
            submit_date: None,
            start_assessment_date: start_assessment_date,
            end_assessment_date: end_assessment_date,
            assessment_method: step_phase_config.assessment_method,
        };

        Ok(step_phase_create)
    } else {
        Err(APIError::NotFound(format!(
            "Step phase config with id: {} for project id: {}, not found!",
            step_phase_id, project_id
        )))
    }
}

fn initialize_step(step_config: &StepConfig) -> StepCreate {
    let questions: Vec<QuestionSubmission>;
    let checkboxes: Vec<CheckBoxSubmission>;
    let numeric_values: Vec<DecimalSubmission>;
    let upload_files: Vec<UploadFile>;

    questions = step_config
        .questions
        .iter()
        .map(|question| QuestionSubmission {
            id: question.id.clone(),
            response: None,
        })
        .collect();

    checkboxes = step_config
        .checkboxes
        .iter()
        .map(|checkbox| CheckBoxSubmission {
            id: checkbox.id.clone(),
            value: false,
        })
        .collect();

    numeric_values = step_config
        .decimal_values
        .iter()
        .map(|decimal_value| DecimalSubmission {
            id: decimal_value.id.clone(),
            value: decimal_value.default_value,
        })
        .collect();

    upload_files = step_config
        .required_upload_files
        .iter()
        .map(|document_type| UploadFile {
            filename: None,
            document_type: document_type.clone(),
            s3_key: None,
        })
        .collect();

    StepCreate {
        questions_submission: questions,
        checkbox_submission: checkboxes,
        numeric_submission: numeric_values,
        upload_files,
    }
}

fn check_submission_lengths(
    config_len: usize,
    submission_len: usize,
    item_type: &str,
) -> Result<(), APIError> {
    if submission_len != config_len {
        return Err(APIError::BadRequest(format!(
            "Mismatch in {} submission lengths: expected {}, but received {}.",
            item_type, config_len, submission_len
        )));
    }

    Ok(())
}

fn check_response_length(
    question_submissions: &[QuestionSubmission],
    config_questions: &[QuestionConfig],
) -> Result<(), APIError> {
    for question in question_submissions {
        if let Some(config_question) = config_questions.iter().find(|q| q.id == question.id) {
            if let Some(response) = &question.response {
                if response.len() > config_question.max_num_bytes {
                    return Err(APIError::BadRequest(format!(
                        "Response too long for question ID: {}. Maximum allowed bytes: {}.",
                        question.id, config_question.max_num_bytes
                    )));
                }
            } else {
                return Err(APIError::BadRequest(format!(
                    "Missing response for question ID: {}.",
                    question.id
                )));
            }
        } else {
            return Err(APIError::BadRequest(format!(
                "Invalid question ID: {} in update.",
                question.id
            )));
        }
    }

    Ok(())
}

fn check_unique_question_ids(submissions: &[QuestionSubmission]) -> Result<(), APIError> {
    let mut seen_ids = HashSet::new();
    for item in submissions {
        let id = &item.id;
        if seen_ids.contains(id) {
            return Err(APIError::BadRequest(format!(
                "Duplicate question ID: {} in submissions.",
                id
            )));
        }
        seen_ids.insert(id.clone());
    }

    Ok(())
}

fn check_valid_question_ids(
    submissions: &[QuestionSubmission],
    config_items: &[QuestionConfig],
) -> Result<(), APIError> {
    for item in submissions {
        let id = &item.id;
        if !config_items.iter().any(|c| &c.id == id) {
            return Err(APIError::BadRequest(format!(
                "Invalid question ID: {} in update.",
                id
            )));
        }
    }

    Ok(())
}

fn check_unique_checkbox_ids(submissions: &[CheckBoxSubmission]) -> Result<(), APIError> {
    let mut seen_ids = HashSet::new();
    for item in submissions {
        let id = &item.id;
        if seen_ids.contains(id) {
            return Err(APIError::BadRequest(format!(
                "Duplicate checkbox ID: {} in submissions.",
                id
            )));
        }
        seen_ids.insert(id.clone());
    }

    Ok(())
}

fn check_valid_checkbox_ids(
    submissions: &[CheckBoxSubmission],
    config_items: &[CheckBoxConfig],
) -> Result<(), APIError> {
    for item in submissions {
        let id = &item.id;
        if !config_items.iter().any(|c| &c.id == id) {
            return Err(APIError::BadRequest(format!(
                "Invalid checkbox ID: {} in update.",
                id
            )));
        }
    }

    Ok(())
}

fn check_unique_numeric_ids(submissions: &[DecimalSubmission]) -> Result<(), APIError> {
    let mut seen_ids = HashSet::new();
    for item in submissions {
        let id = &item.id;
        if seen_ids.contains(id) {
            return Err(APIError::BadRequest(format!(
                "Duplicate numeric ID: {} in submissions.",
                id
            )));
        }
        seen_ids.insert(id.clone());
    }

    Ok(())
}

fn check_valid_numeric_ids(
    submissions: &[DecimalSubmission],
    config_items: &[DecimalValueConfig],
) -> Result<(), APIError> {
    for item in submissions {
        let id = &item.id;
        if !config_items.iter().any(|c| &c.id == id) {
            return Err(APIError::BadRequest(format!(
                "Invalid numeric ID: {} in update.",
                id
            )));
        }
    }

    Ok(())
}

fn check_numeric_values(submissions: &[DecimalSubmission]) -> Result<(), APIError> {
    for item in submissions {
        if item.value.is_nan() {
            return Err(APIError::BadRequest(format!(
                "Invalid numeric value (NaN) for ID: {}.",
                item.id
            )));
        }
    }
    Ok(())
}

pub async fn generate_upload_urls(
    caller_id: UserId,
    project_id: ProjectId,
    step_phase_id: u64,
    step_id: StepId,
    upload_req_list: Vec<UploadUrlRequest>,
) -> Result<Vec<UploadUrlResponse>, APIError> {
    let step_phase =
        repository::get_step_phase_by_id(project_id, step_phase_id).ok_or_else(|| {
            APIError::NotFound(format!(
                "Step phase with id: {} for project id: {}, not found.",
                step_phase_id, project_id
            ))
        })?;

    repository::get_step_by_id(project_id, step_phase_id, step_id).ok_or_else(|| {
        APIError::NotFound(format!(
            "Step with id: {} for project id: {} and step phase id: {}, not found.",
            step_id, project_id, step_phase_id
        ))
    })?;

    let step_config =
        repository::get_step_config_by_id(step_phase_id, step_id).ok_or_else(|| {
            APIError::NotFound(format!(
                "Step config with id: {} for step phase id: {}, not found.",
                step_id, step_phase_id
            ))
        })?;

    if step_phase.status != StepPhaseStatus::Open {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {}, can't be edited since the step phase status is: {} and should be Open.",
            step_id, project_id, step_phase.status
        )));
    }

    if api::time() >= step_phase.end_open_date {
        return Err(APIError::BadRequest(format!(
            "Step with id: {} for project id: {}, can't be edited as the open period has passed.",
            step_id, project_id
        )));
    }

    let required_document_types: Vec<DocumentType> = step_config.required_upload_files.clone();
    let requested_document_types: Vec<DocumentType> = upload_req_list
        .iter()
        .map(|req| req.document_type.clone())
        .collect();

    for req_doc_type in requested_document_types {
        if !required_document_types.contains(&req_doc_type) {
            return Err(APIError::BadRequest(format!(
                "The requested document type: {:?} is not required for step id: {} in step phase id: {}",
                req_doc_type, step_id, step_phase_id
            )));
        }
    }

    let bucket = "icvc-s3-uploads".to_string();
    let mut keys = Vec::new();
    let mut upload_files = Vec::new();
    let mut responses = Vec::new();

    for upload_req in upload_req_list {
        let key: String = format!(
            "projects/{}/{}/{}/{}",
            project_id,
            step_phase_id,
            step_id,
            upload_req.document_type.to_string()
        );
        keys.push(key.clone());

        let upload_file = UploadFile {
            filename: Some(upload_req.filename),
            document_type: upload_req.document_type,
            s3_key: Some(key.clone()),
        };
        upload_files.push(upload_file);
    }

    let upload_pre_signed_request = UploadPreSignedUrlRequest {
        bucket,
        keys,
        method: S3Method::PUT,
    };
    ic_cdk::println!("upload_pre_signed_request: {:?}", upload_pre_signed_request);
    let multiple_response: MultipleUploadUrlResponse =
        generate_presigned_urls(upload_pre_signed_request).await?;

    for url_response in multiple_response.urls {
        if let Some(upload_file) = upload_files
            .iter_mut()
            .find(|file| file.s3_key.as_ref() == Some(&url_response.key))
        {
            let response = UploadUrlResponse {
                project_id: project_id.clone(),
                step_phase_id: step_phase_id,
                step_id: step_id.clone(),
                url: url_response.url,
                document_type: upload_file.document_type.clone(),
            };

            responses.push(response);
        }
    }

    let step_update = StepUpdate {
        questions_submission: None,
        checkbox_submission: None,
        numeric_submission: None,
        upload_files: Some(upload_files.clone()),
    };

    repository::update_step(caller_id, project_id, step_phase_id, step_id, step_update);

    Ok(responses)
}

pub async fn generate_presigned_urls(
    upload_pre_signed_req: UploadPreSignedUrlRequest,
) -> Result<MultipleUploadUrlResponse, APIError> {
    let url =
        "https://ys2m3gt3ngexum4o4w66xcd5z40csxds.lambda-url.eu-central-1.on.aws/batch_generate"
            .to_string();

    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "ic-canister".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let request_body = json!(upload_pre_signed_req).to_string().into_bytes();

    let context = Context {
        bucket_start_time_index: 0,
        closing_price_index: 4,
    };

    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: request_headers,
        body: Some(request_body),
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&context).unwrap(),
        )),
    };

    match http_request(request, 1_703_154_400).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body).map_err(|e| {
                APIError::InternalServerError(format!("Response was not valid UTF-8: {}", e))
            })?;

            ic_cdk::println!("Reponse: {:?}", str_body.clone().to_string());

            if response.status == 403 as u64 {
                return Err(APIError::Forbidden(
                    "AccessDeniedException: Ensure your Lambda and API Gateway permissions are correctly configured".to_string(),
                ));
            }

            if response.status == 502 as u64 {
                return Err(APIError::InternalServerError(
                    "Bad Gateway: The server was acting as a gateway or proxy and received an invalid response from the upstream server.".to_string(),
                ));
            }

            let multiple_upload_url_response: MultipleUploadUrlResponse =
                serde_json::from_str(&str_body).map_err(|e| {
                    APIError::InternalServerError(format!("Failed to parse response: {}", e))
                })?;

            ic_cdk::println!(
                "multiple_upload_url_response: {:?}",
                multiple_upload_url_response.clone()
            );

            Ok(multiple_upload_url_response.clone())
        }
        Err((r, m)) => Err(APIError::InternalServerError(format!(
            "HTTP request failed: RejectionCode: {r:?}, Error: {m}"
        ))),
    }
}

#[ic_cdk::query()]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == 200 as u64 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error: {:?}", raw));
    }
    res
}

fn calculate_voting_end_time(proposal: &ProposalData) -> u64 {
    if let Some(wait_for_quiet_state) = &proposal.wait_for_quiet_state {
        wait_for_quiet_state.current_deadline_timestamp_seconds
    } else {
        proposal.proposal_creation_timestamp_seconds + proposal.initial_voting_period_seconds
    }
}
