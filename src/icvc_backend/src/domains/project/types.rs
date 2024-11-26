use std::fmt;

use candid::{CandidType, Deserialize, Nat};

use crate::{domains::step::types::StepPhaseId, Account, StepPhase, UserId};

pub type ProjectId = u64;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectCreate {
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub categories: Vec<u64>,
    pub transaction_id: u64,
}

type TxId = Nat;
pub type BlockIndex = candid::Nat;

#[derive(CandidType, Deserialize, Debug)]
pub struct GetAccountTransactionsArgs {
    pub account: Account,
    pub start :  Option<TxId>,
    pub max_results: Nat
}

pub type Tokens = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct Burn {
  pub from: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
  pub to: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub approve: Option<Approve>,
  pub timestamp: u64,
  pub transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionWithId {
  pub id: BlockIndex,
  pub transaction: Transaction,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactions {
  pub balance: Tokens,
  pub transactions: Vec<TransactionWithId>,
  pub oldest_tx_id: Option<BlockIndex>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsErr { pub message: String }

#[derive(CandidType, Deserialize)]
pub enum GetTransactionsResult { Ok(GetTransactions), Err(GetTransactionsErr) }

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectUpdate {
    pub title: Option<String>,
    pub moto: Option<String>,
    pub description: Option<String>,
    pub team_members: Option<Vec<TeamMember>>,
    pub links: Option<Vec<Link>>,
    pub categories: Vec<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: ProjectId,
    pub user_id: UserId,
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub categories: Vec<u64>,
    pub current_phase: StepPhaseId,
    pub status: ProjectStatus,
    pub created_at: u64,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProjectAndStepPhase {
    pub id: ProjectId,
    pub user_id: UserId,
    pub title: String,
    pub moto: String,
    pub description: String,
    pub team_members: Vec<TeamMember>,
    pub links: Vec<Link>,
    pub current_phase: StepPhaseId,
    pub status: ProjectStatus,
    pub created_at: u64,
    pub update_by: Option<UserId>,
    pub update_at: Option<u64>,
    pub step_phase: StepPhase,
}

#[derive(CandidType, Deserialize, PartialEq, Clone, Debug)]
pub enum ProjectStatus {
    Open,
    Funded,
    NotFunded,
    NotSubmitted,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProjectStatus::Open => write!(f, "Open"),
            ProjectStatus::NotFunded => write!(f, "Not Funded"),
            ProjectStatus::Funded => write!(f, "Funded"),
            ProjectStatus::NotSubmitted => write!(f, "Not Submitted"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Link {
    pub kind: String,
    pub url: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TeamMember {
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub previous_experience: String,
    pub links: Vec<Link>,
    pub profile_picture: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UploadFile {
    pub filename: Option<String>,
    pub document_type: DocumentType,
    pub s3_key: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum DocumentType {
    PitchDeck,
    Logo,
    CoverPhoto,
    FinancialModels,
    ProductDemo,
    ExpenditurePlan,
}

impl ToString for DocumentType {
    fn to_string(&self) -> String {
        match self {
            DocumentType::PitchDeck => "PitchDeck".to_string(),
            DocumentType::Logo => "Logo".to_string(),
            DocumentType::CoverPhoto => "CoverPhoto".to_string(),
            DocumentType::FinancialModels => "FinancialModels".to_string(),
            DocumentType::ProductDemo => "ProductDemo".to_string(),
            DocumentType::ExpenditurePlan => "ExpenditurePlan".to_string(),
        }
    }
}
