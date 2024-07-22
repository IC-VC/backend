use crate::{
    domains::step,
    repository::{self, generate_project_id},
    APIError, ProjectAndStepPhase, UserId,
};

use super::types::{Project, ProjectCreate, ProjectId, ProjectUpdate};

pub fn create_project(user_id: UserId, project_create: ProjectCreate) -> Result<Project, APIError> {
    let project_id: ProjectId = generate_project_id();

    match repository::insert_project(user_id, project_id, project_create) {
        Some(project) => {
            match step::service::create_step_phase(user_id, project_id, project.current_phase) {
                Ok(_) => Ok(project),
                Err(e) => {
                    //If the initialization of the step fails we remove the project inserted.
                    repository::delete_project(user_id, project_id);

                    Err(e)
                }
            }
        }
        None => Err(APIError::BadRequest(
            "Failed to save the project. Project ID already exists.".to_string(),
        )),
    }
}

pub fn update_project(
    caller_id: UserId,
    project_id: ProjectId,
    update_project: ProjectUpdate,
) -> Result<Project, APIError> {
    let user_id = match repository::retrieve_project_by_id(project_id) {
        Some(project) => project.user_id,
        None => Err(APIError::NotFound(format!(
            "Project with id {} not found",
            project_id
        )))?,
    };

    // After removing the admin permission, we use the caller id instead of the user id retrieved.
    match repository::update_project(caller_id, user_id, project_id, update_project) {
        Some(project) => Ok(project),
        None => Err(APIError::NotFound(format!(
            "Project with id {} not found.",
            project_id
        ))),
    }
}

pub fn delete_project(user_id: UserId, project_id: ProjectId) -> Result<Project, APIError> {
    match repository::delete_project(user_id, project_id) {
        Some(project) => Ok(project),
        None => Err(APIError::NotFound(format!(
            "Project with id {} not found.",
            project_id
        ))),
    }
}

pub fn get_project_by_id(project_id: ProjectId) -> Result<Project, APIError> {
    match repository::retrieve_project_by_id(project_id) {
        Some(project) => Ok(project),
        None => Err(APIError::NotFound(format!(
            "Project with id {} not found.",
            project_id
        ))),
    }
}

pub fn get_project_and_step_phase_by_id(
    project_id: ProjectId,
) -> Result<ProjectAndStepPhase, APIError> {
    let project = repository::retrieve_project_by_id(project_id)
        .ok_or_else(|| APIError::NotFound(format!("Project with id {} not found.", project_id)))?;

    let step_phase = repository::get_step_phase_by_id(project_id, project.current_phase)
        .ok_or_else(|| {
            APIError::NotFound(format!(
                "Step phase for project id {} not found.",
                project_id
            ))
        })?;

    let response = ProjectAndStepPhase {
        id: project.id,
        user_id: project.user_id,
        title: project.title,
        moto: project.moto,
        description: project.description,
        team_members: project.team_members,
        links: project.links,
        current_phase: project.current_phase,
        status: project.status,
        created_at: project.created_at,
        update_by: project.update_by,
        update_at: project.update_at,
        step_phase: step_phase,
    };

    Ok(response)
}

pub fn get_all_projects(
    start_at: Option<ProjectId>,
    limit: Option<usize>,
) -> Result<Vec<Project>, APIError> {
    let projects = repository::retrieve_all_projects(start_at, limit);

    Ok(projects)
}

pub fn get_user_projects(user_id: UserId) -> Result<Vec<Project>, APIError> {
    let projects = repository::retrieve_user_projects(user_id);

    Ok(projects)
}
