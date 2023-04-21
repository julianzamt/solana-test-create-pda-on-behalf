use crate::{errors::*, instructions::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub struct Processor;

impl Processor {
    pub fn process(
        accounts: &[AccountInfo],
        instruction_data: &[u8],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let instruction = CreatorInstruction::unpack(instruction_data)?;

        match instruction {
            CreatorInstruction::CreatePDAInvokeSigned {} => {
                Self::process_create_pda_invoke_signed(accounts, program_id)?;
            }
            CreatorInstruction::CreatePDAInvoke {} => {
                Self::process_create_pda_invoke(accounts)?;
            }
        }

        Ok(())
    }

    pub fn process_create_pda_invoke_signed(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("process_create_pda_invoke_signed ix...");
        let account_info_iter = &mut accounts.iter();
        let pda = next_account_info(account_info_iter)?;
        let intended_owner = next_account_info(account_info_iter)?;
        let other_pda_signer = next_account_info(account_info_iter)?;
        let signer_account_info = next_account_info(account_info_iter)?;
        let _system_program_account_info = next_account_info(account_info_iter)?;

        let rent = Rent::get()?;

        let (pda_address, _) = Pubkey::find_program_address(&[b"pda"], intended_owner.key);

        if pda_address != *pda.key {
            return Err(CreatorError::NotExpectedAddress.into());
        }

        let (other_pda_signer_address, bump) = Pubkey::find_program_address(&[b"signer"], program_id);

        if other_pda_signer_address != *other_pda_signer.key {
            return Err(CreatorError::NotExpectedAddress.into());
        }

        let space = 4;
        let rent_minimum_balance = rent.minimum_balance(space);

        invoke_signed(
            &create_account(
                &signer_account_info.key,
                &pda.key,
                rent_minimum_balance,
                space as u64,
                intended_owner.key,
            ),
            &[signer_account_info.clone(), pda.clone()],
            &[&[b"signer", &[bump]]],
        )?;

        Ok(())
    }

    pub fn process_create_pda_invoke(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_create_pda_invoke ix...");
        let account_info_iter = &mut accounts.iter();
        let pda = next_account_info(account_info_iter)?;
        let intended_owner = next_account_info(account_info_iter)?;
        let signer_account_info = next_account_info(account_info_iter)?;
        let _system_program_account_info = next_account_info(account_info_iter)?;

        let (pda_address, _) = Pubkey::find_program_address(&[b"pda"], intended_owner.key);
        if pda_address != *pda.key {
            return Err(CreatorError::NotExpectedAddress.into());
        }

        let rent = Rent::get()?;

        let space = 4;
        let rent_minimum_balance = rent.minimum_balance(space);

        invoke(
            &create_account(
                &signer_account_info.key,
                &pda.key,
                rent_minimum_balance,
                space as u64,
                intended_owner.key,
            ),
            &[signer_account_info.clone()],
        )?;

        Ok(())
    }
}
