use candid::Principal;

use crate::{utils::authenticator::check_is_owner_or_governance_id, APIError};

use super::{service, types::CanisterConfigUpdate, types_storage::CanisterConfig};

#[ic_cdk::query(name = "getCanisterConfig")]
pub fn get_canister_config() -> Result<CanisterConfig, APIError> {
    let config = service::get_canister_config();

    Ok(config)
}

#[ic_cdk::update]
pub fn validate_update_canister_config(
    canister_config_update: CanisterConfigUpdate,
) -> Result<String, String>{
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id).expect("Not allowed.");

    ic_cdk::println!("{:?}", canister_config_update);

    service::validate_update_canister_config(canister_config_update)
}

#[ic_cdk::update]
pub fn update_canister_config(
    canister_config_update: CanisterConfigUpdate,
) -> Result<CanisterConfig, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id)?;

    service::update_canister_config(canister_config_update)
}

#[ic_cdk::update(name = "setOwner")]
pub fn set_owner(owner: Principal) -> Result<bool, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id)?;

    service::set_owner(owner)
}
