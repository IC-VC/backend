use crate::{utils::authenticator::check_is_owner_or_governance_id, APIError, ICVCConfigUpdate};

use super::{
    service,
    types::{Category, CategoryCreate},
    types_storage::ICVCConfig,
};

#[ic_cdk::query(name = "getICVCConfig")]
pub fn get_icvc_config() -> Result<ICVCConfig, APIError> {
    let config = service::get_icvc_config();

    Ok(config)
}

#[ic_cdk::update(name = "updateICVCConfig")]
pub fn update_icvc_config(icvc_config_update: ICVCConfigUpdate) -> Result<ICVCConfig, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id)?;

    service::update_icvc_config(icvc_config_update)
}

#[ic_cdk::update(name = "createCategory")]
pub fn create_category(category_create: CategoryCreate) -> Result<Category, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id)?;
    service::create_category(category_create)
}

#[ic_cdk::query(name = "getCategoryById")]
pub fn get_category_by_id(category_id: u64) -> Result<Category, APIError> {
    service::get_category_by_id(category_id)
}

#[ic_cdk::query(name = "getAllCategories")]
pub fn get_all_categories() -> Result<Vec<Category>, APIError> {
    service::get_all_categories()
}

#[ic_cdk::update(name = "deleteCategory")]
pub fn delete_category(category_id: u64) -> Result<Category, APIError> {
    let caller_id = ic_cdk::caller();
    check_is_owner_or_governance_id(caller_id)?;
    service::desactivate_category_by_id(category_id)
}
