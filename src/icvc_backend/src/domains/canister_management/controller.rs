use crate::{utils::authenticator::check_is_owner, APIError};

use super::{service, types::CanisterConfigUpdate, types_storage::CanisterConfig};

#[ic_cdk::query(name = "getCanisterConfig")]
pub fn get_canister_config() -> Result<CanisterConfig, APIError> {
    let config = service::get_canister_config();

    Ok(config)
}

#[ic_cdk::update(name = "updateCanisterConfig")]
pub fn update_canister_config(
    canister_config_update: CanisterConfigUpdate,
) -> Result<CanisterConfig, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;

    service::update_canister_config(canister_config_update)
}

#[ic_cdk::query(name = "removeCanisterOwner")]
pub fn remove_canister_owner() -> Result<bool, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner(caller_id)?;

    service::remove_canister_owner()
}
