pub mod state;
pub mod instruction;
pub mod processor;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

use crate::{
    instruction::FreelanceInstruction,
    processor::process_initialize_task,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    msg!("Client Work Init ----- invoked");

    let instruction = FreelanceInstruction::unpack(instruction_data)?;

    match instruction {

        FreelanceInstruction::InitializeTask {
            title,
            description,
            amount,
        } => {
            return process_initialize_task(
                program_id,
                accounts,
                title,
                description,
                amount,
            );
        }

        _ => {
            msg!("Instruction not implemented yet");
            return Ok(());
        }
    }
}