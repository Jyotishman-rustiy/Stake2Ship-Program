use solana_program::{
    account_info::{AccountInfo,next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    
};

use borsh::{BorshDeserialize,BorshSerialize};


pub fn process_initialize_task(
    program_id : &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]



)->ProgramResult{
    let account_info_iter = &mut accounts.iter();

    let client = next_account_info(account_info_iter)?;

    let task_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;






    Ok(())
}