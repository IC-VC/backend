use crate::{
    utils::authenticator::check_is_owner_or_governance_id, APIError, ListProposalsResponse, ProposalData,
    ProjectProposalPayload,
};

use super::service;

#[ic_cdk::query(name = "getSnsListProposals")]
async fn get_sns_list_proposals(limit: u32) -> Result<ListProposalsResponse, APIError> {
    service::get_sns_list_proposals(limit).await
}

#[ic_cdk::query(name = "getSnsProposalById")]
async fn get_sns_proposal_by_id(proposal_id: u64) -> Result<ProposalData, APIError> {
    service::get_sns_proposal_by_id(proposal_id).await
}

#[ic_cdk::update]
fn validate_project_vote_proposal(proposal_payload: ProjectProposalPayload) -> Result<String, String> {
    let caller_id = ic_cdk::caller();

    match check_is_owner_or_governance_id(caller_id) {
        Ok(_) => service::validate_project_vote_proposal(proposal_payload),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn execute_project_vote_proposal(proposal_payload: ProjectProposalPayload) {
    let caller_id = ic_cdk::caller();

    match check_is_owner_or_governance_id(caller_id) {
        Ok(_) => service::execute_project_vote_proposal(proposal_payload).await,
        Err(err) => {
            ic_cdk::println!("Unable to execute vote proposal: {}", err.to_string());
        }
    }
}
