use candid::Principal;

use crate::{repository, APIError, User, UserCreate, UserId, UserNeuron, UserNeuronId, UserUpdate};

pub fn add_admin(user_create: UserCreate) -> Result<User, APIError> {
    match repository::save_admin(user_create.clone()) {
        Some(user) => Ok(user),
        None => Err(APIError::BadRequest(format!(
            "Failed to add admin. Admin {} already exists.",
            user_create.user_id
        ))),
    }
}

pub fn update_user(user_id: UserId, update_user: UserUpdate) -> Result<User, APIError> {
    match repository::update_user(user_id, update_user) {
        Some(user) => Ok(user),
        None => Err(APIError::NotFound(format!(
            "User with id {} not found.",
            user_id
        ))),
    }
}

pub fn delete_user(user_id: UserId) -> Result<User, APIError> {
    match repository::delete_user(user_id) {
        Some(user) => Ok(user),
        None => Err(APIError::NotFound(format!(
            "User with id {} not found.",
            user_id
        ))),
    }
}

pub fn get_all_admins() -> Result<Vec<User>, APIError> {
    let users = repository::get_all_admin_users();

    Ok(users)
}

pub fn add_user_neuron(neuron_id: UserNeuronId, caller: Principal) -> Result<UserNeuron, APIError> {
    match repository::add_user_neuron(neuron_id, caller) {
        Some(neuron) => Ok(neuron),
        None => Err(APIError::Unauthorized(
            "Unable to create neuron".to_string(),
        )),
    }
}
