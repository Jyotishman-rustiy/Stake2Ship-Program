use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]

pub enum FreelanceInstruction {
    InitializeTask {
        title: String,
        description : String,
        amount :u64,
    },
    FundTask,

    AcceptTask {
        stake_amount : u64
    },
    CompleteTask, 
    ReleasePayment,
    CancelTask
}

impl  FreelanceInstruction {
    pub fn unpack(input : &[u8])-> Result<Self, ProgramError>{
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}


