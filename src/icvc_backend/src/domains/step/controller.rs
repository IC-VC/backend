//! This module defines the controller functions for project steps queries and updates.
use crate::{
    utils::authenticator::check_is_project_owner_or_admin, APIError, ProjectId, Step, StepGrade,
    StepId, StepPhase, StepPhaseGradeResult, StepPhaseId, StepPhaseProposal, StepPhaseVoteResult,
    StepUpdate, UploadUrlRequest, UploadUrlResponse,
};

use super::service;

/// Retrieves a specific step phase by its ID for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
///
/// # Returns
///
/// * `Result<StepPhase, APIError>` - The requested step phase or an error.
#[ic_cdk::query(name = "getStepPhaseById")]
pub fn get_step_phase_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhase, APIError> {
    service::get_step_phase_by_id(project_id, step_phase_id)
}

/// Retrieves all step phases associated with a specific project ID.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
///
/// # Returns
///
/// * `Result<Vec<StepPhase>, APIError>` - A list of step phases for the project or an error.
#[ic_cdk::query(name = "getAllStepPhaseByProjectId")]
pub fn get_step_phaget_all_phases_by_projectse_by_id(
    project_id: ProjectId,
) -> Result<Vec<StepPhase>, APIError> {
    let result = service::get_all_phases_by_project(project_id);

    Ok(result)
}

/// Submits a specific step phase for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
///
/// # Returns
///
/// * `Result<StepPhase, APIError>` - The submitted step phase or an error.
#[ic_cdk::update(name = "submitStepPhase")]
pub async fn submit_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhase, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::submit_step_phase(project_id, step_phase_id).await
}

/// Updates a specific step in a step phase for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
/// * `step_id` - The ID of the step to be updated.
/// * `step_update` - The updated step data.
///
/// # Returns
///
/// * `Result<Step, APIError>` - The updated step or an error.
#[ic_cdk::update(name = "updateStep")]
pub fn update_step(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    step_update: StepUpdate,
) -> Result<Step, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::update_step(caller_id, project_id, step_phase_id, step_id, step_update)
}

/// Retrieves a specific step by its ID for a given project and step phase.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
/// * `step_id` - The ID of the step.
///
/// # Returns
///
/// * `Result<Step, APIError>` - The requested step or an error.
#[ic_cdk::query(name = "getStepById")]
pub fn get_step_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Result<Step, APIError> {
    service::get_project_step_by_id(project_id, step_phase_id, step_id)
}

/// Retrieves all steps in a specific step phase for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
///
/// # Returns
///
/// * `Result<Vec<Step>, APIError>` - A list of steps for the step phase or an error.
#[ic_cdk::query(name = "getAllSteps")]
pub fn get_all_steps(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<Vec<Step>, APIError> {
    let steps = service::get_all_project_steps(project_id, step_phase_id);
    Ok(steps)
}

/// Submits a grade for a specific step in a step phase for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
/// * `step_id` - The ID of the step.
/// * `grade` - The grade to be submitted.
///
/// # Returns
///
/// * `Result<u32, APIError>` - The submitted grade or an error.
#[ic_cdk::update(name = "submitStepGrade")]
pub fn submit_step_grade(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    grade: u32,
) -> Result<u32, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::submit_step_grade(caller_id, project_id, step_phase_id, step_id, grade)
}

/// Retrieves the grade of a specific step by its ID for a given project and step phase.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
/// * `step_id` - The ID of the step.
///
/// # Returns
///
/// * `Result<StepGrade, APIError>` - The requested step grade or an error.
#[ic_cdk::query(name = "getStepGradepById")]
pub fn get_step_grade_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Result<StepGrade, APIError> {
    let caller_id = ic_cdk::caller();

    service::get_step_grade_by_id(caller_id, project_id, step_phase_id, step_id)
}

/// Retrieves all step grades for a specific step phase of a given project by the user.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
///
/// # Returns
///
/// * `Result<Vec<StepGrade>, APIError>` - A list of step grades for the step phase or an error.
#[ic_cdk::query(name = "getAllUserStepPhaseStepsGrade")]
pub fn get_all_user_steps_grade(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<Vec<StepGrade>, APIError> {
    let caller_id = ic_cdk::caller();

    service::get_all_user_phase_steps_grade(caller_id, project_id, step_phase_id)
}

/// Retrieves the assessment result of a specific step phase for a given project.
///
/// # Arguments
///
/// * `project_id` - The ID of the project.
/// * `step_phase_id` - The ID of the step phase.
///
/// # Returns
///
/// * `Result<StepPhaseGradeResult, APIError>` - The assessment result of the step phase or an error.
#[ic_cdk::query(name = "getStepPhaseAssessmentResult")]
pub fn get_step_phase_assessment_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseGradeResult, APIError> {
    service::get_step_phase_grade_result(project_id, step_phase_id)
}

#[ic_cdk::query(name = "getProposalByPhaseId")]
pub fn get_proposal_by_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseProposal, APIError> {
    service::get_proposal_by_phase_id(project_id, step_phase_id)
}

#[ic_cdk::query(name = "getAllProposalsByStepPhase")]
pub fn get_all_proposals_by_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<Vec<StepPhaseProposal>, APIError> {
    let proposals = service::get_all_proposals_by_step_phase(project_id, step_phase_id);
    Ok(proposals)
}

#[ic_cdk::query(name = "getVoteResultByStepPhaseId")]
pub fn get_vote_result_by_step_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Result<StepPhaseVoteResult, APIError> {
    service::get_vote_result_by_step_phase_id(project_id, step_phase_id)
}

#[ic_cdk::update(name = "generateUploadUrl")]
pub async fn generate_upload_url(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    upload_request: Vec<UploadUrlRequest>,
) -> Result<Vec<UploadUrlResponse>, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::generate_upload_urls(
        caller_id,
        project_id,
        step_phase_id,
        step_id,
        upload_request,
    )
    .await
}
