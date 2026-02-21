use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]

pub struct Task {
    pub is_initialized: bool,
    pub client: Pubkey,
    pub worker: Pubkey,
    pub title: String,
    pub description: String,
    pub amount: u64,
    pub created_at: i64,
    pub status: TaskStatus,
    pub is_paid: bool,
}
impl Task {
    pub const MAX_TITLE_LEN: usize = 50;
    pub const MAX_DESC_LEN: usize = 300;

    pub const LEN: usize =
        1 +  
        32 + 
        32 +
        4 + Self::MAX_TITLE_LEN +
        4 + Self::MAX_DESC_LEN +
        8 +  
        8 + 
        1 +  
        1;  
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorkerStake {
    pub is_initialized: bool,
    pub worker: Pubkey,
    pub task: Pubkey, //for whom he staked
    pub stake_amount: u64,
    pub is_refunded: bool,
}

impl WorkerStake {
    pub const LEN: usize =
        1 + 
        32 + 
        32 + 
        8 +  
        1;  
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Created,
    Funded,
    InProgress,
    Completed,
    Cancelled,
}