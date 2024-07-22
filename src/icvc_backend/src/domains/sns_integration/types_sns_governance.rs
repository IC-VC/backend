use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Account {
    pub owner: Option<Principal>,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetProposalArguments {
    pub proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Action {
    ManageNervousSystemParameters(NervousSystemParameters),
    AddGenericNervousSystemFunction(NervousSystemFunction),
    ManageDappCanisterSettings(ManageDappCanisterSettings),
    RemoveGenericNervousSystemFunction(u64),
    UpgradeSnsToNextVersion {},
    RegisterDappCanisters(RegisterDappCanisters),
    TransferSnsTreasuryFunds(TransferSnsTreasuryFunds),
    UpgradeSnsControlledCanister(UpgradeSnsControlledCanister),
    DeregisterDappCanisters(DeregisterDappCanisters),
    MintSnsTokens(MintSnsTokens),
    Unspecified {},
    ManageSnsMetadata(ManageSnsMetadata),
    ExecuteGenericNervousSystemFunction(ExecuteGenericNervousSystemFunction),
    ManageLedgerParameters(ManageLedgerParameters),
    Motion(Motion),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ActionAuxiliary {
    TransferSnsTreasuryFunds(MintSnsTokensActionAuxiliary),
    MintSnsTokens(MintSnsTokensActionAuxiliary),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct AddNeuronPermissions {
    pub permissions_to_add: Option<NeuronPermissionList>,
    pub principal_id: Option<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Amount {
    pub e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Ballot {
    pub vote: i32,
    pub cast_timestamp_seconds: u64,
    pub voting_power: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum By {
    MemoAndController(MemoAndController),
    NeuronId {},
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct CanisterStatusResultV2 {
    pub status: CanisterStatusType,
    pub memory_size: u64,
    pub cycles: u64,
    pub settings: DefiniteCanisterSettingsArgs,
    pub idle_cycles_burned_per_day: u64,
    pub module_hash: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum CanisterStatusType {
    Stopped,
    Stopping,
    Running,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ChangeAutoStakeMaturity {
    pub requested_setting_for_auto_stake_maturity: bool,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimOrRefresh {
    pub by: Option<By>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimOrRefreshResponse {
    pub refreshed_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimSwapNeuronsRequest {
    pub neuron_parameters: Vec<NeuronParameters>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimSwapNeuronsResponse {
    pub claim_swap_neurons_result: Option<ClaimSwapNeuronsResult>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ClaimSwapNeuronsResult {
    Ok(ClaimedSwapNeurons),
    Err(i32),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ClaimedSwapNeurons {
    pub swap_neurons: Vec<SwapNeuron>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Command {
    Split(Split),
    Follow(Follow),
    DisburseMaturity(DisburseMaturity),
    ClaimOrRefresh(ClaimOrRefresh),
    Configure(Configure),
    RegisterVote(RegisterVote),
    MakeProposal(Proposal),
    StakeMaturity(StakeMaturity),
    RemoveNeuronPermissions(RemoveNeuronPermissions),
    AddNeuronPermissions(AddNeuronPermissions),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Command1 {
    Error(GovernanceError),
    Split(SplitResponse),
    Follow {},
    DisburseMaturity(DisburseMaturityResponse),
    ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure {},
    RegisterVote {},
    MakeProposal(GetProposal),
    RemoveNeuronPermission {},
    StakeMaturity(StakeMaturityResponse),
    MergeMaturity(MergeMaturityResponse),
    Disburse(DisburseResponse),
    AddNeuronPermission {},
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Command2 {
    Split(Split),
    Follow(Follow),
    DisburseMaturity(DisburseMaturity),
    Configure(Configure),
    RegisterVote(RegisterVote),
    SyncCommand {},
    MakeProposal(Proposal),
    FinalizeDisburseMaturity(FinalizeDisburseMaturity),
    ClaimOrRefreshNeuron(ClaimOrRefresh),
    RemoveNeuronPermissions(RemoveNeuronPermissions),
    AddNeuronPermissions(AddNeuronPermissions),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Configure {
    pub operation: Option<Operation>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Decimal {
    pub human_readable: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DefaultFollowees {
    pub followees: Vec<(u64, Followees)>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DefiniteCanisterSettingsArgs {
    pub freezing_threshold: u64,
    pub controllers: Vec<Principal>,
    pub memory_allocation: u64,
    pub compute_allocation: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DeregisterDappCanisters {
    pub canister_ids: Vec<Principal>,
    pub new_controllers: Vec<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Disburse {
    pub to_account: Option<Account>,
    pub amount: Option<Amount>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DisburseMaturity {
    pub to_account: Option<Account>,
    pub percentage_to_disburse: u32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DisburseMaturityInProgress {
    pub timestamp_of_disbursement_seconds: u64,
    pub amount_e8s: u64,
    pub account_to_disburse_to: Option<Account>,
    pub finalize_disbursement_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DisburseMaturityResponse {
    pub amount_disbursed_e8s: u64,
    pub amount_deducted_e8s: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DisburseResponse {
    pub transfer_block_height: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ExecuteGenericNervousSystemFunction {
    pub function_id: u64,
    pub payload: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FinalizeDisburseMaturity {
    pub amount_to_be_disbursed_e8s: u64,
    pub to_account: Option<Account>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Follow {
    pub function_id: u64,
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum FunctionType {
    NativeNervousSystemFunction {},
    GenericNervousSystemFunction(GenericNervousSystemFunction),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GenericNervousSystemFunction {
    pub validator_canister_id: Option<Principal>,
    pub target_canister_id: Option<Principal>,
    pub validator_method_name: Option<String>,
    pub target_method_name: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetMaturityModulationResponse {
    pub maturity_modulation: Option<MaturityModulation>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetMetadataResponse {
    pub url: Option<String>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetModeResponse {
    pub mode: Option<i32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetNeuron {
    pub neuron_id: Option<NeuronId>,
}

/*
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetNeuronResponse {
    pub result: Option<Result>,
}
*/

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetProposal {
    pub proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetProposalResponse {
    pub result: Option<Result1>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetRunningSnsVersionResponse {
    pub deployed_version: Option<Version>,
    pub pending_version: Option<UpgradeInProgress>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetSnsInitializationParametersResponse {
    pub sns_initialization_parameters: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Governance {
    pub root_canister_id: Option<Principal>,
    pub id_to_nervous_system_functions: Vec<(u64, NervousSystemFunction)>,
    pub metrics: Option<GovernanceCachedMetrics>,
    pub maturity_modulation: Option<MaturityModulation>,
    pub mode: i32,
    pub parameters: Option<NervousSystemParameters>,
    pub is_finalizing_disburse_maturity: Option<bool>,
    pub deployed_version: Option<Version>,
    pub sns_initialization_parameters: String,
    pub latest_reward_event: Option<RewardEvent>,
    pub pending_version: Option<UpgradeInProgress>,
    pub swap_canister_id: Option<Principal>,
    pub ledger_canister_id: Option<Principal>,
    pub proposals: Vec<(u64, ProposalData)>,
    pub in_flight_commands: Vec<(String, NeuronInFlightCommand)>,
    pub sns_metadata: Option<ManageSnsMetadata>,
    pub neurons: Vec<(String, Neuron)>,
    pub genesis_timestamp_seconds: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GovernanceCachedMetrics {
    pub not_dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    pub garbage_collectable_neurons_count: u64,
    pub neurons_with_invalid_stake_count: u64,
    pub not_dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    pub neurons_with_less_than_6_months_dissolve_delay_count: u64,
    pub dissolved_neurons_count: u64,
    pub total_staked_e8s: u64,
    pub total_supply_governance_tokens: u64,
    pub not_dissolving_neurons_count: u64,
    pub dissolved_neurons_e8s: u64,
    pub neurons_with_less_than_6_months_dissolve_delay_e8s: u64,
    pub dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    pub dissolving_neurons_count: u64,
    pub dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    pub timestamp_seconds: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GovernanceError {
    pub error_message: String,
    pub error_type: i32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct IncreaseDissolveDelay {
    pub additional_dissolve_delay_seconds: u32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ListNervousSystemFunctionsResponse {
    pub reserved_ids: Vec<u64>,
    pub functions: Vec<NervousSystemFunction>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ListNeurons {
    pub of_principal: Option<Principal>,
    pub limit: u32,
    pub start_page_at: Option<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ListNeuronsResponse {
    pub neurons: Vec<Neuron>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ListProposals {
    pub include_reward_status: Vec<i32>,
    pub before_proposal: Option<ProposalId>,
    pub limit: u32,
    pub exclude_type: Vec<u64>,
    pub include_status: Vec<i32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ListProposalsResponse {
    pub include_ballots_by_caller: Option<bool>,
    pub proposals: Vec<ProposalData>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageDappCanisterSettings {
    pub freezing_threshold: Option<u64>,
    pub canister_ids: Vec<Principal>,
    pub reserved_cycles_limit: Option<u64>,
    pub log_visibility: Option<i32>,
    pub memory_allocation: Option<u64>,
    pub compute_allocation: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageLedgerParameters {
    pub transfer_fee: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageNeuron {
    pub subaccount: Vec<u8>,
    pub command: Option<Command>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageNeuronResponse {
    pub command: Option<Command1>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ManageSnsMetadata {
    pub url: Option<String>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MaturityModulation {
    pub current_basis_points: Option<i32>,
    pub updated_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MemoAndController {
    pub controller: Option<Principal>,
    pub memo: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MergeMaturity {
    pub percentage_to_merge: u32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MergeMaturityResponse {
    pub merged_maturity_e8s: u64,
    pub new_stake_e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MintSnsTokens {
    pub to_principal: Option<Principal>,
    pub to_subaccount: Option<Subaccount>,
    pub memo: Option<u64>,
    pub amount_e8s: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MintSnsTokensActionAuxiliary {
    pub valuation: Option<Valuation>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Motion {
    pub motion_text: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NervousSystemFunction {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub function_type: Option<FunctionType>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NervousSystemParameters {
    pub default_followees: Option<DefaultFollowees>,
    pub max_dissolve_delay_seconds: Option<u64>,
    pub max_dissolve_delay_bonus_percentage: Option<u64>,
    pub max_followees_per_function: Option<u64>,
    pub neuron_claimer_permissions: Option<NeuronPermissionList>,
    pub neuron_minimum_stake_e8s: Option<u64>,
    pub max_neuron_age_for_age_bonus: Option<u64>,
    pub initial_voting_period_seconds: Option<u64>,
    pub neuron_minimum_dissolve_delay_to_vote_seconds: Option<u64>,
    pub reject_cost_e8s: Option<u64>,
    pub max_proposals_to_keep_per_action: Option<u32>,
    pub wait_for_quiet_deadline_increase_seconds: Option<u64>,
    pub max_number_of_neurons: Option<u64>,
    pub transaction_fee_e8s: Option<u64>,
    pub max_number_of_proposals_with_ballots: Option<u64>,
    pub max_age_bonus_percentage: Option<u64>,
    pub neuron_grantable_permissions: Option<NeuronPermissionList>,
    pub voting_rewards_parameters: Option<VotingRewardsParameters>,
    pub maturity_modulation_disabled: Option<bool>,
    pub max_number_of_principals_per_neuron: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub permissions: Vec<NeuronPermission>,
    pub maturity_e8s_equivalent: u64,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub source_nns_neuron_id: Option<u64>,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub dissolve_state: Option<DissolveState>,
    pub voting_power_percentage_multiplier: u64,
    pub vesting_period_seconds: Option<u64>,
    pub disburse_maturity_in_progress: Vec<DisburseMaturityInProgress>,
    pub followees: Vec<(u64, Followees)>,
    pub neuron_fees_e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronId {
    pub id: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronInFlightCommand {
    pub command: Option<Command2>,
    pub timestamp: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronParameters {
    pub controller: Option<Principal>,
    pub dissolve_delay_seconds: Option<u64>,
    pub source_nns_neuron_id: Option<u64>,
    pub stake_e8s: Option<u64>,
    pub followees: Vec<NeuronId>,
    pub hotkey: Option<Principal>,
    pub neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronPermission {
    pub principal: Option<Principal>,
    pub permission_type: Vec<i32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronPermissionList {
    pub permissions: Vec<i32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Operation {
    ChangeAutoStakeMaturity(ChangeAutoStakeMaturity),
    StopDissolving {},
    StartDissolving {},
    IncreaseDissolveDelay(IncreaseDissolveDelay),
    SetDissolveTimestamp(SetDissolveTimestamp),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Percentage {
    pub basis_points: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Proposal {
    pub url: String,
    pub title: String,
    pub action: Option<Action>,
    pub summary: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ProposalData {
    pub id: Option<ProposalId>,
    pub payload_text_rendering: Option<String>,
    pub action: u64,
    pub failure_reason: Option<GovernanceError>,
    pub action_auxiliary: Option<ActionAuxiliary>,
    pub ballots: HashMap<String, Ballot>,
    pub minimum_yes_proportion_of_total: Option<Percentage>,
    pub reward_event_round: u64,
    pub failed_timestamp_seconds: u64,
    pub reward_event_end_timestamp_seconds: Option<u64>,
    pub proposal_creation_timestamp_seconds: u64,
    pub initial_voting_period_seconds: u64,
    pub reject_cost_e8s: u64,
    pub latest_tally: Option<Tally>,
    pub wait_for_quiet_deadline_increase_seconds: u64,
    pub decided_timestamp_seconds: u64,
    pub proposal: Option<Proposal>,
    pub proposer: Option<NeuronId>,
    pub wait_for_quiet_state: Option<WaitForQuietState>,
    pub minimum_yes_proportion_of_exercised: Option<Percentage>,
    pub is_eligible_for_rewards: bool,
    pub executed_timestamp_seconds: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ProposalId {
    pub id: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RegisterDappCanisters {
    pub canister_ids: Vec<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RegisterVote {
    pub vote: i32,
    pub proposal: Option<ProposalId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RemoveNeuronPermissions {
    pub permissions_to_remove: Option<NeuronPermissionList>,
    pub principal_id: Option<Principal>,
}

/* TODO: Understand this error
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Result {
    Error(GovernanceError),
    Neuron(Neuron),
}
*/

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Result1 {
    Error(GovernanceError),
    Proposal(ProposalData),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RewardEvent {
    pub rounds_since_last_distribution: Option<u64>,
    pub actual_timestamp_seconds: u64,
    pub end_timestamp_seconds: Option<u64>,
    pub distributed_e8s_equivalent: u64,
    pub round: u64,
    pub settled_proposals: Vec<ProposalId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SetDissolveTimestamp {
    pub dissolve_timestamp_seconds: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SetMode {
    pub mode: i32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Split {
    pub memo: u64,
    pub amount_e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SplitResponse {
    pub created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct StakeMaturity {
    pub percentage_to_stake: Option<u32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct StakeMaturityResponse {
    pub maturity_e8s: u64,
    pub staked_maturity_e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Subaccount {
    pub subaccount: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SwapNeuron {
    pub id: Option<NeuronId>,
    pub status: i32,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Tally {
    pub no: u64,
    pub yes: u64,
    pub total: u64,
    pub timestamp_seconds: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub e8s: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct TransferSnsTreasuryFunds {
    pub from_treasury: i32,
    pub to_principal: Option<Principal>,
    pub to_subaccount: Option<Subaccount>,
    pub memo: Option<u64>,
    pub amount_e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeInProgress {
    pub mark_failed_at_seconds: u64,
    pub checking_upgrade_lock: u64,
    pub proposal_id: u64,
    pub target_version: Option<Version>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeSnsControlledCanister {
    pub new_canister_wasm: Vec<u8>,
    pub mode: Option<i32>,
    pub canister_id: Option<Principal>,
    pub canister_upgrade_arg: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Valuation {
    pub token: Option<i32>,
    pub account: Option<Account>,
    pub valuation_factors: Option<ValuationFactors>,
    pub timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ValuationFactors {
    pub xdrs_per_icp: Option<Decimal>,
    pub icps_per_token: Option<Decimal>,
    pub tokens: Option<Tokens>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Version {
    pub archive_wasm_hash: Vec<u8>,
    pub root_wasm_hash: Vec<u8>,
    pub swap_wasm_hash: Vec<u8>,
    pub ledger_wasm_hash: Vec<u8>,
    pub governance_wasm_hash: Vec<u8>,
    pub index_wasm_hash: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct VotingRewardsParameters {
    pub final_reward_rate_basis_points: Option<u64>,
    pub initial_reward_rate_basis_points: Option<u64>,
    pub reward_rate_transition_duration_seconds: Option<u64>,
    pub round_duration_seconds: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct WaitForQuietState {
    pub current_deadline_timestamp_seconds: u64,
}
