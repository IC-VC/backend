pub mod domains;
pub mod repository;
pub mod utils;

use domains::canister_management::types::InitArgs;
use domains::canister_management::types_storage::CanisterConfig;
use domains::core;
use domains::icvc_configuration;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

use crate::domains::canister_management::types::*;
use crate::domains::core::types::*;
use crate::domains::icvc_configuration::types::*;
use crate::domains::icvc_configuration::types_storage::*;
use crate::domains::project::types::*;
use crate::domains::sns_integration::types::*;
use crate::domains::sns_integration::types_sns_governance::*;
use crate::domains::step::types::*;
use crate::domains::user::types::*;
use candid::Principal;

/// Initializes the canister with the caller as the owner and sets up the default configuration.
///
/// This function is called once when the canister is first deployed. It performs the following actions:
/// - Sets the caller as the owner of the canister.
/// - Configures the canister with the owner's principal ID.
/// - Initializes the default step phases configuration.
/// - Starts a timer to update projects every x seconds.
///
/// # Panics
///
/// This function will panic if it fails to set the canister configuration in the repository.
#[ic_cdk::init]
fn init(init_args: Option<InitArgs>) {
    let owner = ic_cdk::caller();

    let config = match init_args {
        None => CanisterConfig {
            owner: Some(owner),
            sns_governance_id: None,
            subaccount: None,
            max_stable_memory_size: Some(0),
        },
        Some(args) => CanisterConfig {
            owner: Some(owner),
            sns_governance_id: args.sns_governance_id,
            subaccount: args.subaccount,
            max_stable_memory_size: Some(args.max_stable_memory_size),
        },
    };

    repository::set_canister_config(config)
        .expect("Initialization failed: Unable to set canister config!");

    icvc_configuration::service::ini_default_categories();
    icvc_configuration::service::init_default_step_phases_config();

    let config = icvc_configuration::service::get_icvc_config();
    core::service::start_update_projects_timer(config.projects_update_timer_interval);

    ic_cdk::println!(
        "Canister init successful! Owner's Principal ID: {:?}",
        owner.to_text()
    );
}

/// Handles post-upgrade logic for the canister.
///
/// This function is called after the canister has been upgraded. It performs the following actions:
/// - Logs a message indicating that the timer will be restarted.
/// - Restarts the timer to update projects every x seconds.
///
/// This ensures that any necessary periodic tasks continue to run after the canister upgrade.
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Restart timer.");
    let config = icvc_configuration::service::get_icvc_config();

    core::service::start_update_projects_timer(config.projects_update_timer_interval);
}

ic_cdk::export_candid!();
