use crate::{utils::authenticator::check_is_owner, APIError, User, UserCreate, UserId, UserUpdate};

use super::service;

/// Adds a new admin user.
///
/// # Arguments
///
/// * `user_create` - The details of the user to be added as an admin.
///
/// # Returns
///
/// * `Result<User, APIError>` - The newly added admin user or an error.
#[ic_cdk::update(name = "addAdmin")]
pub fn add_admin(user_create: UserCreate) -> Result<User, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;

    service::add_admin(user_create)
}

/// Updates the details of an existing user.
///
/// # Arguments
///
/// * `user_id` - The ID of the user to be updated.
/// * `user_update` - The updated user details.
///
/// # Returns
///
/// * `Result<User, APIError>` - The updated user or an error.
#[ic_cdk::update(name = "updateUser")]
pub fn update_user(user_id: UserId, user_update: UserUpdate) -> Result<User, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;

    service::update_user(user_id, user_update)
}

/// Retrieves all admin users.
///
/// # Returns
///
/// * `Result<Vec<User>, APIError>` - A list of all admin users or an error.
#[ic_cdk::query(name = "getAllAdmins")]
pub fn get_all_admins() -> Result<Vec<User>, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;

    service::get_all_admins()
}

/// Deletes an existing user.
///
/// # Arguments
///
/// * `user_id` - The ID of the user to be deleted.
///
/// # Returns
///
/// * `Result<User, APIError>` - The deleted user or an error.
#[ic_cdk::update(name = "deleteUser")]
pub fn delete_user(user_id: UserId) -> Result<User, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;
    service::delete_user(user_id)
}
