use core::task;

use solana_program::{
    account_info::{AccountInfo, next_account_info, next_account_infos}, 
    clock::Clock, 
    entrypoint::ProgramResult, 
     msg,
      program::invoke_signed, 
      program_error::ProgramError, 
      pubkey::Pubkey, system_instruction, 
      sysvar::{Sysvar, rent::Rent}
};

use borsh::{BorshDeserialize,BorshSerialize};

use crate::{instruction::FreelanceInstruction, state::{Task, TaskStatus}};


pub fn process_initialize_task(
  program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    description: String,
    amount: u64,


)->ProgramResult{
    let account_info_iter = &mut accounts.iter();

    let client = next_account_info(account_info_iter)?;
    let task_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;


    if !client.is_signer{
        msg!("Unauthorised! Client Signature missing");
        return  Err(ProgramError::MissingRequiredSignature)
        
}


  
   if title.len() > Task::MAX_TITLE_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    if description.len() > Task::MAX_DESC_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }



    let (expected_pda, bump)= Pubkey::find_program_address(&[b"task", client.key.as_ref()], program_id);



    if expected_pda != *task_pda.key {
        msg!("Mismatched Seeds || PDA address invalid");
        return Err(ProgramError::InvalidSeeds);
    }

    let space = Task::LEN;

    let rent_lamports = Rent::get()?.minimum_balance(space);

   invoke_signed(
        &system_instruction::create_account(
            client.key,
            task_pda.key,
            rent_lamports,
            space as u64,
            program_id,
        ),
        &[client.clone(), task_pda.clone(), system_program.clone()],
        &[&[b"task", client.key.as_ref(), &[bump]]],
    )?;

let clock = Clock::get()?;

   let task_data = Task {
        is_initialized: true,
        client: *client.key,
        worker: Pubkey::default(),
        title,
        description,
        amount,
        created_at: clock.unix_timestamp,
        status: TaskStatus::Created,
        is_paid: false,
    };


 task_data.serialize(&mut &mut task_pda.data.borrow_mut()[..])?;


    Ok(())
}