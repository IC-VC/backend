use crate::{repository, APIError};

use super::{types::CanisterConfigUpdate, types_storage::CanisterConfig};

pub fn get_canister_config() -> CanisterConfig {
    repository::get_canister_config()
}

pub fn update_canister_config(
    update_canister_config: CanisterConfigUpdate,
) -> Result<CanisterConfig, APIError> {
    match repository::update_canister_config(update_canister_config) {
        Ok(config) => Ok(config),
        Err(_) => Err(APIError::InternalServerError(
            "Unable to update config".to_string(),
        )),
    }
}

pub fn remove_canister_owner() -> Result<bool, APIError> {
    match repository::remove_canister_owner() {
        Ok(config) => Ok(config.owner == None),
        Err(_) => Err(APIError::InternalServerError(
            "Unable to remove owner".to_string(),
        )),
    }
}
