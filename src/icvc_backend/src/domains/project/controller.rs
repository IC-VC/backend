//! This module defines the controller functions for projects queries and updates.

use crate::{
    utils::authenticator::check_is_project_owner_or_admin, APIError, ProjectAndStepPhase, UserId,
};

use super::{
    service,
    types::{Project, ProjectCreate, ProjectId, ProjectUpdate},
};

/// Creates a new project with the specified details.
///
/// # Arguments
/// * `project_create` - The details of the project to be created.
///
/// # Returns
/// * `Result<Project, APIError>` - The newly created project or an error.
#[ic_cdk::update(name = "createProject")]
pub fn create_project(project_create: ProjectCreate) -> Result<Project, APIError> {
    let caller_id = ic_cdk::caller();

    service::create_project(caller_id, project_create)
}

/// Updates an existing project with the specified details.
///
/// # Arguments
/// * `project_id` - The ID of the project to be updated.
/// * `project_update` - The updated details of the project.
///
/// # Returns
/// * `Result<Project, APIError>` - The updated project or an error.
#[ic_cdk::update(name = "updateProject")]
pub fn update_project(
    project_id: ProjectId,
    project_update: ProjectUpdate,
) -> Result<Project, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::update_project(caller_id, project_id, project_update)
}

/// Updates an existing project with the specified details.
///
/// # Arguments
/// * `project_id` - The ID of the project to be updated.
/// * `project_update` - The updated details of the project.
///
/// # Returns
/// * `Result<Project, APIError>` - The updated project or an error.
#[ic_cdk::update(name = "deleteProject")]
pub fn delete_project(user_id: UserId, project_id: ProjectId) -> Result<Project, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_project_owner_or_admin(caller_id, project_id)?;

    service::delete_project(user_id, project_id)
}

/// Retrieves a project by its ID.
///
/// # Arguments
/// * `project_id` - The ID of the project to be retrieved.
///
/// # Returns
/// * `Result<Project, APIError>` - The requested project or an error.
#[ic_cdk::query(name = "getProjectById")]
pub fn get_project_by_id(project_id: ProjectId) -> Result<Project, APIError> {
    service::get_project_by_id(project_id)
}

/// Retrieves all projects, optionally paginated by a starting ID and a limit.
///
/// # Arguments
/// * `start_at` - The ID to start retrieving projects from (optional).
/// * `limit` - The maximum number of projects to retrieve (optional).
///
/// # Returns
/// * `Result<Vec<Project>, APIError>` - A list of projects or an error.
#[ic_cdk::query(name = "getAllProjects")]
pub fn get_all_projects(
    start_at: Option<ProjectId>,
    limit: Option<usize>,
) -> Result<Vec<Project>, APIError> {
    service::get_all_projects(start_at, limit)
}

/// Retrieves all projects owned by the caller.
///
/// # Returns
/// * `Result<Vec<Project>, APIError>` - A list of projects owned by the caller or an error.
#[ic_cdk::query(name = "getUserProjects")]
pub fn get_user_projects() -> Result<Vec<Project>, APIError> {
    let caller_id = ic_cdk::caller();
    service::get_user_projects(caller_id)
}

#[ic_cdk::query(name = "getProjectAndStepPhase")]
pub fn get_project_and_step_phase(project_id: ProjectId) -> Result<ProjectAndStepPhase, APIError> {
    service::get_project_and_step_phase_by_id(project_id)
}
