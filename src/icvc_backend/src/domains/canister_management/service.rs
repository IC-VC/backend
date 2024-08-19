use candid::Principal;

use crate::{repository, APIError};

use super::{types::CanisterConfigUpdate, types_storage::CanisterConfig};

pub fn get_canister_config() -> CanisterConfig {
    repository::get_canister_config()
}

pub fn validate_update_canister_config(canister_config_update: CanisterConfigUpdate) -> Result<String, String>  {
    let current_canister_config = repository::get_canister_config();
    
    match repository::update_canister_config(canister_config_update.clone()){
        Ok(_) =>  {
            let _ = repository::set_canister_config(current_canister_config.clone());
            Ok(format!(
                "Update canister config is valid for votting with values: {:?}", canister_config_update
            ))
        },
        _ =>  Err(format!(
            "Unable to validate proposal: update canister config with values: {:?}", canister_config_update
        ))
    }
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

pub fn set_owner(owner: Principal) -> Result<bool, APIError> {
    match repository::set_owner(owner) {
        Ok(_) => Ok(true),
        Err(_) => Err(APIError::InternalServerError(
            "Unable to remove owner".to_string(),
        )),
    }
}
