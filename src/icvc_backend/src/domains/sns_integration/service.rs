use candid::Encode;
use ic_cdk::api::call::RejectionCode;

use super::{types::ProposalPayload, types_sns_governance::ProposalId};
use crate::{
    domains::{
        self,
        canister_management::{self, types_storage::CanisterConfig},
        step,
    },
    repository, APIError, Action, Command, Command1, ExecuteGenericNervousSystemFunction,
    GetProposalArguments, ListProposals, ListProposalsResponse, ManageNeuron, ManageNeuronResponse,
    ProjectId, Proposal, ProposalData, StepPhaseId, StepPhaseVoteResultCreate,
};
use domains::core;

pub fn validate_project_vote_proposal(proposal_payload: ProposalPayload) -> Result<String, String> {
    let project_id = proposal_payload.project_id;
    let step_phase_id = proposal_payload.phase_id;

    match repository::retrieve_project_by_id(project_id) {
        Some(project) => {
            if project.current_phase == step_phase_id {
                Ok(format!(
                    "Project with id {} on phase: {} is valid for votting.",
                    project.id, project.current_phase
                ))
            } else {
                Err(format!(
                    "Unable to validate proposal, project id {} is not on phase id {}",
                    project.id, step_phase_id
                ))
            }
        }
        _ => Err(format!(
            "Unable to validate proposal, project id {} not found",
            project_id
        )),
    }
}

pub async fn execute_project_vote_proposal(proposal_payload: ProposalPayload) {
    let project_id = proposal_payload.project_id;
    let step_phase_id = proposal_payload.phase_id;

    let project = match repository::retrieve_project_by_id(project_id) {
        Some(project) => project,
        None => {
            ic_cdk::println!(
                "Unable to execute proposal, project id {} not found",
                project_id
            );
            return;
        }
    };

    let step_phase = match repository::get_step_phase_by_id(project_id, step_phase_id) {
        Some(step_phase) => step_phase,
        None => {
            ic_cdk::println!(
                "Unable to execute proposal, step phase id {} not found",
                project_id
            );
            return;
        }
    };

    let proposal = match step::service::get_proposal_by_phase_id(project_id, step_phase_id) {
        Ok(proposal) => proposal,
        Err(e) => {
            ic_cdk::println!("{:?}", e);
            return;
        }
    };

    let sns_proposal = match get_sns_proposal_by_id(proposal.proposal_id).await {
        Ok(proposal) => proposal,
        _ => {
            ic_cdk::println!("Unable to found sns proposal id {}", proposal.proposal_id);
            return;
        }
    };

    if let Some(tally) = &sns_proposal.latest_tally {
        // Assuming 50% + 1 votes is needed to pass
        let yes_votes = tally.yes;
        let total_votes = tally.total;
        let yes_percentage = yes_votes as f64 / total_votes as f64 * 100.0;

        // Check if yes votes exceed 50% of the total votes
        let approved = yes_percentage > 50.0;

        let step_phase_vote_result = StepPhaseVoteResultCreate {
            yes: yes_votes,
            no: tally.no,
            total: total_votes,
            approved,
        };
        step::service::put_step_phase_vote_result(
            project_id,
            step_phase_id,
            step_phase_vote_result,
        );
        core::service::update_phase_status(&project, step_phase, approved);

        ic_cdk::println!(
            "Execute proposal, project id {} on phase id {}",
            project.id,
            step_phase_id
        );
    } else {
        ic_cdk::println!(
            "Unable to Execute proposal, project id {} on phase id {}",
            project.id,
            step_phase_id
        );
    }
}

pub async fn get_sns_proposal_by_id_old(proposal_id: u64) -> Result<ProposalData, APIError> {
    let canister_config = canister_management::service::get_canister_config();
    let gov_canister_id = match canister_config.sns_governance_id {
        Some(id) => id,
        None => {
            return Err(APIError::InternalServerError(
                "Governance canister ID not set".to_string(),
            ))
        }
    };
    let method = "get_proposal";
    let arguments = GetProposalArguments {
        proposal_id: Some(ProposalId { id: proposal_id }),
    };

    ic_cdk::println!("Calling get_proposal with arguments: {:?}", arguments);

    let result: Result<(Option<ProposalData>,), (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::call(gov_canister_id, method, (arguments,)).await;

    match result {
        Ok((Some(proposal_data),)) => Ok(proposal_data),
        Ok((None,)) => Err(APIError::NotFound(format!(
            "Proposal with id {} not found",
            proposal_id
        ))),
        Err((code, msg)) => Err(APIError::InternalServerError(format!(
            "Error: {}: {}",
            code as i32, msg
        ))),
    }
}

pub async fn get_sns_proposal_by_id(proposal_id: u64) -> Result<ProposalData, APIError> {
    let canister_config = canister_management::service::get_canister_config();
    let gov_canister_id = match canister_config.sns_governance_id {
        Some(id) => id,
        None => {
            return Err(APIError::InternalServerError(
                "Governance canister ID not set".to_string(),
            ))
        }
    };
    //Note: This approach was taken due to some errors encountered when calling the SNS governance for a single proposal.
    let limit = 100;
    let method = "list_proposals";
    let arguments = ListProposals {
        include_reward_status: vec![],
        before_proposal: None,
        limit: limit,
        exclude_type: vec![],
        include_status: vec![],
    };

    let result: Result<(ListProposalsResponse,), (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::call(gov_canister_id, method, (arguments,)).await;

    ic_cdk::println!("get proposal: {:?}", result);
    match result {
        Ok((response,)) => {
            ic_cdk::println!("{:?}", response);
            for proposal in response.proposals {
                ic_cdk::println!("proposal: {:?}", proposal.id);
                if let Some(proposal_id_struct) = &proposal.id {
                    if proposal_id_struct.id == proposal_id {
                        return Ok(proposal);
                    }
                }
            }
            Err(APIError::NotFound(format!(
                "Proposal with id {} not found",
                proposal_id
            )))
        }
        Err((code, msg)) => {
            ic_cdk::println!("Error code: {:?}, message: {:?}", code, msg);
            Err(APIError::InternalServerError(format!(
                "Error: {}: {}",
                code as i32, msg
            )))
        }
    }
}

pub async fn get_sns_list_proposals(limit: u32) -> Result<ListProposalsResponse, APIError> {
    let canister_config = canister_management::service::get_canister_config();
    let gov_canister_id = match canister_config.sns_governance_id {
        Some(id) => id,
        None => {
            return Err(APIError::InternalServerError(
                "Governance canister ID not set".to_string(),
            ))
        }
    };
    let method = "list_proposals";
    let arguments = ListProposals {
        include_reward_status: vec![],
        before_proposal: None,
        limit: limit,
        exclude_type: vec![],
        include_status: vec![],
    };

    let result: Result<(ListProposalsResponse,), (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::call(gov_canister_id, method, (arguments,)).await;

    match result {
        Ok((response,)) => {
            ic_cdk::println!("{:?}", response);
            Ok(response)
        }
        Err((code, msg)) => {
            ic_cdk::println!("Error code: {:?}, message: {:?}", code, msg);
            Err(APIError::InternalServerError(format!("Error msg: {}", msg)))
        }
    }
}

pub async fn submit_project_vote_proposal(
    project_id: ProjectId,
    phase_id: StepPhaseId,
) -> Result<u64, APIError> {
    let project = match repository::retrieve_project_by_id(project_id) {
        Some(project) => project,
        None => return Err(APIError::NotFound("Project not found".to_string())),
    };

    let payload = ProposalPayload {
        project_id,
        phase_id,
    };

    let serialized_payload = match Encode!(&payload) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(APIError::InternalServerError(format!(
                "Failed to serialize payload: {}",
                e
            )))
        }
    };

    let proposal = Proposal {
        url: "https://icvc.com/".to_string(),
        title: "ICVC project votting".to_string(),
        action: Some(Action::ExecuteGenericNervousSystemFunction(
            ExecuteGenericNervousSystemFunction {
                function_id: 4001,
                payload: serialized_payload,
            },
        )),
        summary: format!("Votting for {}", project.title),
    };

    match make_sns_proposal(proposal).await {
        Ok(proposal) => {
            repository::put_step_phase_proposal(project_id, phase_id, proposal.id);
            Ok(proposal.id)
        }
        Err(e) => return Err(e),
    }
}

async fn make_sns_proposal(proposal: Proposal) -> Result<ProposalId, APIError> {
    let canister_config: CanisterConfig = canister_management::service::get_canister_config();

    let subaccount = match canister_config.subaccount {
        Some(ref subaccount) => subaccount.clone(),
        None => {
            return Err(APIError::BadRequest(
                "Subaccount is not set in the canister configuration.".to_string(),
            ))
        }
    };

    let sns_governance_id = match canister_config.sns_governance_id {
        Some(sns_gov_canister_id) => sns_gov_canister_id,
        None => {
            return Err(APIError::BadRequest(
                "SNS governance canister ID is not set in the canister configuration.".to_string(),
            ))
        }
    };

    let manage_neuron_command = ManageNeuron {
        subaccount,
        command: Some(Command::MakeProposal(proposal)),
    };

    let result: Result<(ManageNeuronResponse,), (RejectionCode, String)> =
        ic_cdk::call(sns_governance_id, "manage_neuron", (manage_neuron_command,)).await;

    match result {
        Ok((response,)) => {
            ic_cdk::println!("{:?}", response);
            match response.command {
                Some(command) => match command {
                    Command1::MakeProposal(get_proposal) => {
                        if let Some(proposal_id) = get_proposal.proposal_id {
                            Ok(proposal_id)
                        } else {
                            Err(APIError::InternalServerError(format!(
                                "Error making proposa, no proposal id found."
                            )))
                        }
                    }
                    _ => Err(APIError::InternalServerError(format!(
                        "Error making proposal command make proposal not found."
                    ))),
                },
                None => Err(APIError::InternalServerError(format!(
                    "Error making proposal, no command found"
                ))),
            }
        }
        Err((code, msg)) => {
            ic_cdk::println!("Error code: {:?}, message: {:?}", code, msg);
            Err(APIError::InternalServerError(format!(
                "Error making proposal: code: {:?}, message: {}",
                code, msg
            )))
        }
    }
}
