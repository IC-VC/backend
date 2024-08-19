use candid::Principal;

use crate::{domains::canister_management, repository, APIError, ProjectId};

pub fn check_is_owner_or_governance_id(caller_id: Principal) -> Result<(), APIError> {
    if _check_is_owner(caller_id) {
        Ok(())
    } else {
        Err(APIError::Forbidden(
            "Access denied: You are not the owner of the canister.".to_string(),
        ))
    }
}

#[allow(dead_code)]
pub fn check_is_admin(caller_id: Principal) -> Result<(), APIError> {
    if _check_is_owner(caller_id) {
        return Ok(());
    }

    match repository::get_user_by_id(caller_id) {
        Some(user) if user.is_admin => Ok(()),
        _ => Err(APIError::Forbidden(
            "Access denied: You do not have administrative privileges.".to_string(),
        )),
    }
}

pub fn check_is_project_owner_or_admin(
    caller_id: Principal,
    project_id: ProjectId,
) -> Result<(), APIError> {
    if _check_is_owner(caller_id) || _check_is_admin(caller_id) {
        return Ok(());
    }

    match repository::retrieve_project_by_user_id_and_project_id(caller_id, project_id) {
        Some(_) => Ok(()),
        _ => Err(APIError::Forbidden(
            "Access denied: You are not the project owner.".to_string(),
        )),
    }
}

fn _check_is_owner(caller_id: Principal) -> bool {
    let owner = repository::get_owner();
    let canister_config = canister_management::service::get_canister_config();
    let gov_canister_id = canister_config.sns_governance_id;

    if let Some(owner_id) = owner {
        if caller_id == owner_id {
            return true;
        }
    }

    if let Some(gov_id) = gov_canister_id {
        if caller_id == gov_id {
            return true;
        }
    }

    false
}

fn _check_is_admin(caller_id: Principal) -> bool {
    if let Some(user) = repository::get_user_by_id(caller_id) {
        user.is_admin
    } else {
        false
    }
}

pub fn check_is_sns_governance(caller_id: Principal) -> Result<(), APIError> {
    let config = canister_management::service::get_canister_config();

    match config.sns_governance_id {
        Some(sns_gov_id) if caller_id == sns_gov_id => Ok(()),
        _ => Err(APIError::Forbidden(
            "Access denied: You are not authorized.".to_string(),
        )),
    }
}
